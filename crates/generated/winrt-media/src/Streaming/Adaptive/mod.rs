#[repr(transparent)]
pub struct AdaptiveMediaSource(::windows_core::IUnknown);
impl AdaptiveMediaSource {
    pub fn IsLive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DesiredLiveOffset(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredLiveOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDesiredLiveOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredLiveOffset)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn InitialBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).InitialBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetInitialBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentDownloadBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentDownloadBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CurrentPlaybackBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPlaybackBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableBitrates(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableBitrates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn DesiredMinBitrate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredMinBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetDesiredMinBitrate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredMinBitrate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DesiredMaxBitrate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredMaxBitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetDesiredMaxBitrate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredMaxBitrate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AudioOnlyPlayback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AudioOnlyPlayback)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn InboundBitsPerSecond(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).InboundBitsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn InboundBitsPerSecondWindow(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).InboundBitsPerSecondWindow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetInboundBitsPerSecondWindow<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInboundBitsPerSecondWindow)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DownloadBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadBitrateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadBitrateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PlaybackBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackBitrateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePlaybackBitrateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackBitrateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DownloadRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DownloadCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DownloadFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AdvancedSettings(&self) -> ::windows_core::Result<AdaptiveMediaSourceAdvancedSettings> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdvancedSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceAdvancedSettings>(result__)
        }
    }
    pub fn MinLiveOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinLiveOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn MaxSeekableWindowSize(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekableWindowSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn DesiredSeekableWindowSize(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredSeekableWindowSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetDesiredSeekableWindowSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredSeekableWindowSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Diagnostics(&self) -> ::windows_core::Result<AdaptiveMediaSourceDiagnostics> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Diagnostics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDiagnostics>(result__)
        }
    }
    pub fn GetCorrelatedTimes(&self) -> ::windows_core::Result<AdaptiveMediaSourceCorrelatedTimes> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCorrelatedTimes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceCorrelatedTimes>(result__)
        }
    }
    pub fn IsContentTypeSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contenttype: Param0) -> ::windows_core::Result<bool> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentTypeSupported)(::windows_core::Interface::as_raw(this), contenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CreateFromUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(feature = "Web_Http")]
    pub fn CreateFromUriWithDownloaderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_web::Http::HttpClient>>(uri: Param0, httpclient: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithDownloaderAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), httpclient.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(stream: Param0, uri: Param1, contenttype: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Http"))]
    pub fn CreateFromStreamWithDownloaderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_web::Http::HttpClient>>(stream: Param0, uri: Param1, contenttype: Param2, httpclient: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithDownloaderAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), httpclient.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IAdaptiveMediaSourceStatics<R, F: FnOnce(&IAdaptiveMediaSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdaptiveMediaSource, IAdaptiveMediaSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSource {}
impl ::core::fmt::Debug for AdaptiveMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSource;{4c7332ef-d39f-4396-b4d9-043957a7c964})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
}
impl ::core::convert::From<AdaptiveMediaSource> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSource> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSource> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSource> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AdaptiveMediaSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AdaptiveMediaSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<&AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Core::IMediaSource> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Core::IMediaSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Core::IMediaSource> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Core::IMediaSource> {
        ::core::convert::TryInto::<super::super::Core::IMediaSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSource {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSource {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceAdvancedSettings(::windows_core::IUnknown);
impl AdaptiveMediaSourceAdvancedSettings {
    pub fn AllSegmentsIndependent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllSegmentsIndependent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllSegmentsIndependent(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllSegmentsIndependent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredBitrateHeadroomRatio(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredBitrateHeadroomRatio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetDesiredBitrateHeadroomRatio<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredBitrateHeadroomRatio)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BitrateDowngradeTriggerRatio(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitrateDowngradeTriggerRatio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetBitrateDowngradeTriggerRatio<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitrateDowngradeTriggerRatio)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceAdvancedSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceAdvancedSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceAdvancedSettings {}
impl ::core::fmt::Debug for AdaptiveMediaSourceAdvancedSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceAdvancedSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceAdvancedSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings;{55db1680-1aeb-47dc-aa08-9a11610ba45a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceAdvancedSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
}
impl ::core::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceAdvancedSettings {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceAdvancedSettings {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceCorrelatedTimes(::windows_core::IUnknown);
impl AdaptiveMediaSourceCorrelatedTimes {
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn PresentationTimeStamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PresentationTimeStamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ProgramDateTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProgramDateTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceCorrelatedTimes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceCorrelatedTimes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceCorrelatedTimes {}
impl ::core::fmt::Debug for AdaptiveMediaSourceCorrelatedTimes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCorrelatedTimes").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceCorrelatedTimes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes;{05108787-e032-48e1-ab8d-002b0b3051df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceCorrelatedTimes as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
}
impl ::core::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceCorrelatedTimes {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceCorrelatedTimes {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationResult(::windows_core::IUnknown);
impl AdaptiveMediaSourceCreationResult {
    pub fn Status(&self) -> ::windows_core::Result<AdaptiveMediaSourceCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceCreationStatus>(result__)
        }
    }
    pub fn MediaSource(&self) -> ::windows_core::Result<AdaptiveMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSource>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows_core::Result<::winrt_web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HttpResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_web::Http::HttpResponseMessage>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceCreationResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceCreationResult {}
impl ::core::fmt::Debug for AdaptiveMediaSourceCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceCreationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult;{4686b6b2-800f-4e31-9093-76d4782013e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceCreationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
}
impl ::core::convert::From<AdaptiveMediaSourceCreationResult> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceCreationResult> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceCreationResult {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceCreationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const ManifestDownloadFailure: Self = Self(1i32);
    pub const ManifestParseFailure: Self = Self(2i32);
    pub const UnsupportedManifestContentType: Self = Self(3i32);
    pub const UnsupportedManifestVersion: Self = Self(4i32);
    pub const UnsupportedManifestProfile: Self = Self(5i32);
    pub const UnknownFailure: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceCreationStatus {}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdaptiveMediaSourceCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    pub fn DiagnosticType(&self) -> ::windows_core::Result<AdaptiveMediaSourceDiagnosticType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceDiagnosticType>::zeroed();
            (::windows_core::Interface::vtable(this).DiagnosticType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDiagnosticType>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i32>>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SegmentId(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SegmentId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ResourceType(&self) -> ::windows_core::Result<::winrt_foundation::IReference<AdaptiveMediaSourceResourceType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<AdaptiveMediaSourceResourceType>>(result__)
        }
    }
    pub fn ResourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ResourceByteRangeLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn Bitrate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bitrate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ResourceDuration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnosticAvailableEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs;{3af64f06-6d9c-494a-b7a9-b3a5dee6ad68})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDiagnosticAvailableEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: Self = Self(0i32);
    pub const ManifestMismatchUponReload: Self = Self(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: Self = Self(2i32);
    pub const MediaSegmentSkipped: Self = Self(3i32);
    pub const ResourceNotFound: Self = Self(4i32);
    pub const ResourceTimedOut: Self = Self(5i32);
    pub const ResourceParsingError: Self = Self(6i32);
    pub const BitrateDisabled: Self = Self(7i32);
    pub const FatalMediaSourceError: Self = Self(8i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnosticType {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceDiagnosticType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdaptiveMediaSourceDiagnosticType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnosticType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnosticType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDiagnosticType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(::windows_core::IUnknown);
impl AdaptiveMediaSourceDiagnostics {
    pub fn DiagnosticAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DiagnosticAvailable)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDiagnosticAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDiagnosticAvailable)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDiagnostics {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnostics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDiagnostics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics;{9b24ee68-962e-448c-aebf-b29b56098e23})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDiagnostics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDiagnostics {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDiagnostics {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    pub fn OldValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OldValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn NewValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NewValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceDownloadBitrateChangedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDownloadBitrateChangedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadBitrateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs;{670c0a44-e04e-4eff-816a-17399f78f4ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadBitrateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: Self = Self(0i32);
    pub const InsufficientInboundBitsPerSecond: Self = Self(1i32);
    pub const LowBufferLevel: Self = Self(2i32);
    pub const PositionChanged: Self = Self(3i32);
    pub const TrackSelectionChanged: Self = Self(4i32);
    pub const DesiredBitratesChanged: Self = Self(5i32);
    pub const ErrorInPreviousBitrate: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadBitrateChangedReason {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdaptiveMediaSourceDownloadBitrateChangedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadBitrateChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadCompletedEventArgs {
    pub fn ResourceType(&self) -> ::windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceResourceType>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    pub fn ResourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ResourceByteRangeLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows_core::Result<::winrt_web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HttpResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_web::Http::HttpResponseMessage>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Statistics(&self) -> ::windows_core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Statistics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceDuration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadCompletedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs;{19240dc3-5b37-4a1a-8970-d621cb6ca83b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadCompletedEventArgs {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadFailedEventArgs {
    pub fn ResourceType(&self) -> ::windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceResourceType>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    pub fn ResourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ResourceByteRangeLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows_core::Result<::winrt_web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HttpResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_web::Http::HttpResponseMessage>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Statistics(&self) -> ::windows_core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Statistics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceDuration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadFailedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs;{37739048-f4ab-40a4-b135-c6dfd8bd7ff1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadFailedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadFailedEventArgs {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadRequestedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadRequestedDeferral {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadRequestedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral;{05c68f64-fa20-4dbd-9821-4bf4c9bf77ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadRequestedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadRequestedDeferral {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadRequestedDeferral {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadRequestedEventArgs {
    pub fn ResourceType(&self) -> ::windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveMediaSourceResourceType>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    pub fn ResourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ResourceByteRangeLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn Result(&self) -> ::windows_core::Result<AdaptiveMediaSourceDownloadResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDownloadResult>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<AdaptiveMediaSourceDownloadRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveMediaSourceDownloadRequestedDeferral>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceDuration(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn ResourceContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadRequestedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs;{c83fdffd-44a9-47a2-bf96-03398b4bfaaf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadRequestedEventArgs {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadResult(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadResult {
    pub fn ResourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetResourceUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResourceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetInputStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInputStream)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBuffer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendedStatus(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetExtendedStatus(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendedStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResourceByteRangeOffset(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn SetResourceByteRangeOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetResourceByteRangeOffset)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResourceByteRangeLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceByteRangeLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn SetResourceByteRangeLength<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetResourceByteRangeLength)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadResult {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult;{f4afdc73-bcee-4a6a-9f0a-fec41e2339b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadResult {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadResult {}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadStatistics(::windows_core::IUnknown);
impl AdaptiveMediaSourceDownloadStatistics {
    pub fn ContentBytesReceivedCount(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ContentBytesReceivedCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TimeToHeadersReceived(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToHeadersReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn TimeToFirstByteReceived(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToFirstByteReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn TimeToLastByteReceived(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToLastByteReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadStatistics {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceDownloadStatistics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics;{a306cefb-e96a-4dff-a9b8-1ae08c01ae98})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourceDownloadStatistics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadStatistics {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadStatistics {}
#[repr(transparent)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(::windows_core::IUnknown);
impl AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    pub fn OldValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OldValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn NewValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).NewValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AudioOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AudioOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourcePlaybackBitrateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs;{23a29f6d-7dda-4a51-87a9-6fa8c5b292be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: Self = Self(0i32);
    pub const InitializationSegment: Self = Self(1i32);
    pub const MediaSegment: Self = Self(2i32);
    pub const Key: Self = Self(3i32);
    pub const InitializationVector: Self = Self(4i32);
    pub const MediaSegmentIndex: Self = Self(5i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceResourceType {}
impl ::core::clone::Clone for AdaptiveMediaSourceResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceResourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdaptiveMediaSourceResourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceResourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveMediaSourceResourceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c7332ef_d39f_4396_b4d9_043957a7c964);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DesiredLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDesiredLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub InitialBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetInitialBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CurrentDownloadBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CurrentPlaybackBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableBitrates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableBitrates: usize,
    pub DesiredMinBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredMinBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DesiredMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AudioOnlyPlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub InboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetInboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DownloadBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlaybackBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DownloadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DownloadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSource2 {
    type Vtable = IAdaptiveMediaSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17890342_6760_4bb9_a58a_f7aa98b08c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AdvancedSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSource3 {
    type Vtable = IAdaptiveMediaSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba7023fd_c334_461b_a36e_c99f54f7174a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Diagnostics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCorrelatedTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceAdvancedSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55db1680_1aeb_47dc_aa08_9a11610ba45a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceAdvancedSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllSegmentsIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllSegmentsIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCorrelatedTimes(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05108787_e032_48e1_ab8d_002b0b3051df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCorrelatedTimes_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PresentationTimeStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProgramDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4686b6b2_800f_4e31_9093_76d4782013e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows_core::HRESULT,
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceCreationResult2 {
    type Vtable = IAdaptiveMediaSourceCreationResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c3243bf_1c44_404b_a201_df45ac7898e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3af64f06_6d9c_494a_b7a9_b3a5dee6ad68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DiagnosticType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SegmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c6dd857_16a5_4d9f_810e_00bd901b3ef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3650cd5_daeb_4103_84da_68769ad513ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnostics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b24ee68_962e_448c_aebf_b29b56098e23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnostics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DiagnosticAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDiagnosticAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x670c0a44_e04e_4eff_816a_17399f78f4ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3f1f444_96ae_4de0_b540_2b3246e6968c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19240dc3_5b37_4a1a_8970_d621cb6ca83b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x704744c4_964a_40e4_af95_9177dd6dfa00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f8a8bd1_93b2_47c6_badc_8be2c8f7f6e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37739048_f4ab_40a4_b135_c6dfd8bd7ff1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70919568_967c_4986_90c5_c6fc4b31e2d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0354549_1132_4a10_915a_c2211b5b9409);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05c68f64_fa20_4dbd_9821_4bf4c9bf77ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc83fdffd_44a9_47a2_bf96_03398b4bfaaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb37d8bfe_aa44_4d82_825b_611de3bcfecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x333c50fd_4f62_4481_ab44_1e47b0574225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4afdc73_bcee_4a6a_9f0a_fec41e2339b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetInputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBuffer: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadResult2 {
    type Vtable = IAdaptiveMediaSourceDownloadResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15552cb7_7b80_4ac4_8660_a4b97f7c70f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadStatistics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa306cefb_e96a_4dff_a9b8_1ae08c01ae98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadStatistics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentBytesReceivedCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TimeToHeadersReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TimeToFirstByteReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TimeToLastByteReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23a29f6d_7dda_4a51_87a9_6fa8c5b292be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AudioOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveMediaSourceStatics {
    type Vtable = IAdaptiveMediaSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50a6bd5d_66ef_4cd3_9579_9e660507dc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CreateFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub CreateFromUriWithDownloaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, httpclient: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    CreateFromUriWithDownloaderAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Http"))]
    pub CreateFromStreamWithDownloaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, httpclient: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "Web_Http")))]
    CreateFromStreamWithDownloaderAsync: usize,
}
