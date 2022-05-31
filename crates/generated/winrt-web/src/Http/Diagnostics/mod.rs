#[repr(transparent)]
pub struct HttpDiagnosticProvider(::windows_core::IUnknown);
impl HttpDiagnosticProvider {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestSent<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSent)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRequestSent<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestSent)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ResponseReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveResponseReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResponseReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RequestResponseCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RequestResponseCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRequestResponseCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestResponseCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn CreateFromProcessDiagnosticInfo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::Diagnostics::ProcessDiagnosticInfo>>(processdiagnosticinfo: Param0) -> ::windows_core::Result<HttpDiagnosticProvider> {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromProcessDiagnosticInfo)(::windows_core::Interface::as_raw(this), processdiagnosticinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpDiagnosticProvider>(result__)
        })
    }
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProvider {}
impl ::core::fmt::Debug for HttpDiagnosticProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProvider;{bd811501-a056-4d39-b174-833b7b03b02c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProvider> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProvider> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProvider {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProvider {}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(::windows_core::IUnknown);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Timestamps(&self) -> ::windows_core::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpDiagnosticProviderRequestResponseTimestamps>(result__)
        }
    }
    pub fn RequestedUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ThreadId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows_core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HttpDiagnosticRequestInitiator>::zeroed();
            (::windows_core::Interface::vtable(this).Initiator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SourceLocations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceLocations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestResponseCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs;{735f98ee-94f6-4532-b26e-61e1b1e4efd4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticProviderRequestResponseCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(::windows_core::IUnknown);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    pub fn CacheCheckedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CacheCheckedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ConnectionInitiatedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionInitiatedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn NameResolvedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NameResolvedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SslNegotiatedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SslNegotiatedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ConnectionCompletedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionCompletedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn RequestSentTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSentTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn RequestCompletedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestCompletedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ResponseReceivedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseReceivedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ResponseCompletedTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCompletedTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestResponseTimestamps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestResponseTimestamps {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestResponseTimestamps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestResponseTimestamps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps;{e0afde10-55cf-4c01-91d4-a20557d849f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticProviderRequestResponseTimestamps as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestResponseTimestamps> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestResponseTimestamps> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestResponseTimestamps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticProviderRequestResponseTimestamps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(::windows_core::IUnknown);
impl HttpDiagnosticProviderRequestSentEventArgs {
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HttpRequestMessage>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ThreadId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ThreadId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Initiator(&self) -> ::windows_core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HttpDiagnosticRequestInitiator>::zeroed();
            (::windows_core::Interface::vtable(this).Initiator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpDiagnosticRequestInitiator>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SourceLocations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceLocations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderRequestSentEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderRequestSentEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderRequestSentEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderRequestSentEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs;{3f5196d0-4c1f-4ebe-a57a-06930771c50d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticProviderRequestSentEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderRequestSentEventArgs> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderRequestSentEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticProviderRequestSentEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticProviderRequestSentEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[repr(transparent)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(::windows_core::IUnknown);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HttpResponseMessage>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticProviderResponseReceivedEventArgs {}
impl ::core::fmt::Debug for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticProviderResponseReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs;{a0a2566c-ab5f-4d66-bb2d-084cf41635d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticProviderResponseReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticProviderResponseReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticProviderResponseReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticProviderResponseReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: Self = Self(0i32);
    pub const Script: Self = Self(1i32);
    pub const Image: Self = Self(2i32);
    pub const Link: Self = Self(3i32);
    pub const Style: Self = Self(4i32);
    pub const XmlHttpRequest: Self = Self(5i32);
    pub const Media: Self = Self(6i32);
    pub const HtmlDownload: Self = Self(7i32);
    pub const Prefetch: Self = Self(8i32);
    pub const Other: Self = Self(9i32);
    pub const CrossOriginPreFlight: Self = Self(10i32);
    pub const Fetch: Self = Self(11i32);
    pub const Beacon: Self = Self(12i32);
}
impl ::core::marker::Copy for HttpDiagnosticRequestInitiator {}
impl ::core::clone::Clone for HttpDiagnosticRequestInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpDiagnosticRequestInitiator {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpDiagnosticRequestInitiator {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpDiagnosticRequestInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticRequestInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(::windows_core::IUnknown);
impl HttpDiagnosticSourceLocation {
    pub fn SourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).LineNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ColumnNumber(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDiagnosticSourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDiagnosticSourceLocation {}
impl ::core::fmt::Debug for HttpDiagnosticSourceLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDiagnosticSourceLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation;{54a9d260-8860-423f-b6fa-d77716f647a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDiagnosticSourceLocation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows_core::IUnknown {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows_core::IUnknown {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDiagnosticSourceLocation> for ::windows_core::IInspectable {
    fn from(value: HttpDiagnosticSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDiagnosticSourceLocation> for ::windows_core::IInspectable {
    fn from(value: &HttpDiagnosticSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDiagnosticSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpDiagnosticSourceLocation {}
unsafe impl ::core::marker::Sync for HttpDiagnosticSourceLocation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd811501_a056_4d39_b174_833b7b03b02c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRequestSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RequestResponseCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRequestResponseCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Timestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestedUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0afde10_55cf_4c01_91d4_a20557d849f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CacheCheckedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConnectionInitiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NameResolvedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SslNegotiatedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConnectionCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestSentTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseReceivedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseCompletedTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Initiator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpDiagnosticRequestInitiator) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SourceLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SourceLocations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticProviderStatics {
    type Vtable = IHttpDiagnosticProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b824ec1_6a6c_47cc_afec_1e86bc26053b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub CreateFromProcessDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processdiagnosticinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    CreateFromProcessDiagnosticInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDiagnosticSourceLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54a9d260_8860_423f_b6fa_d77716f647a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDiagnosticSourceLocation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub ColumnNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
