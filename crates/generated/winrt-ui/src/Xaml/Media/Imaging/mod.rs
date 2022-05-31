#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: Self = Self(0u32);
    pub const IgnoreImageCache: Self = Self(8u32);
}
impl ::core::marker::Copy for BitmapCreateOptions {}
impl ::core::clone::Clone for BitmapCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapCreateOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapCreateOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCreateOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BitmapImage(::windows_core::IUnknown);
impl BitmapImage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapImage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CreateOptions(&self) -> ::windows_core::Result<BitmapCreateOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapCreateOptions>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapCreateOptions>(result__)
        }
    }
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCreateOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UriSource(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UriSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUriSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUriSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DecodePixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodePixelWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodePixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodePixelHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DownloadProgress<'a, Param0: ::windows_core::IntoParam<'a, DownloadProgressEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgress)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDownloadProgress<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDownloadProgress)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImageOpened<'a, Param0: ::windows_core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ImageOpened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImageOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImageFailed<'a, Param0: ::windows_core::IntoParam<'a, super::super::ExceptionRoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ImageFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveImageFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImageFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DecodePixelType(&self) -> ::windows_core::Result<DecodePixelType> {
        let this = &::windows_core::Interface::cast::<IBitmapImage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DecodePixelType>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DecodePixelType>(result__)
        }
    }
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBitmapImage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodePixelType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnimatedBitmap(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnimatedBitmap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPlaying(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaying)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AutoPlay(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoPlay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Play(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Play)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBitmapImage3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(urisource: Param0) -> ::windows_core::Result<BitmapImage> {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(::windows_core::Interface::as_raw(this), urisource.into_param().abi(), result__.as_mut_ptr()).from_abi::<BitmapImage>(result__)
        })
    }
    pub fn CreateOptionsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOptionsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn UriSourceProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UriSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelWidthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelHeightProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DecodePixelTypeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsAnimatedBitmapProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnimatedBitmapProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPlayingProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AutoPlayProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoPlayProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBitmapImageFactory<R, F: FnOnce(&IBitmapImageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapImage, IBitmapImageFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics<R, F: FnOnce(&IBitmapImageStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapImage, IBitmapImageStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics2<R, F: FnOnce(&IBitmapImageStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapImage, IBitmapImageStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapImageStatics3<R, F: FnOnce(&IBitmapImageStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapImage, IBitmapImageStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapImage {}
impl ::core::fmt::Debug for BitmapImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapImage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapImage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.BitmapImage;{31af3271-e3b4-442d-a341-4c0226b2725b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapImage {
    type Vtable = IBitmapImage_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapImage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapImage {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapImage";
}
impl ::core::convert::From<BitmapImage> for ::windows_core::IUnknown {
    fn from(value: BitmapImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows_core::IUnknown {
    fn from(value: &BitmapImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapImage> for ::windows_core::IInspectable {
    fn from(value: BitmapImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapImage> for ::windows_core::IInspectable {
    fn from(value: &BitmapImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapImage> for BitmapSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for BitmapSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, BitmapSource> for BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, BitmapSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, BitmapSource> for &BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, BitmapSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl ::core::convert::From<BitmapImage> for super::ImageSource {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::ImageSource {
    fn from(value: &BitmapImage) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<BitmapImage> for super::super::DependencyObject {
    fn from(value: BitmapImage) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapImage> for super::super::DependencyObject {
    fn from(value: &BitmapImage) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &BitmapImage {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BitmapImage {}
unsafe impl ::core::marker::Sync for BitmapImage {}
#[repr(transparent)]
pub struct BitmapSource(::windows_core::IUnknown);
impl BitmapSource {
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), streamsource.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(::windows_core::Interface::as_raw(this), streamsource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn PixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeightProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBitmapSourceStatics<R, F: FnOnce(&IBitmapSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapSource, IBitmapSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapSource {}
impl ::core::fmt::Debug for BitmapSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.BitmapSource;{23d86411-202f-41b2-8c5b-a8a3b333800b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapSource {
    type Vtable = IBitmapSource_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.BitmapSource";
}
impl ::core::convert::From<BitmapSource> for ::windows_core::IUnknown {
    fn from(value: BitmapSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows_core::IUnknown {
    fn from(value: &BitmapSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapSource> for ::windows_core::IInspectable {
    fn from(value: BitmapSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapSource> for ::windows_core::IInspectable {
    fn from(value: &BitmapSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapSource> for super::ImageSource {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::ImageSource {
    fn from(value: &BitmapSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<BitmapSource> for super::super::DependencyObject {
    fn from(value: BitmapSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BitmapSource> for super::super::DependencyObject {
    fn from(value: &BitmapSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &BitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BitmapSource {}
unsafe impl ::core::marker::Sync for BitmapSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: Self = Self(0i32);
    pub const Logical: Self = Self(1i32);
}
impl ::core::marker::Copy for DecodePixelType {}
impl ::core::clone::Clone for DecodePixelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DecodePixelType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DecodePixelType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DecodePixelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DecodePixelType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.DecodePixelType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DownloadProgressEventArgs(::windows_core::IUnknown);
impl DownloadProgressEventArgs {
    pub fn Progress(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetProgress(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for DownloadProgressEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DownloadProgressEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DownloadProgressEventArgs {}
impl ::core::fmt::Debug for DownloadProgressEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadProgressEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs;{7311e0d4-fe94-4e70-9b90-cdd47ac23afb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDownloadProgressEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows_core::IUnknown {
    fn from(value: DownloadProgressEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DownloadProgressEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DownloadProgressEventArgs> for ::windows_core::IInspectable {
    fn from(value: DownloadProgressEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadProgressEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DownloadProgressEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DownloadProgressEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DownloadProgressEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::core::marker::Sync for DownloadProgressEventArgs {}
#[repr(transparent)]
pub struct DownloadProgressEventHandler(pub ::windows_core::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DownloadProgressEventHandlerBox::<F> { vtable: &DownloadProgressEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, DownloadProgressEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DownloadProgressEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DownloadProgressEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DownloadProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DownloadProgressEventHandlerBox<F> {
    const VTABLE: DownloadProgressEventHandler_Vtbl = DownloadProgressEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DownloadProgressEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DownloadProgressEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DownloadProgressEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DownloadProgressEventHandler {}
impl ::core::fmt::Debug for DownloadProgressEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadProgressEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1abaee23_74ee_4cc7_99ba_b171e3cda61e);
}
unsafe impl ::windows_core::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1abaee23-74ee-4cc7-99ba-b171e3cda61e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DownloadProgressEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImage {
    type Vtable = IBitmapImage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31af3271_e3b4_442d_a341_4c0226b2725b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapCreateOptions) -> ::windows_core::HRESULT,
    pub SetCreateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapCreateOptions) -> ::windows_core::HRESULT,
    pub UriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DecodePixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDecodePixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub DecodePixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDecodePixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ImageOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveImageOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ImageFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveImageFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImage2 {
    type Vtable = IBitmapImage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1069c1b6_8c9b_4762_be3d_759f5698f2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DecodePixelType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DecodePixelType) -> ::windows_core::HRESULT,
    pub SetDecodePixelType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DecodePixelType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImage3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImage3 {
    type Vtable = IBitmapImage3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1de6f26_3c73_453f_a7ba_9b85c18b3733);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAnimatedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9132978_4810_4e5e_8087_03671ee60d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urisource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImageStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e282143_70e8_437c_9fa4_2cbf295cff84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateOptionsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DecodePixelWidthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DecodePixelHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImageStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageStatics2 {
    type Vtable = IBitmapImageStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5f5576a_75af_41a4_b893_8fe91fee2882);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DecodePixelTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapImageStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageStatics3 {
    type Vtable = IBitmapImageStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b44e30d_f6d5_4411_a8cd_bf7603c4faa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAnimatedBitmapProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPlayingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSource {
    type Vtable = IBitmapSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23d86411_202f_41b2_8c5b_a8a3b333800b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamsource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSource: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamsource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe240420e_d4a7_49a4_a0b4_a59fdd77e508);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a9c9981_827b_4e51_891b_8a15b511842d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PixelWidthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadProgressEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7311e0d4_fe94_4e70_9b90_cdd47ac23afb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderTargetBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x500dee81_893c_4c0a_8fec_4678ac717589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RenderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RenderToSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPixelsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPixelsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRenderTargetBitmapStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0a1efee_c131_4d40_9c47_f7d7cf2b077f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PixelWidthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2dd9ed0_d3c5_4056_91b5_b7c1d1e8130e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISurfaceImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62f7d416_c714_4c4c_8273_f839bc58135c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISurfaceImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ab2212a_ef65_4a5f_bfac_73993e8c12c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISvgImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e1cec3_0ca8_4a4e_8d7c_c808a0838586);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RasterizePixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRasterizePixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RasterizePixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRasterizePixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub OpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamsource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISvgImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc794e9e7_cf23_4d72_bf1a_dfaa16d8ea52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urisource: ::windows_core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISvgImageSourceFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68bb3170_3ccc_4035_ac01_9834543d744e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SvgImageSourceLoadStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISvgImageSourceOpenedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85ef4c16_748e_4008_95c7_6a23dd7316db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISvgImageSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c6638ce_bed1_4aab_acbb_d3e2185d315a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RasterizePixelWidthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RasterizePixelHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVirtualSurfaceImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a711fea_bfac_11e0_a06a_9de44724019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ab2212a_bfac_11e0_8a92_69e44724019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, isopaque: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWriteableBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf0b7e6f_df7c_4a85_8413_a1216285835c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PixelBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelBuffer: usize,
    pub Invalidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWriteableBitmapFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5563ebb1_3ef2_42c5_9c6d_1cf5dcc041ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTask(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d5fe9aa_533e_44b8_a975_fc5f1e3bff52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3d1bb63_38f8_4da3_9fca_fd8128a2cbf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c2a6997_a908_4711_b4b2_a960db3d8e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub OnRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskinstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    OnRun: usize,
}
#[repr(transparent)]
pub struct RenderTargetBitmap(::windows_core::IUnknown);
impl RenderTargetBitmap {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RenderTargetBitmap, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn RenderAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, element: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenderAsync)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RenderToSizeAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, element: Param0, scaledwidth: i32, scaledheight: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenderToSizeAsync)(::windows_core::Interface::as_raw(this), element.into_param().abi(), scaledwidth, scaledheight, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPixelsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn PixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeightProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRenderTargetBitmapStatics<R, F: FnOnce(&IRenderTargetBitmapStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RenderTargetBitmap, IRenderTargetBitmapStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RenderTargetBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RenderTargetBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RenderTargetBitmap {}
impl ::core::fmt::Debug for RenderTargetBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderTargetBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RenderTargetBitmap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap;{500dee81-893c-4c0a-8fec-4678ac717589})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_Vtbl;
    const IID: ::windows_core::GUID = <IRenderTargetBitmap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows_core::IUnknown {
    fn from(value: RenderTargetBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows_core::IUnknown {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RenderTargetBitmap> for ::windows_core::IInspectable {
    fn from(value: RenderTargetBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for ::windows_core::IInspectable {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::ImageSource {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::ImageSource {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: RenderTargetBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RenderTargetBitmap> for super::super::DependencyObject {
    fn from(value: &RenderTargetBitmap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &RenderTargetBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RenderTargetBitmap {}
unsafe impl ::core::marker::Sync for RenderTargetBitmap {}
#[repr(transparent)]
pub struct SoftwareBitmapSource(::windows_core::IUnknown);
impl SoftwareBitmapSource {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SoftwareBitmapSource, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(&self, softwarebitmap: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetBitmapAsync)(::windows_core::Interface::as_raw(this), softwarebitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for SoftwareBitmapSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SoftwareBitmapSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SoftwareBitmapSource {}
impl ::core::fmt::Debug for SoftwareBitmapSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoftwareBitmapSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource;{d2dd9ed0-d3c5-4056-91b5-b7c1d1e8130e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_Vtbl;
    const IID: ::windows_core::GUID = <ISoftwareBitmapSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows_core::IUnknown {
    fn from(value: SoftwareBitmapSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows_core::IUnknown {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for ::windows_core::IInspectable {
    fn from(value: SoftwareBitmapSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for ::windows_core::IInspectable {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SoftwareBitmapSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SoftwareBitmapSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SoftwareBitmapSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SoftwareBitmapSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::ImageSource {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::ImageSource {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: SoftwareBitmapSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SoftwareBitmapSource> for super::super::DependencyObject {
    fn from(value: &SoftwareBitmapSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SoftwareBitmapSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SoftwareBitmapSource {}
unsafe impl ::core::marker::Sync for SoftwareBitmapSource {}
#[repr(transparent)]
pub struct SurfaceImageSource(::windows_core::IUnknown);
impl SurfaceImageSource {
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows_core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensions_compose<T: ::windows_core::Compose>(pixelwidth: i32, pixelheight: i32, compose: T) -> ::windows_core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows_core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensionsAndOpacity)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, isopaque, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity_compose<T: ::windows_core::Compose>(pixelwidth: i32, pixelheight: i32, isopaque: bool, compose: T) -> ::windows_core::Result<SurfaceImageSource> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensionsAndOpacity)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, isopaque, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<SurfaceImageSource>(result__)
        })
    }
    pub fn ISurfaceImageSourceFactory<R, F: FnOnce(&ISurfaceImageSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SurfaceImageSource, ISurfaceImageSourceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SurfaceImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SurfaceImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SurfaceImageSource {}
impl ::core::fmt::Debug for SurfaceImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SurfaceImageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SurfaceImageSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SurfaceImageSource;{62f7d416-c714-4c4c-8273-f839bc58135c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SurfaceImageSource {
    type Vtable = ISurfaceImageSource_Vtbl;
    const IID: ::windows_core::GUID = <ISurfaceImageSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
impl ::core::convert::From<SurfaceImageSource> for ::windows_core::IUnknown {
    fn from(value: SurfaceImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows_core::IUnknown {
    fn from(value: &SurfaceImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SurfaceImageSource> for ::windows_core::IInspectable {
    fn from(value: SurfaceImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SurfaceImageSource> for ::windows_core::IInspectable {
    fn from(value: &SurfaceImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::ImageSource {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::ImageSource {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: SurfaceImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &SurfaceImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SurfaceImageSource {}
unsafe impl ::core::marker::Sync for SurfaceImageSource {}
#[repr(transparent)]
pub struct SvgImageSource(::windows_core::IUnknown);
impl SvgImageSource {
    pub fn UriSource(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UriSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUriSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUriSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RasterizePixelWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRasterizePixelWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RasterizePixelHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRasterizePixelHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn OpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpenFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, streamsource: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SvgImageSourceLoadStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(::windows_core::Interface::as_raw(this), streamsource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SvgImageSourceLoadStatus>>(result__)
        }
    }
    pub fn new() -> ::windows_core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<SvgImageSource>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<SvgImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithUriSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(urisource: Param0) -> ::windows_core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(::windows_core::Interface::as_raw(this), urisource.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<SvgImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithUriSource_compose<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, T: ::windows_core::Compose>(urisource: Param0, compose: T) -> ::windows_core::Result<SvgImageSource> {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(::windows_core::Interface::as_raw(this), urisource.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<SvgImageSource>(result__)
        })
    }
    pub fn UriSourceProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UriSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RasterizePixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelWidthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RasterizePixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelHeightProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISvgImageSourceFactory<R, F: FnOnce(&ISvgImageSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SvgImageSource, ISvgImageSourceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISvgImageSourceStatics<R, F: FnOnce(&ISvgImageSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SvgImageSource, ISvgImageSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SvgImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SvgImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SvgImageSource {}
impl ::core::fmt::Debug for SvgImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SvgImageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SvgImageSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSource;{03e1cec3-0ca8-4a4e-8d7c-c808a0838586})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SvgImageSource {
    type Vtable = ISvgImageSource_Vtbl;
    const IID: ::windows_core::GUID = <ISvgImageSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSource";
}
impl ::core::convert::From<SvgImageSource> for ::windows_core::IUnknown {
    fn from(value: SvgImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows_core::IUnknown {
    fn from(value: &SvgImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SvgImageSource> for ::windows_core::IInspectable {
    fn from(value: SvgImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSource> for ::windows_core::IInspectable {
    fn from(value: &SvgImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SvgImageSource> for super::ImageSource {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::ImageSource {
    fn from(value: &SvgImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<SvgImageSource> for super::super::DependencyObject {
    fn from(value: SvgImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SvgImageSource> for super::super::DependencyObject {
    fn from(value: &SvgImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SvgImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SvgImageSource {}
unsafe impl ::core::marker::Sync for SvgImageSource {}
#[repr(transparent)]
pub struct SvgImageSourceFailedEventArgs(::windows_core::IUnknown);
impl SvgImageSourceFailedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<SvgImageSourceLoadStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SvgImageSourceLoadStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SvgImageSourceLoadStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for SvgImageSourceFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SvgImageSourceFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SvgImageSourceFailedEventArgs {}
impl ::core::fmt::Debug for SvgImageSourceFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SvgImageSourceFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs;{68bb3170-3ccc-4035-ac01-9834543d744e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISvgImageSourceFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SvgImageSourceFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SvgImageSourceFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSourceFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SvgImageSourceFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SvgImageSourceFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for SvgImageSourceLoadStatus {}
impl ::core::clone::Clone for SvgImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SvgImageSourceLoadStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SvgImageSourceLoadStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SvgImageSourceLoadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SvgImageSourceLoadStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SvgImageSourceOpenedEventArgs(::windows_core::IUnknown);
impl SvgImageSourceOpenedEventArgs {}
impl ::core::clone::Clone for SvgImageSourceOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SvgImageSourceOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SvgImageSourceOpenedEventArgs {}
impl ::core::fmt::Debug for SvgImageSourceOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SvgImageSourceOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs;{85ef4c16-748e-4008-95c7-6a23dd7316db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISvgImageSourceOpenedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SvgImageSourceOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SvgImageSourceOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SvgImageSourceOpenedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SvgImageSourceOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SvgImageSourceOpenedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[repr(transparent)]
pub struct VirtualSurfaceImageSource(::windows_core::IUnknown);
impl VirtualSurfaceImageSource {
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows_core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, result__.as_mut_ptr()).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(pixelwidth: i32, pixelheight: i32, isopaque: bool) -> ::windows_core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensionsAndOpacity)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, isopaque, result__.as_mut_ptr()).from_abi::<VirtualSurfaceImageSource>(result__)
        })
    }
    pub fn IVirtualSurfaceImageSourceFactory<R, F: FnOnce(&IVirtualSurfaceImageSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VirtualSurfaceImageSource, IVirtualSurfaceImageSourceFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VirtualSurfaceImageSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VirtualSurfaceImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VirtualSurfaceImageSource {}
impl ::core::fmt::Debug for VirtualSurfaceImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualSurfaceImageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VirtualSurfaceImageSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource;{4a711fea-bfac-11e0-a06a-9de44724019b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_Vtbl;
    const IID: ::windows_core::GUID = <IVirtualSurfaceImageSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows_core::IUnknown {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows_core::IUnknown {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for ::windows_core::IInspectable {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for ::windows_core::IInspectable {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for SurfaceImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SurfaceImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, SurfaceImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SurfaceImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, SurfaceImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<SurfaceImageSource>::into(self))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::ImageSource {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: VirtualSurfaceImageSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VirtualSurfaceImageSource> for super::super::DependencyObject {
    fn from(value: &VirtualSurfaceImageSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &VirtualSurfaceImageSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::core::marker::Sync for VirtualSurfaceImageSource {}
#[repr(transparent)]
pub struct WriteableBitmap(::windows_core::IUnknown);
impl WriteableBitmap {
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Invalidate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invalidate)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> ::windows_core::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(::windows_core::Interface::as_raw(this), pixelwidth, pixelheight, result__.as_mut_ptr()).from_abi::<WriteableBitmap>(result__)
        })
    }
    pub fn IWriteableBitmapFactory<R, F: FnOnce(&IWriteableBitmapFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WriteableBitmap, IWriteableBitmapFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WriteableBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WriteableBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WriteableBitmap {}
impl ::core::fmt::Debug for WriteableBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WriteableBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WriteableBitmap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.WriteableBitmap;{bf0b7e6f-df7c-4a85-8413-a1216285835c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WriteableBitmap {
    type Vtable = IWriteableBitmap_Vtbl;
    const IID: ::windows_core::GUID = <IWriteableBitmap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.WriteableBitmap";
}
impl ::core::convert::From<WriteableBitmap> for ::windows_core::IUnknown {
    fn from(value: WriteableBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows_core::IUnknown {
    fn from(value: &WriteableBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WriteableBitmap> for ::windows_core::IInspectable {
    fn from(value: WriteableBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WriteableBitmap> for ::windows_core::IInspectable {
    fn from(value: &WriteableBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WriteableBitmap> for BitmapSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for BitmapSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, BitmapSource> for WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, BitmapSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, BitmapSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, BitmapSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<BitmapSource>::into(self))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::ImageSource {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::ImageSource {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ImageSource> for &WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::ImageSource> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ImageSource>::into(self))
    }
}
impl ::core::convert::From<WriteableBitmap> for super::super::DependencyObject {
    fn from(value: WriteableBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&WriteableBitmap> for super::super::DependencyObject {
    fn from(value: &WriteableBitmap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &WriteableBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for WriteableBitmap {}
unsafe impl ::core::marker::Sync for WriteableBitmap {}
#[repr(transparent)]
pub struct XamlRenderingBackgroundTask(::windows_core::IUnknown);
impl XamlRenderingBackgroundTask {}
impl ::core::clone::Clone for XamlRenderingBackgroundTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlRenderingBackgroundTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlRenderingBackgroundTask {}
impl ::core::fmt::Debug for XamlRenderingBackgroundTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlRenderingBackgroundTask").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask;{5d5fe9aa-533e-44b8-a975-fc5f1e3bff52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_Vtbl;
    const IID: ::windows_core::GUID = <IXamlRenderingBackgroundTask as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows_core::IUnknown {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows_core::IUnknown {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlRenderingBackgroundTask> for ::windows_core::IInspectable {
    fn from(value: XamlRenderingBackgroundTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlRenderingBackgroundTask> for ::windows_core::IInspectable {
    fn from(value: &XamlRenderingBackgroundTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlRenderingBackgroundTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::core::marker::Sync for XamlRenderingBackgroundTask {}
