#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundDownloadProgress {}
impl ::core::clone::Clone for BackgroundDownloadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundDownloadProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundDownloadProgress").field("BytesReceived", &self.BytesReceived).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("Status", &self.Status).field("HasResponseChanged", &self.HasResponseChanged).field("HasRestarted", &self.HasRestarted).finish()
    }
}
unsafe impl ::windows_core::Abi for BackgroundDownloadProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BackgroundDownloadProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BackgroundDownloadProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundDownloadProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundDownloadProgress {}
impl ::core::default::Default for BackgroundDownloadProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundDownloader(::windows_core::IUnknown);
impl BackgroundDownloader {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundDownloader, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateDownload<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1) -> ::windows_core::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDownload)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), resultfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateDownloadFromFile<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1, requestbodyfile: Param2) -> ::windows_core::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDownloadFromFile)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodyfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateDownloadAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, resultfile: Param1, requestbodystream: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDownloadAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodystream.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn TransferGroup(&self) -> ::windows_core::Result<BackgroundTransferGroup> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetTransferGroup<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransferGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuccessToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuccessToastNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FailureToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFailureToastNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuccessTileNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuccessTileNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FailureTileNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFailureTileNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CompletionGroup(&self) -> ::windows_core::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows_core::Interface::cast::<IBackgroundDownloader3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompletionGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows_core::Result<BackgroundDownloader> {
        Self::IBackgroundDownloaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithCompletionGroup)(::windows_core::Interface::as_raw(this), completiongroup.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundDownloader>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentDownloadsAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDownloadsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentDownloadsForGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(group: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDownloadsForGroupAsync)(::windows_core::Interface::as_raw(this), group.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentDownloadsForTransferGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDownloadsForTransferGroupAsync)(::windows_core::Interface::as_raw(this), group.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedDownloadsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<DownloadOperation>>>(operations: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundDownloaderUserConsent(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnconstrainedDownloadsAsync)(::windows_core::Interface::as_raw(this), operations.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMethod)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderFactory<R, F: FnOnce(&IBackgroundDownloaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods<R, F: FnOnce(&IBackgroundDownloaderStaticMethods) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods2<R, F: FnOnce(&IBackgroundDownloaderStaticMethods2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundDownloaderUserConsent<R, F: FnOnce(&IBackgroundDownloaderUserConsent) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderUserConsent> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundDownloader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundDownloader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundDownloader {}
impl ::core::fmt::Debug for BackgroundDownloader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundDownloader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundDownloader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundDownloader;{c1c79333-6649-4b1d-a826-a4b3dd234d0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundDownloader {
    type Vtable = IBackgroundDownloader_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundDownloader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundDownloader";
}
impl ::core::convert::From<BackgroundDownloader> for ::windows_core::IUnknown {
    fn from(value: BackgroundDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundDownloader> for ::windows_core::IUnknown {
    fn from(value: &BackgroundDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundDownloader> for ::windows_core::IInspectable {
    fn from(value: BackgroundDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundDownloader> for ::windows_core::IInspectable {
    fn from(value: &BackgroundDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows_core::Error;
    fn try_from(value: BackgroundDownloader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &BackgroundDownloader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferBase> for BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferBase> for &BackgroundDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferBase> {
        ::core::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundDownloader {}
unsafe impl ::core::marker::Sync for BackgroundDownloader {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: Self = Self(0i32);
    pub const Serialized: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundTransferBehavior {}
impl ::core::clone::Clone for BackgroundTransferBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTransferBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BackgroundTransferBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for BackgroundTransferBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroup(::windows_core::IUnknown);
impl BackgroundTransferCompletionGroup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundTransferCompletionGroup, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Trigger(&self) -> ::windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Background::IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BackgroundTransferCompletionGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferCompletionGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferCompletionGroup {}
impl ::core::fmt::Debug for BackgroundTransferCompletionGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCompletionGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferCompletionGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup;{2d930225-986b-574d-7950-0add47f5d706})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroup_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundTransferCompletionGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
}
impl ::core::convert::From<BackgroundTransferCompletionGroup> for ::windows_core::IUnknown {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroup> for ::windows_core::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferCompletionGroup> for ::windows_core::IInspectable {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroup> for ::windows_core::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferCompletionGroup {}
unsafe impl ::core::marker::Sync for BackgroundTransferCompletionGroup {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(::windows_core::IUnknown);
impl BackgroundTransferCompletionGroupTriggerDetails {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Downloads(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Downloads)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Uploads(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uploads)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<UploadOperation>>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundTransferCompletionGroupTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferCompletionGroupTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferCompletionGroupTriggerDetails {}
impl ::core::fmt::Debug for BackgroundTransferCompletionGroupTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCompletionGroupTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferCompletionGroupTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails;{7b6be286-6e47-5136-7fcb-fa4389f46f5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundTransferCompletionGroupTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
}
impl ::core::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferCompletionGroupTriggerDetails {}
unsafe impl ::core::marker::Sync for BackgroundTransferCompletionGroupTriggerDetails {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferContentPart(::windows_core::IUnknown);
impl BackgroundTransferContentPart {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundTransferContentPart, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetFile<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFile)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, filename: Param1) -> ::windows_core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithNameAndFileName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), filename.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferContentPartFactory<R, F: FnOnce(&IBackgroundTransferContentPartFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundTransferContentPart, IBackgroundTransferContentPartFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTransferContentPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferContentPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferContentPart {}
impl ::core::fmt::Debug for BackgroundTransferContentPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferContentPart").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferContentPart {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart;{e8e15657-d7d1-4ed8-838e-674ac217ace6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPart_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundTransferContentPart as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
}
impl ::core::convert::From<BackgroundTransferContentPart> for ::windows_core::IUnknown {
    fn from(value: BackgroundTransferContentPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferContentPart> for ::windows_core::IUnknown {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundTransferContentPart {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferContentPart> for ::windows_core::IInspectable {
    fn from(value: BackgroundTransferContentPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferContentPart> for ::windows_core::IInspectable {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundTransferContentPart {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferContentPart {}
unsafe impl ::core::marker::Sync for BackgroundTransferContentPart {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: Self = Self(0i32);
    pub const UnrestrictedOnly: Self = Self(1i32);
    pub const Always: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferCostPolicy {}
impl ::core::clone::Clone for BackgroundTransferCostPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTransferCostPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BackgroundTransferCostPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for BackgroundTransferCostPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCostPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferCostPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundTransferError;
impl BackgroundTransferError {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Web\"`*"]
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> ::windows_core::Result<super::super::Web::WebErrorStatus> {
        Self::IBackgroundTransferErrorStaticMethods(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Web::WebErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), hresult, result__.as_mut_ptr()).from_abi::<super::super::Web::WebErrorStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferErrorStaticMethods<R, F: FnOnce(&IBackgroundTransferErrorStaticMethods) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundTransferError, IBackgroundTransferErrorStaticMethods> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for BackgroundTransferError {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferError";
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BackgroundTransferFileRange {}
impl ::core::clone::Clone for BackgroundTransferFileRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundTransferFileRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundTransferFileRange").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows_core::Abi for BackgroundTransferFileRange {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferFileRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange;u8;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferFileRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundTransferFileRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundTransferFileRange {}
impl ::core::default::Default for BackgroundTransferFileRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferGroup(::windows_core::IUnknown);
impl BackgroundTransferGroup {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn TransferBehavior(&self) -> ::windows_core::Result<BackgroundTransferBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).TransferBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransferBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<BackgroundTransferGroup> {
        Self::IBackgroundTransferGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateGroup)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundTransferGroup>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferGroupStatics<R, F: FnOnce(&IBackgroundTransferGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundTransferGroup, IBackgroundTransferGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTransferGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferGroup {}
impl ::core::fmt::Debug for BackgroundTransferGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferGroup;{d8c3e3e4-6459-4540-85eb-aaa1c8903677})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroup_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundTransferGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
}
impl ::core::convert::From<BackgroundTransferGroup> for ::windows_core::IUnknown {
    fn from(value: BackgroundTransferGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferGroup> for ::windows_core::IUnknown {
    fn from(value: &BackgroundTransferGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundTransferGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferGroup> for ::windows_core::IInspectable {
    fn from(value: BackgroundTransferGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferGroup> for ::windows_core::IInspectable {
    fn from(value: &BackgroundTransferGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundTransferGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferGroup {}
unsafe impl ::core::marker::Sync for BackgroundTransferGroup {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferPriority {}
impl ::core::clone::Clone for BackgroundTransferPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTransferPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BackgroundTransferPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for BackgroundTransferPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferPriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferRangesDownloadedEventArgs(::windows_core::IUnknown);
impl BackgroundTransferRangesDownloadedEventArgs {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn WasDownloadRestarted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasDownloadRestarted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddedRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundTransferRangesDownloadedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferRangesDownloadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferRangesDownloadedEventArgs {}
impl ::core::fmt::Debug for BackgroundTransferRangesDownloadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferRangesDownloadedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferRangesDownloadedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs;{3ebc7453-bf48-4a88-9248-b0c165184f5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundTransferRangesDownloadedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
}
impl ::core::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferRangesDownloadedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTransferRangesDownloadedEventArgs {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const PausedByApplication: Self = Self(2i32);
    pub const PausedCostedNetwork: Self = Self(3i32);
    pub const PausedNoNetwork: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const PausedRecoverableWebErrorStatus: Self = Self(8i32);
    pub const PausedSystemPolicy: Self = Self(32i32);
}
impl ::core::marker::Copy for BackgroundTransferStatus {}
impl ::core::clone::Clone for BackgroundTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BackgroundTransferStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BackgroundTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundTransferStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundUploadProgress {}
impl ::core::clone::Clone for BackgroundUploadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundUploadProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundUploadProgress").field("BytesReceived", &self.BytesReceived).field("BytesSent", &self.BytesSent).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("TotalBytesToSend", &self.TotalBytesToSend).field("Status", &self.Status).field("HasResponseChanged", &self.HasResponseChanged).field("HasRestarted", &self.HasRestarted).finish()
    }
}
unsafe impl ::windows_core::Abi for BackgroundUploadProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BackgroundUploadProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundUploadProgress;u8;u8;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BackgroundUploadProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundUploadProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundUploadProgress {}
impl ::core::default::Default for BackgroundUploadProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundUploader(::windows_core::IUnknown);
impl BackgroundUploader {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundUploader, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMethod)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateUpload<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, sourcefile: Param1) -> ::windows_core::Result<UploadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUpload)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), sourcefile.into_param().abi(), result__.as_mut_ptr()).from_abi::<UploadOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateUploadFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, sourcestream: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUploadFromStreamAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), sourcestream.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithFormDataAndAutoBoundaryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>>(&self, uri: Param0, parts: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUploadWithFormDataAndAutoBoundaryAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), parts.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithSubTypeAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, uri: Param0, parts: Param1, subtype: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUploadWithSubTypeAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithSubTypeAndBoundaryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, uri: Param0, parts: Param1, subtype: Param2, boundary: Param3) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateUploadWithSubTypeAndBoundaryAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), boundary.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn TransferGroup(&self) -> ::windows_core::Result<BackgroundTransferGroup> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetTransferGroup<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransferGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuccessToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuccessToastNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FailureToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFailureToastNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuccessTileNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuccessTileNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> ::windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FailureTileNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFailureTileNotification)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CompletionGroup(&self) -> ::windows_core::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows_core::Interface::cast::<IBackgroundUploader3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompletionGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows_core::Result<BackgroundUploader> {
        Self::IBackgroundUploaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithCompletionGroup)(::windows_core::Interface::as_raw(this), completiongroup.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundUploader>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentUploadsAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentUploadsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentUploadsForGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(group: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentUploadsForGroupAsync)(::windows_core::Interface::as_raw(this), group.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentUploadsForTransferGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentUploadsForTransferGroupAsync)(::windows_core::Interface::as_raw(this), group.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedUploadsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<UploadOperation>>>(operations: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundUploaderUserConsent(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnconstrainedUploadsAsync)(::windows_core::Interface::as_raw(this), operations.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderFactory<R, F: FnOnce(&IBackgroundUploaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundUploader, IBackgroundUploaderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods<R, F: FnOnce(&IBackgroundUploaderStaticMethods) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods2<R, F: FnOnce(&IBackgroundUploaderStaticMethods2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundUploaderUserConsent<R, F: FnOnce(&IBackgroundUploaderUserConsent) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackgroundUploader, IBackgroundUploaderUserConsent> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundUploader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundUploader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundUploader {}
impl ::core::fmt::Debug for BackgroundUploader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundUploader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundUploader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundUploader;{c595c9ae-cead-465b-8801-c55ac90a01ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundUploader {
    type Vtable = IBackgroundUploader_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundUploader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundUploader";
}
impl ::core::convert::From<BackgroundUploader> for ::windows_core::IUnknown {
    fn from(value: BackgroundUploader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundUploader> for ::windows_core::IUnknown {
    fn from(value: &BackgroundUploader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundUploader> for ::windows_core::IInspectable {
    fn from(value: BackgroundUploader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundUploader> for ::windows_core::IInspectable {
    fn from(value: &BackgroundUploader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows_core::Error;
    fn try_from(value: BackgroundUploader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &BackgroundUploader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferBase> for BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferBase> for &BackgroundUploader {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferBase> {
        ::core::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundUploader {}
unsafe impl ::core::marker::Sync for BackgroundUploader {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct ContentPrefetcher;
impl ContentPrefetcher {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContentUris() -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIndirectContentUri<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IContentPrefetcher(|this| unsafe { (::windows_core::Interface::vtable(this).SetIndirectContentUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IndirectContentUri() -> ::windows_core::Result<super::super::Foundation::Uri> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IndirectContentUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulPrefetchTime() -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        Self::IContentPrefetcherTime(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulPrefetchTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetcher<R, F: FnOnce(&IContentPrefetcher) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContentPrefetcher, IContentPrefetcher> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IContentPrefetcherTime<R, F: FnOnce(&IContentPrefetcherTime) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContentPrefetcher, IContentPrefetcherTime> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ContentPrefetcher";
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct DownloadOperation(::windows_core::IUnknown);
impl DownloadOperation {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Guid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Guid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResultStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn GetResponseInformation(&self) -> ::windows_core::Result<ResponseInformation> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResponseInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Priority(&self) -> ::windows_core::Result<BackgroundTransferPriority> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferPriority>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn ResultFile(&self) -> ::windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResultFile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Progress(&self) -> ::windows_core::Result<BackgroundDownloadProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundDownloadProgress>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundDownloadProgress>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn TransferGroup(&self) -> ::windows_core::Result<BackgroundTransferGroup> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn IsRandomAccessRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRandomAccessRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetIsRandomAccessRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRandomAccessRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultRandomAccessStreamReference(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResultRandomAccessStreamReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDownloadedRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDownloadedRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RangesDownloaded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>>(&self, eventhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RangesDownloaded)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRangesDownloaded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRangesDownloaded)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequestedUri<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestedUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`, `\"Web\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub fn RecoverableWebErrorStatuses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecoverableWebErrorStatuses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`, `\"Web\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    pub fn CurrentWebErrorStatus(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentWebErrorStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).MakeCurrentInTransferGroup)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DownloadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DownloadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DownloadOperation {}
impl ::core::fmt::Debug for DownloadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DownloadOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.DownloadOperation;{bd87ebb0-5714-4e09-ba68-bef73903b0d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DownloadOperation {
    type Vtable = IDownloadOperation_Vtbl;
    const IID: ::windows_core::GUID = <IDownloadOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.DownloadOperation";
}
impl ::core::convert::From<DownloadOperation> for ::windows_core::IUnknown {
    fn from(value: DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadOperation> for ::windows_core::IUnknown {
    fn from(value: &DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DownloadOperation> for ::windows_core::IInspectable {
    fn from(value: DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadOperation> for ::windows_core::IInspectable {
    fn from(value: &DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: DownloadOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &DownloadOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperation> for DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperation> for &DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperation> {
        ::core::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows_core::Error;
    fn try_from(value: DownloadOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows_core::Error;
    fn try_from(value: &DownloadOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperationPriority> for DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperationPriority> for &DownloadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperationPriority> {
        ::core::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DownloadOperation {}
unsafe impl ::core::marker::Sync for DownloadOperation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloader {
    type Vtable = IBackgroundDownloader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1c79333_6649_4b1d_a826_a4b3dd234d0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, resultfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateDownload: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateDownloadFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, resultfile: ::windows_core::RawPtr, requestbodyfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateDownloadFromFile: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateDownloadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, resultfile: ::windows_core::RawPtr, requestbodystream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateDownloadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloader2 {
    type Vtable = IBackgroundDownloader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa94a5847_348d_4a35_890e_8a1ef3798479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloader3 {
    type Vtable = IBackgroundDownloader3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd11a8c48_86e8_48e2_b615_6976aabf861d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloaderFactory {
    type Vtable = IBackgroundDownloaderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26836c24_d89e_46f4_a29a_4f4d4f144155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiongroup: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloaderStaticMethods {
    type Vtable = IBackgroundDownloaderStaticMethods_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52a65a35_c64e_426c_9919_540d0d21a650);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentDownloadsForGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentDownloadsForGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundDownloaderStaticMethods2 {
    type Vtable = IBackgroundDownloaderStaticMethods2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2faa1327_1ad4_4ca5_b2cd_08dbf0746afe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsForTransferGroupAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundDownloaderUserConsent(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IBackgroundDownloaderUserConsent {
    type Vtable = IBackgroundDownloaderUserConsent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d14e906_9266_4808_bd71_5925f2a3130a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderUserConsent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedDownloadsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operations: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedDownloadsAsync: usize,
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct IBackgroundTransferBase(::windows_core::IUnknown);
impl IBackgroundTransferBase {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetMethod<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMethod)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<IBackgroundTransferBase> for ::windows_core::IUnknown {
    fn from(value: IBackgroundTransferBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferBase> for ::windows_core::IUnknown {
    fn from(value: &IBackgroundTransferBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBackgroundTransferBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferBase> for ::windows_core::IInspectable {
    fn from(value: IBackgroundTransferBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferBase> for ::windows_core::IInspectable {
    fn from(value: &IBackgroundTransferBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBackgroundTransferBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferBase {}
impl ::core::fmt::Debug for IBackgroundTransferBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBackgroundTransferBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2a9da250-c769-458c-afe8-feb8d4d3b2ef}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBackgroundTransferBase {
    type Vtable = IBackgroundTransferBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9da250_c769_458c_afe8_feb8d4d3b2ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    #[cfg(feature = "deprecated")]
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGroup: usize,
    pub CostPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows_core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d930225_986b_574d_7950_0add47f5d706);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    Trigger: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b6be286_6e47_5136_7fcb_fa4389f46f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Downloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Downloads: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uploads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uploads: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferContentPart(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPart_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8e15657_d7d1_4ed8_838e_674ac217ace6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPart_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetFile: usize,
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct IBackgroundTransferContentPartFactory(::windows_core::IUnknown);
impl IBackgroundTransferContentPartFactory {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, filename: Param1) -> ::windows_core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithNameAndFileName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), filename.into_param().abi(), result__.as_mut_ptr()).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTransferContentPartFactory> for ::windows_core::IUnknown {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferContentPartFactory> for ::windows_core::IUnknown {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferContentPartFactory> for ::windows_core::IInspectable {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferContentPartFactory> for ::windows_core::IInspectable {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferContentPartFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferContentPartFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferContentPartFactory {}
impl ::core::fmt::Debug for IBackgroundTransferContentPartFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferContentPartFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBackgroundTransferContentPartFactory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{90ef98a9-7a01-4a0b-9f80-a0b0bb370f8d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBackgroundTransferContentPartFactory {
    type Vtable = IBackgroundTransferContentPartFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90ef98a9_7a01_4a0b_9f80_a0b0bb370f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPartFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithNameAndFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferErrorStaticMethods(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferErrorStaticMethods {
    type Vtable = IBackgroundTransferErrorStaticMethods_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaad33b04_1192_4bf4_8b68_39c5add244e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferErrorStaticMethods_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8c3e3e4_6459_4540_85eb_aaa1c8903677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransferBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferBehavior) -> ::windows_core::HRESULT,
    pub SetTransferBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferGroupStatics {
    type Vtable = IBackgroundTransferGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02ec50b2_7d18_495b_aa22_32a97d45d3e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct IBackgroundTransferOperation(::windows_core::IUnknown);
impl IBackgroundTransferOperation {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Guid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Guid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResultStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn GetResponseInformation(&self) -> ::windows_core::Result<ResponseInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResponseInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResponseInformation>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTransferOperation> for ::windows_core::IUnknown {
    fn from(value: IBackgroundTransferOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperation> for ::windows_core::IUnknown {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBackgroundTransferOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferOperation> for ::windows_core::IInspectable {
    fn from(value: IBackgroundTransferOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperation> for ::windows_core::IInspectable {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBackgroundTransferOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferOperation {}
impl ::core::fmt::Debug for IBackgroundTransferOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBackgroundTransferOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ded06846-90ca-44fb-8fb1-124154c0d539}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBackgroundTransferOperation {
    type Vtable = IBackgroundTransferOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xded06846_90ca_44fb_8fb1_124154c0d539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedUri: usize,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    pub CostPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows_core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultStreamAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultStreamAt: usize,
    pub GetResponseInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct IBackgroundTransferOperationPriority(::windows_core::IUnknown);
impl IBackgroundTransferOperationPriority {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Priority(&self) -> ::windows_core::Result<BackgroundTransferPriority> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferPriority>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<IBackgroundTransferOperationPriority> for ::windows_core::IUnknown {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperationPriority> for ::windows_core::IUnknown {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferOperationPriority> for ::windows_core::IInspectable {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperationPriority> for ::windows_core::IInspectable {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferOperationPriority {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferOperationPriority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferOperationPriority {}
impl ::core::fmt::Debug for IBackgroundTransferOperationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferOperationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBackgroundTransferOperationPriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{04854327-5254-4b3a-915e-0aa49275c0f9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBackgroundTransferOperationPriority {
    type Vtable = IBackgroundTransferOperationPriority_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04854327_5254_4b3a_915e_0aa49275c0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperationPriority_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferRangesDownloadedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ebc7453_bf48_4a88_9248_b0c165184f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferRangesDownloadedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WasDownloadRestarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedRanges: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploader {
    type Vtable = IBackgroundUploader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc595c9ae_cead_465b_8801_c55ac90a01ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateUpload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, sourcefile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateUpload: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateUploadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, sourcestream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateUploadFromStreamAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithFormDataAndAutoBoundaryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, parts: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithFormDataAndAutoBoundaryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, parts: ::windows_core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAndBoundaryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, parts: ::windows_core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAndBoundaryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploader2 {
    type Vtable = IBackgroundUploader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e0612ce_0c34_4463_807f_198a1b8bd4ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploader3 {
    type Vtable = IBackgroundUploader3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb95e9439_5bf0_4b3a_8c47_2c6199a854b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploaderFactory {
    type Vtable = IBackgroundUploaderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x736203c7_10e7_48a0_ac3c_1ac71095ec57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiongroup: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploaderStaticMethods {
    type Vtable = IBackgroundUploaderStaticMethods_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2875cfb_9b05_4741_9121_740a83e247df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentUploadsForGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentUploadsForGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundUploaderStaticMethods2 {
    type Vtable = IBackgroundUploaderStaticMethods2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe919ac62_ea08_42f0_a2ac_07e467549080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsForTransferGroupAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundUploaderUserConsent(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IBackgroundUploaderUserConsent {
    type Vtable = IBackgroundUploaderUserConsent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bb384cb_0760_461d_907f_5138f84d44c1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderUserConsent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedUploadsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operations: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedUploadsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentPrefetcher {
    type Vtable = IContentPrefetcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8d6f754_7dc1_4cd9_8810_2a6aa9417e11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContentUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContentUris: usize,
    #[cfg(feature = "Foundation")]
    pub SetIndirectContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIndirectContentUri: usize,
    #[cfg(feature = "Foundation")]
    pub IndirectContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IndirectContentUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetcherTime(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentPrefetcherTime {
    type Vtable = IContentPrefetcherTime_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe361fd08_132a_4fde_a7cc_fcb0e66523af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherTime_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulPrefetchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulPrefetchTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadOperation {
    type Vtable = IDownloadOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd87ebb0_5714_4e09_ba68_bef73903b0d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub ResultFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ResultFile: usize,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundDownloadProgress) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadOperation2 {
    type Vtable = IDownloadOperation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3cced40_8f9c_4353_9cd4_290dee387c38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadOperation3 {
    type Vtable = IDownloadOperation3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5027351c_7d5e_4adc_b8d3_df5c6031b9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRandomAccessRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRandomAccessRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultRandomAccessStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultRandomAccessStreamReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDownloadedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDownloadedRanges: usize,
    #[cfg(feature = "Foundation")]
    pub RangesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RangesDownloaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRangesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRangesDownloaded: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequestedUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequestedUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub RecoverableWebErrorStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web")))]
    RecoverableWebErrorStatuses: usize,
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    pub CurrentWebErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web")))]
    CurrentWebErrorStatus: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadOperation4 {
    type Vtable = IDownloadOperation4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cdaaef4_8cef_404a_966d_f058400bed80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadOperation5 {
    type Vtable = IDownloadOperation5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa699a86f_5590_463a_b8d6_1e491a2760a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResponseInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResponseInformation {
    type Vtable = IResponseInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8bb9a12_f713_4792_8b68_d9d297f91d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResponseInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsResumable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActualUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUri: usize,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IUnconstrainedTransferRequestResult(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IUnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c24b81f_d944_4112_a98e_6a69522b7ebb);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IUnconstrainedTransferRequestResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub IsUnconstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsUnconstrained: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUploadOperation {
    type Vtable = IUploadOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e5624e0_7389_434c_8b35_427fd36bbdae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundUploadProgress) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUploadOperation2 {
    type Vtable = IUploadOperation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x556189f2_2774_4df6_9fa5_209f2bfb12f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUploadOperation3 {
    type Vtable = IUploadOperation3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42c92ca3_de39_4546_bc62_3774b4294de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUploadOperation4 {
    type Vtable = IUploadOperation4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50edef31_fac5_41ee_b030_dc77caee9faa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct ResponseInformation(::windows_core::IUnknown);
impl ResponseInformation {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn IsResumable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsResumable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActualUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn StatusCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).StatusCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ResponseInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResponseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResponseInformation {}
impl ::core::fmt::Debug for ResponseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResponseInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResponseInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.ResponseInformation;{f8bb9a12-f713-4792-8b68-d9d297f91d2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResponseInformation {
    type Vtable = IResponseInformation_Vtbl;
    const IID: ::windows_core::GUID = <IResponseInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ResponseInformation";
}
impl ::core::convert::From<ResponseInformation> for ::windows_core::IUnknown {
    fn from(value: ResponseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResponseInformation> for ::windows_core::IUnknown {
    fn from(value: &ResponseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResponseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResponseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResponseInformation> for ::windows_core::IInspectable {
    fn from(value: ResponseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResponseInformation> for ::windows_core::IInspectable {
    fn from(value: &ResponseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResponseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResponseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResponseInformation {}
unsafe impl ::core::marker::Sync for ResponseInformation {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct UnconstrainedTransferRequestResult(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl UnconstrainedTransferRequestResult {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsUnconstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUnconstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for UnconstrainedTransferRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for UnconstrainedTransferRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for UnconstrainedTransferRequestResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for UnconstrainedTransferRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnconstrainedTransferRequestResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for UnconstrainedTransferRequestResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult;{4c24b81f-d944-4112-a98e-6a69522b7ebb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for UnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResult_Vtbl;
    const IID: ::windows_core::GUID = <IUnconstrainedTransferRequestResult as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for UnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<UnconstrainedTransferRequestResult> for ::windows_core::IUnknown {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&UnconstrainedTransferRequestResult> for ::windows_core::IUnknown {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<UnconstrainedTransferRequestResult> for ::windows_core::IInspectable {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&UnconstrainedTransferRequestResult> for ::windows_core::IInspectable {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for UnconstrainedTransferRequestResult {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for UnconstrainedTransferRequestResult {}
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct UploadOperation(::windows_core::IUnknown);
impl UploadOperation {
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Guid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Guid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn CostPolicy(&self) -> ::windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferCostPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).CostPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCostPolicy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResultStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn GetResponseInformation(&self) -> ::windows_core::Result<ResponseInformation> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResponseInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Priority(&self) -> ::windows_core::Result<BackgroundTransferPriority> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundTransferPriority>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceFile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn Progress(&self) -> ::windows_core::Result<BackgroundUploadProgress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BackgroundUploadProgress>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundUploadProgress>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn TransferGroup(&self) -> ::windows_core::Result<BackgroundTransferGroup> {
        let this = &::windows_core::Interface::cast::<IUploadOperation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUploadOperation3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).MakeCurrentInTransferGroup)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, headername: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestHeader)(::windows_core::Interface::as_raw(this), headername.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UploadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UploadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UploadOperation {}
impl ::core::fmt::Debug for UploadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UploadOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UploadOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UploadOperation;{3e5624e0-7389-434c-8b35-427fd36bbdae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UploadOperation {
    type Vtable = IUploadOperation_Vtbl;
    const IID: ::windows_core::GUID = <IUploadOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UploadOperation";
}
impl ::core::convert::From<UploadOperation> for ::windows_core::IUnknown {
    fn from(value: UploadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UploadOperation> for ::windows_core::IUnknown {
    fn from(value: &UploadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UploadOperation> for ::windows_core::IInspectable {
    fn from(value: UploadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UploadOperation> for ::windows_core::IInspectable {
    fn from(value: &UploadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: UploadOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &UploadOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperation> for UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperation> for &UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperation> {
        ::core::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows_core::Error;
    fn try_from(value: UploadOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows_core::Error;
    fn try_from(value: &UploadOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperationPriority> for UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundTransferOperationPriority> for &UploadOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundTransferOperationPriority> {
        ::core::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UploadOperation {}
unsafe impl ::core::marker::Sync for UploadOperation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
