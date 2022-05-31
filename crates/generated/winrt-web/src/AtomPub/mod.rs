#[repr(transparent)]
pub struct AtomPubClient(::windows_core::IUnknown);
impl AtomPubClient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AtomPubClient, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-web")]
    pub fn RetrieveServiceDocumentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveServiceDocumentAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub fn RetrieveMediaResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn RetrieveResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn CreateResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, description: Param1, item: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), description.into_param().abi(), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub fn CreateMediaResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, uri: Param0, mediatype: Param1, description: Param2, mediastream: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), mediatype.into_param().abi(), description.into_param().abi(), mediastream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub fn UpdateMediaResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, uri: Param0, mediatype: Param1, mediastream: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateMediaResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), mediatype.into_param().abi(), mediastream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn UpdateResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, uri: Param0, item: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn UpdateResourceItemAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateResourceItemAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn DeleteResourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteResourceAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn DeleteResourceItemAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Syndication::SyndicationItem>>(&self, item: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteResourceItemAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>(result__)
        }
    }
    pub fn CancelAsyncOperations(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CancelAsyncOperations)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-security")]
    pub fn CreateAtomPubClientWithCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(servercredential: Param0) -> ::windows_core::Result<AtomPubClient> {
        Self::IAtomPubClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAtomPubClientWithCredentials)(::windows_core::Interface::as_raw(this), servercredential.into_param().abi(), result__.as_mut_ptr()).from_abi::<AtomPubClient>(result__)
        })
    }
    #[cfg(all(feature = "winrt-security", feature = "winrt-web"))]
    pub fn ServerCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(all(feature = "winrt-security", feature = "winrt-web"))]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-security", feature = "winrt-web"))]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(all(feature = "winrt-security", feature = "winrt-web"))]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn RetrieveFeedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationClient>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationFeed, super::Syndication::RetrievalProgress>>(result__)
        }
    }
    pub fn IAtomPubClientFactory<R, F: FnOnce(&IAtomPubClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AtomPubClient, IAtomPubClientFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AtomPubClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AtomPubClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AtomPubClient {}
impl ::core::fmt::Debug for AtomPubClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AtomPubClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AtomPubClient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.AtomPubClient;{35392c38-cded-4d4c-9637-05f15c1c9406})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
    const IID: ::windows_core::GUID = <IAtomPubClient as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.AtomPubClient";
}
impl ::core::convert::From<AtomPubClient> for ::windows_core::IUnknown {
    fn from(value: AtomPubClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows_core::IUnknown {
    fn from(value: &AtomPubClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AtomPubClient> for ::windows_core::IInspectable {
    fn from(value: AtomPubClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AtomPubClient> for ::windows_core::IInspectable {
    fn from(value: &AtomPubClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows_core::Error;
    fn try_from(value: AtomPubClient) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<&AtomPubClient> for super::Syndication::ISyndicationClient {
    type Error = ::windows_core::Error;
    fn try_from(value: &AtomPubClient) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationClient> for AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationClient> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationClient> for &AtomPubClient {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationClient> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationClient>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AtomPubClient {}
unsafe impl ::core::marker::Sync for AtomPubClient {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClient(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAtomPubClient {
    type Vtable = IAtomPubClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35392c38_cded_4d4c_9637_05f15c1c9406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClient_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-web")]
    pub RetrieveServiceDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    RetrieveServiceDocumentAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub RetrieveMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-web")))]
    RetrieveMediaResourceAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub RetrieveResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    RetrieveResourceAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub CreateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, description: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    CreateResourceAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub CreateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mediastream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-web")))]
    CreateMediaResourceAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-web"))]
    pub UpdateMediaResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mediastream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-web")))]
    UpdateMediaResourceAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub UpdateResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    UpdateResourceAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub UpdateResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    UpdateResourceItemAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub DeleteResourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    DeleteResourceAsync: usize,
    #[cfg(feature = "winrt-web")]
    pub DeleteResourceItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    DeleteResourceItemAsync: usize,
    pub CancelAsyncOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAtomPubClientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAtomPubClientFactory {
    type Vtable = IAtomPubClientFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49d55012_57cb_4bde_ab9f_2610b172777b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAtomPubClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub CreateAtomPubClientWithCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    CreateAtomPubClientWithCredentials: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f5fd609_bc88_41d4_88fa_3de6704d428e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-web")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    Title: usize,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-web")))]
    Categories: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Accepts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Accepts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b7ec771_2ab3_4dbe_8bcc_778f92b75e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceDocument_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Workspaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Workspaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkspace(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb41da63b_a4b8_4036_89c5_83c31266ba49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-web")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    Title: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Collections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Collections: usize,
}
#[repr(transparent)]
pub struct ResourceCollection(::windows_core::IUnknown);
impl ResourceCollection {
    #[cfg(feature = "winrt-web")]
    pub fn Title(&self) -> ::windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn Categories(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Accepts(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Accepts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-data", feature = "winrt-web"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCollection {}
impl ::core::fmt::Debug for ResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ResourceCollection;{7f5fd609-bc88-41d4-88fa-3de6704d428e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceCollection {
    type Vtable = IResourceCollection_Vtbl;
    const IID: ::windows_core::GUID = <IResourceCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.ResourceCollection";
}
impl ::core::convert::From<ResourceCollection> for ::windows_core::IUnknown {
    fn from(value: ResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows_core::IUnknown {
    fn from(value: &ResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceCollection> for ::windows_core::IInspectable {
    fn from(value: ResourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCollection> for ::windows_core::IInspectable {
    fn from(value: &ResourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: ResourceCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<&ResourceCollection> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &ResourceCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for &ResourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceCollection {}
unsafe impl ::core::marker::Sync for ResourceCollection {}
#[repr(transparent)]
pub struct ServiceDocument(::windows_core::IUnknown);
impl ServiceDocument {
    #[cfg(feature = "winrt-foundation")]
    pub fn Workspaces(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Workspace>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Workspaces)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Workspace>>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-data", feature = "winrt-web"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::clone::Clone for ServiceDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceDocument {}
impl ::core::fmt::Debug for ServiceDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ServiceDocument {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.ServiceDocument;{8b7ec771-2ab3-4dbe-8bcc-778f92b75e51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ServiceDocument {
    type Vtable = IServiceDocument_Vtbl;
    const IID: ::windows_core::GUID = <IServiceDocument as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.ServiceDocument";
}
impl ::core::convert::From<ServiceDocument> for ::windows_core::IUnknown {
    fn from(value: ServiceDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows_core::IUnknown {
    fn from(value: &ServiceDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServiceDocument> for ::windows_core::IInspectable {
    fn from(value: ServiceDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceDocument> for ::windows_core::IInspectable {
    fn from(value: &ServiceDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: ServiceDocument) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<&ServiceDocument> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &ServiceDocument) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for &ServiceDocument {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ServiceDocument {}
unsafe impl ::core::marker::Sync for ServiceDocument {}
#[repr(transparent)]
pub struct Workspace(::windows_core::IUnknown);
impl Workspace {
    #[cfg(feature = "winrt-web")]
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::SyndicationAttribute>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-web"))]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Syndication::ISyndicationNode>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-data", feature = "winrt-web"))]
    pub fn GetXmlDocument(&self, format: super::Syndication::SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<super::Syndication::ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Title(&self) -> ::windows_core::Result<super::Syndication::ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Syndication::ISyndicationText>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Collections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ResourceCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Collections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ResourceCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for Workspace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Workspace {}
impl ::core::fmt::Debug for Workspace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Workspace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Workspace {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.AtomPub.Workspace;{b41da63b-a4b8-4036-89c5-83c31266ba49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Workspace {
    type Vtable = IWorkspace_Vtbl;
    const IID: ::windows_core::GUID = <IWorkspace as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Workspace {
    const NAME: &'static str = "Windows.Web.AtomPub.Workspace";
}
impl ::core::convert::From<Workspace> for ::windows_core::IUnknown {
    fn from(value: Workspace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Workspace> for ::windows_core::IUnknown {
    fn from(value: &Workspace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Workspace> for ::windows_core::IInspectable {
    fn from(value: Workspace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Workspace> for ::windows_core::IInspectable {
    fn from(value: &Workspace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: Workspace) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-web")]
impl ::core::convert::TryFrom<&Workspace> for super::Syndication::ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &Workspace) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-web")]
impl<'a> ::windows_core::IntoParam<'a, super::Syndication::ISyndicationNode> for &Workspace {
    fn into_param(self) -> ::windows_core::Param<'a, super::Syndication::ISyndicationNode> {
        ::core::convert::TryInto::<super::Syndication::ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Workspace {}
unsafe impl ::core::marker::Sync for Workspace {}
