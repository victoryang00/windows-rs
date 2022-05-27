#[doc(hidden)]
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe132d1b9_7b9d_5820_9728_4262b5289142);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Json: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISysStorageProviderEventReceivedEventArgsFactory {
    type Vtable = ISysStorageProviderEventReceivedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde1a780e_e975_5f68_bcc6_fb46281c6a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderEventSource(::windows_core::IUnknown);
impl ISysStorageProviderEventSource {
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EventReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EventReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEventReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEventReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ISysStorageProviderEventSource> for ::windows_core::IUnknown {
    fn from(value: ISysStorageProviderEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderEventSource> for ::windows_core::IUnknown {
    fn from(value: &ISysStorageProviderEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISysStorageProviderEventSource> for ::windows_core::IInspectable {
    fn from(value: ISysStorageProviderEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderEventSource> for ::windows_core::IInspectable {
    fn from(value: &ISysStorageProviderEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISysStorageProviderEventSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISysStorageProviderEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISysStorageProviderEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderEventSource {}
impl ::core::fmt::Debug for ISysStorageProviderEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISysStorageProviderEventSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1f36c476-9546-536a-8381-2f9a2c08cedd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISysStorageProviderEventSource {
    type Vtable = ISysStorageProviderEventSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f36c476_9546_536a_8381_2f9a2c08cedd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub EventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EventReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEventReceived: usize,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderHandlerFactory(::windows_core::IUnknown);
impl ISysStorageProviderHandlerFactory {
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
    pub fn GetHttpRequestProvider<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, syncrootid: Param0) -> ::windows_core::Result<ISysStorageProviderHttpRequestProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHttpRequestProvider)(::windows_core::Interface::as_raw(this), syncrootid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ISysStorageProviderHttpRequestProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
    pub fn GetEventSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, syncrootid: Param0, eventname: Param1) -> ::windows_core::Result<ISysStorageProviderEventSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEventSource)(::windows_core::Interface::as_raw(this), syncrootid.into_param().abi(), eventname.into_param().abi(), result__.as_mut_ptr()).from_abi::<ISysStorageProviderEventSource>(result__)
        }
    }
}
impl ::core::convert::From<ISysStorageProviderHandlerFactory> for ::windows_core::IUnknown {
    fn from(value: ISysStorageProviderHandlerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderHandlerFactory> for ::windows_core::IUnknown {
    fn from(value: &ISysStorageProviderHandlerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISysStorageProviderHandlerFactory> for ::windows_core::IInspectable {
    fn from(value: ISysStorageProviderHandlerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderHandlerFactory> for ::windows_core::IInspectable {
    fn from(value: &ISysStorageProviderHandlerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISysStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISysStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISysStorageProviderHandlerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderHandlerFactory {}
impl ::core::fmt::Debug for ISysStorageProviderHandlerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderHandlerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISysStorageProviderHandlerFactory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ee798431-8213-5e89-a623-14d8c72b8a61}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISysStorageProviderHandlerFactory {
    type Vtable = ISysStorageProviderHandlerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee798431_8213_5e89_a623_14d8c72b8a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHandlerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetHttpRequestProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, eventname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct ISysStorageProviderHttpRequestProvider(::windows_core::IUnknown);
impl ISysStorageProviderHttpRequestProvider {
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`, `\"Foundation\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub fn SendRequestAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>(result__)
        }
    }
}
impl ::core::convert::From<ISysStorageProviderHttpRequestProvider> for ::windows_core::IUnknown {
    fn from(value: ISysStorageProviderHttpRequestProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderHttpRequestProvider> for ::windows_core::IUnknown {
    fn from(value: &ISysStorageProviderHttpRequestProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISysStorageProviderHttpRequestProvider> for ::windows_core::IInspectable {
    fn from(value: ISysStorageProviderHttpRequestProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISysStorageProviderHttpRequestProvider> for ::windows_core::IInspectable {
    fn from(value: &ISysStorageProviderHttpRequestProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISysStorageProviderHttpRequestProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISysStorageProviderHttpRequestProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISysStorageProviderHttpRequestProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISysStorageProviderHttpRequestProvider {}
impl ::core::fmt::Debug for ISysStorageProviderHttpRequestProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISysStorageProviderHttpRequestProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISysStorageProviderHttpRequestProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{cb6fefb6-e76a-5c25-a33e-3e78a6e0e0ce}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISysStorageProviderHttpRequestProvider {
    type Vtable = ISysStorageProviderHttpRequestProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb6fefb6_e76a_5c25_a33e_3e78a6e0e0ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHttpRequestProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    SendRequestAsync: usize,
}
#[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
#[repr(transparent)]
pub struct SysStorageProviderEventReceivedEventArgs(::windows_core::IUnknown);
impl SysStorageProviderEventReceivedEventArgs {
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
    pub fn Json(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Json)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Implementation_FileExplorer\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(json: Param0) -> ::windows_core::Result<SysStorageProviderEventReceivedEventArgs> {
        Self::ISysStorageProviderEventReceivedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), json.into_param().abi(), result__.as_mut_ptr()).from_abi::<SysStorageProviderEventReceivedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISysStorageProviderEventReceivedEventArgsFactory<R, F: FnOnce(&ISysStorageProviderEventReceivedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SysStorageProviderEventReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SysStorageProviderEventReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SysStorageProviderEventReceivedEventArgs {}
impl ::core::fmt::Debug for SysStorageProviderEventReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SysStorageProviderEventReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs;{e132d1b9-7b9d-5820-9728-4262b5289142})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISysStorageProviderEventReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
}
impl ::core::convert::From<SysStorageProviderEventReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SysStorageProviderEventReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SysStorageProviderEventReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SysStorageProviderEventReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SysStorageProviderEventReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SysStorageProviderEventReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SysStorageProviderEventReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SysStorageProviderEventReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SysStorageProviderEventReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SysStorageProviderEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SysStorageProviderEventReceivedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
