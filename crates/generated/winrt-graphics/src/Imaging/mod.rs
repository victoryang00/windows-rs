#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapAlphaMode {}
impl ::core::clone::Clone for BitmapAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapAlphaMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapAlphaMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapAlphaMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapAlphaMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct BitmapBounds {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapBounds {}
impl ::core::clone::Clone for BitmapBounds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapBounds").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows_core::Abi for BitmapBounds {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BitmapBounds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapBounds;u4;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BitmapBounds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BitmapBounds>()) == 0 }
    }
}
impl ::core::cmp::Eq for BitmapBounds {}
impl ::core::default::Default for BitmapBounds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct BitmapBuffer(::windows_core::IUnknown);
impl BitmapBuffer {
    pub fn GetPlaneCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaneCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GetPlaneDescription(&self, index: i32) -> ::windows_core::Result<BitmapPlaneDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPlaneDescription>::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaneDescription)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<BitmapPlaneDescription>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> ::windows_core::Result<::winrt_foundation::IMemoryBufferReference> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IMemoryBufferReference>(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapBuffer {}
impl ::core::fmt::Debug for BitmapBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapBuffer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapBuffer;{a53e04c4-399c-438c-b28f-a63a6b83d1a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapBuffer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapBuffer";
}
impl ::core::convert::From<BitmapBuffer> for ::windows_core::IUnknown {
    fn from(value: BitmapBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapBuffer> for ::windows_core::IUnknown {
    fn from(value: &BitmapBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapBuffer> for ::windows_core::IInspectable {
    fn from(value: BitmapBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapBuffer> for ::windows_core::IInspectable {
    fn from(value: &BitmapBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BitmapBuffer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapBuffer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapBuffer> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<BitmapBuffer> for ::winrt_foundation::IMemoryBuffer {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapBuffer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapBuffer> for ::winrt_foundation::IMemoryBuffer {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IMemoryBuffer> for BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IMemoryBuffer> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IMemoryBuffer> for &BitmapBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<::winrt_foundation::IMemoryBuffer>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapBuffer {}
unsafe impl ::core::marker::Sync for BitmapBuffer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapBufferAccessMode {}
impl ::core::clone::Clone for BitmapBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapBufferAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapBufferAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapBufferAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapBufferAccessMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapBufferAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BitmapCodecInformation(::windows_core::IUnknown);
impl BitmapCodecInformation {
    pub fn CodecId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CodecId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FileExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MimeTypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MimeTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapCodecInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapCodecInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapCodecInformation {}
impl ::core::fmt::Debug for BitmapCodecInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCodecInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapCodecInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapCodecInformation;{400caaf2-c4b0-4392-a3b0-6f6f9ba95cb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapCodecInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapCodecInformation";
}
impl ::core::convert::From<BitmapCodecInformation> for ::windows_core::IUnknown {
    fn from(value: BitmapCodecInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapCodecInformation> for ::windows_core::IUnknown {
    fn from(value: &BitmapCodecInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapCodecInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapCodecInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapCodecInformation> for ::windows_core::IInspectable {
    fn from(value: BitmapCodecInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapCodecInformation> for ::windows_core::IInspectable {
    fn from(value: &BitmapCodecInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapCodecInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapCodecInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BitmapCodecInformation {}
unsafe impl ::core::marker::Sync for BitmapCodecInformation {}
#[repr(transparent)]
pub struct BitmapDecoder(::windows_core::IUnknown);
impl BitmapDecoder {
    pub fn BitmapContainerProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapContainerProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn DecoderInformation(&self) -> ::windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecoderInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapCodecInformation>(result__)
        }
    }
    pub fn FrameCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).FrameCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetPreviewAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn GetFrameAsync(&self, frameindex: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFrameAsync)(::windows_core::Interface::as_raw(this), frameindex, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapFrame>>(result__)
        }
    }
    pub fn BmpDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).BmpDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn JpegDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).JpegDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn PngDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).PngDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn TiffDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TiffDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn GifDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GifDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn JpegXRDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).JpegXRDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn IcoDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).IcoDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetDecoderInformationEnumerator() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDecoderInformationEnumerator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<BitmapCodecInformation>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapDecoder>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapDecoder>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateWithIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(decoderid: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapDecoder>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithIdAsync)(::windows_core::Interface::as_raw(this), decoderid.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapDecoder>>(result__)
        })
    }
    pub fn HeifDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).HeifDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn WebpDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).WebpDecoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn IBitmapDecoderStatics<R, F: FnOnce(&IBitmapDecoderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapDecoderStatics2<R, F: FnOnce(&IBitmapDecoderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapDecoder {}
impl ::core::fmt::Debug for BitmapDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapDecoder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapDecoder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapDecoder;{acef22ba-1d74-4c91-9dfc-9620745233e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapDecoder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapDecoder";
}
impl ::core::convert::From<BitmapDecoder> for ::windows_core::IUnknown {
    fn from(value: BitmapDecoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapDecoder> for ::windows_core::IUnknown {
    fn from(value: &BitmapDecoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapDecoder> for ::windows_core::IInspectable {
    fn from(value: BitmapDecoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapDecoder> for ::windows_core::IInspectable {
    fn from(value: &BitmapDecoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BitmapDecoder> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapDecoder) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for &BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::core::convert::TryInto::<IBitmapFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<BitmapDecoder> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapDecoder) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for &BitmapDecoder {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::core::convert::TryInto::<IBitmapFrameWithSoftwareBitmap>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapDecoder {}
unsafe impl ::core::marker::Sync for BitmapDecoder {}
#[repr(transparent)]
pub struct BitmapEncoder(::windows_core::IUnknown);
impl BitmapEncoder {
    pub fn EncoderInformation(&self) -> ::windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncoderInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapCodecInformation>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapProperties>(result__)
        }
    }
    pub fn BitmapContainerProperties(&self) -> ::windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapContainerProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapProperties>(result__)
        }
    }
    pub fn IsThumbnailGenerated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsThumbnailGenerated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsThumbnailGenerated)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GeneratedThumbnailWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGeneratedThumbnailWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GeneratedThumbnailHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGeneratedThumbnailHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitmapTransform(&self) -> ::windows_core::Result<BitmapTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapTransform>(result__)
        }
    }
    pub fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPixelData)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, width, height, dpix, dpiy, pixels.len() as u32, ::core::mem::transmute(pixels.as_ptr())).ok() }
    }
    pub fn GoToNextFrameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GoToNextFrameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GoToNextFrameWithEncodingOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>>(&self, encodingoptions: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GoToNextFrameWithEncodingOptionsAsync)(::windows_core::Interface::as_raw(this), encodingoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn FlushAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn BmpEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).BmpEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn JpegEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).JpegEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn PngEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).PngEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn TiffEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TiffEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn GifEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GifEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn JpegXREncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).JpegXREncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetEncoderInformationEnumerator() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEncoderInformationEnumerator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<BitmapCodecInformation>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(encoderid: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), encoderid.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn CreateWithEncodingOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>>(encoderid: Param0, stream: Param1, encodingoptions: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithEncodingOptionsAsync)(::windows_core::Interface::as_raw(this), encoderid.into_param().abi(), stream.into_param().abi(), encodingoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateForTranscodingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, BitmapDecoder>>(stream: Param0, bitmapdecoder: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForTranscodingAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), bitmapdecoder.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    pub fn CreateForInPlacePropertyEncodingAsync<'a, Param0: ::windows_core::IntoParam<'a, BitmapDecoder>>(bitmapdecoder: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForInPlacePropertyEncodingAsync)(::windows_core::Interface::as_raw(this), bitmapdecoder.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    pub fn HeifEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).HeifEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn SetSoftwareBitmap<'a, Param0: ::windows_core::IntoParam<'a, SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBitmapEncoderWithSoftwareBitmap>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
    pub fn IBitmapEncoderStatics<R, F: FnOnce(&IBitmapEncoderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapEncoderStatics2<R, F: FnOnce(&IBitmapEncoderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapEncoder {}
impl ::core::fmt::Debug for BitmapEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapEncoder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapEncoder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapEncoder;{2bc468e3-e1f8-4b54-95e8-32919551ce62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapEncoder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapEncoder";
}
impl ::core::convert::From<BitmapEncoder> for ::windows_core::IUnknown {
    fn from(value: BitmapEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapEncoder> for ::windows_core::IUnknown {
    fn from(value: &BitmapEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapEncoder> for ::windows_core::IInspectable {
    fn from(value: BitmapEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapEncoder> for ::windows_core::IInspectable {
    fn from(value: &BitmapEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BitmapEncoder {}
unsafe impl ::core::marker::Sync for BitmapEncoder {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapFlip {}
impl ::core::clone::Clone for BitmapFlip {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapFlip {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapFlip {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapFlip").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapFlip {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapFlip;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BitmapFrame(::windows_core::IUnknown);
impl BitmapFrame {
    #[cfg(feature = "winrt-storage")]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapFrame {}
impl ::core::fmt::Debug for BitmapFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapFrame;{72a49a1c-8081-438d-91bc-94ecfc8185c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapFrame";
}
impl ::core::convert::From<BitmapFrame> for ::windows_core::IUnknown {
    fn from(value: BitmapFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapFrame> for ::windows_core::IUnknown {
    fn from(value: &BitmapFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapFrame> for ::windows_core::IInspectable {
    fn from(value: BitmapFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapFrame> for ::windows_core::IInspectable {
    fn from(value: &BitmapFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BitmapFrame> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for &BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::core::convert::TryInto::<IBitmapFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<BitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for &BitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::core::convert::TryInto::<IBitmapFrameWithSoftwareBitmap>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapFrame {}
unsafe impl ::core::marker::Sync for BitmapFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const Fant: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapInterpolationMode {}
impl ::core::clone::Clone for BitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapInterpolationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapInterpolationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapInterpolationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapInterpolationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Rgba16: Self = Self(12i32);
    pub const Rgba8: Self = Self(30i32);
    pub const Gray16: Self = Self(57i32);
    pub const Gray8: Self = Self(62i32);
    pub const Bgra8: Self = Self(87i32);
    pub const Nv12: Self = Self(103i32);
    pub const P010: Self = Self(104i32);
    pub const Yuy2: Self = Self(107i32);
}
impl ::core::marker::Copy for BitmapPixelFormat {}
impl ::core::clone::Clone for BitmapPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapPixelFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapPixelFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapPixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPixelFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapPixelFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapPixelFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct BitmapPlaneDescription {
    pub StartIndex: i32,
    pub Width: i32,
    pub Height: i32,
    pub Stride: i32,
}
impl ::core::marker::Copy for BitmapPlaneDescription {}
impl ::core::clone::Clone for BitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapPlaneDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapPlaneDescription").field("StartIndex", &self.StartIndex).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).finish()
    }
}
unsafe impl ::windows_core::Abi for BitmapPlaneDescription {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BitmapPlaneDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapPlaneDescription;i4;i4;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BitmapPlaneDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for BitmapPlaneDescription {}
impl ::core::default::Default for BitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct BitmapProperties(::windows_core::IUnknown);
impl BitmapProperties {
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>>(&self, propertiestoset: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoset.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = &::windows_core::Interface::cast::<IBitmapPropertiesView>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapProperties {}
impl ::core::fmt::Debug for BitmapProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapProperties;{ea9f4f1b-b505-4450-a4d1-e8ca94529d8d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapProperties";
}
impl ::core::convert::From<BitmapProperties> for ::windows_core::IUnknown {
    fn from(value: BitmapProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapProperties> for ::windows_core::IUnknown {
    fn from(value: &BitmapProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapProperties> for ::windows_core::IInspectable {
    fn from(value: BitmapProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapProperties> for ::windows_core::IInspectable {
    fn from(value: &BitmapProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BitmapProperties> for IBitmapPropertiesView {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapProperties) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapProperties> for IBitmapPropertiesView {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapProperties) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapPropertiesView> for BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapPropertiesView> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapPropertiesView> for &BitmapProperties {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapPropertiesView> {
        ::core::convert::TryInto::<IBitmapPropertiesView>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapProperties {}
unsafe impl ::core::marker::Sync for BitmapProperties {}
#[repr(transparent)]
pub struct BitmapPropertiesView(::windows_core::IUnknown);
impl BitmapPropertiesView {
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapPropertiesView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapPropertiesView {}
impl ::core::fmt::Debug for BitmapPropertiesView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPropertiesView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapPropertiesView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertiesView;{7e0fe87a-3a70-48f8-9c55-196cf5a545f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapPropertiesView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertiesView";
}
impl ::core::convert::From<BitmapPropertiesView> for ::windows_core::IUnknown {
    fn from(value: BitmapPropertiesView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapPropertiesView> for ::windows_core::IUnknown {
    fn from(value: &BitmapPropertiesView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapPropertiesView> for ::windows_core::IInspectable {
    fn from(value: BitmapPropertiesView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapPropertiesView> for ::windows_core::IInspectable {
    fn from(value: &BitmapPropertiesView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BitmapPropertiesView> for IBitmapPropertiesView {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapPropertiesView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapPropertiesView> for IBitmapPropertiesView {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapPropertiesView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapPropertiesView> for BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapPropertiesView> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapPropertiesView> for &BitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapPropertiesView> {
        ::core::convert::TryInto::<IBitmapPropertiesView>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapPropertiesView {}
unsafe impl ::core::marker::Sync for BitmapPropertiesView {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct BitmapPropertySet(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl BitmapPropertySet {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapPropertySet, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<BitmapTypedValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<BitmapTypedValue>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, BitmapTypedValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, BitmapTypedValue>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, BitmapTypedValue>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for BitmapPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for BitmapPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for BitmapPropertySet {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for BitmapPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for BitmapPropertySet {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for BitmapPropertySet {
    type Vtable = ::winrt_foundation::Collections::IMap_Vtbl<::windows_core::HSTRING, BitmapTypedValue>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for BitmapPropertySet {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertySet";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for BitmapPropertySet {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &BitmapPropertySet {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<BitmapPropertySet> for ::windows_core::IUnknown {
    fn from(value: BitmapPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&BitmapPropertySet> for ::windows_core::IUnknown {
    fn from(value: &BitmapPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<BitmapPropertySet> for ::windows_core::IInspectable {
    fn from(value: BitmapPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&BitmapPropertySet> for ::windows_core::IInspectable {
    fn from(value: &BitmapPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<BitmapPropertySet> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>> {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapPropertySet) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> for BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> for &BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<BitmapPropertySet> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: BitmapPropertySet) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>> for BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>> for &BitmapPropertySet {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for BitmapPropertySet {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for BitmapPropertySet {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapRotation {}
impl ::core::clone::Clone for BitmapRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BitmapRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapRotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct BitmapSize {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapSize {}
impl ::core::clone::Clone for BitmapSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapSize").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows_core::Abi for BitmapSize {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BitmapSize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapSize;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BitmapSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BitmapSize>()) == 0 }
    }
}
impl ::core::cmp::Eq for BitmapSize {}
impl ::core::default::Default for BitmapSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct BitmapTransform(::windows_core::IUnknown);
impl BitmapTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapTransform, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ScaledWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ScaledWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetScaledWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaledWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaledHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ScaledHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetScaledHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaledHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InterpolationMode(&self) -> ::windows_core::Result<BitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapInterpolationMode>::zeroed();
            (::windows_core::Interface::vtable(this).InterpolationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapInterpolationMode>(result__)
        }
    }
    pub fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterpolationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Flip(&self) -> ::windows_core::Result<BitmapFlip> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapFlip>::zeroed();
            (::windows_core::Interface::vtable(this).Flip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapFlip>(result__)
        }
    }
    pub fn SetFlip(&self, value: BitmapFlip) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlip)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<BitmapRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapRotation>::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapRotation>(result__)
        }
    }
    pub fn SetRotation(&self, value: BitmapRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> ::windows_core::Result<BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapBounds>::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapBounds>(result__)
        }
    }
    pub fn SetBounds<'a, Param0: ::windows_core::IntoParam<'a, BitmapBounds>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBounds)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for BitmapTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapTransform {}
impl ::core::fmt::Debug for BitmapTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTransform;{ae755344-e268-4d35-adcf-e995d31a8d34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTransform";
}
impl ::core::convert::From<BitmapTransform> for ::windows_core::IUnknown {
    fn from(value: BitmapTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapTransform> for ::windows_core::IUnknown {
    fn from(value: &BitmapTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapTransform> for ::windows_core::IInspectable {
    fn from(value: BitmapTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapTransform> for ::windows_core::IInspectable {
    fn from(value: &BitmapTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BitmapTransform {}
unsafe impl ::core::marker::Sync for BitmapTransform {}
#[repr(transparent)]
pub struct BitmapTypedValue(::windows_core::IUnknown);
impl BitmapTypedValue {
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::winrt_foundation::PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::PropertyType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::PropertyType>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(value: Param0, r#type: ::winrt_foundation::PropertyType) -> ::windows_core::Result<BitmapTypedValue> {
        Self::IBitmapTypedValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value.into_param().abi(), r#type, result__.as_mut_ptr()).from_abi::<BitmapTypedValue>(result__)
        })
    }
    pub fn IBitmapTypedValueFactory<R, F: FnOnce(&IBitmapTypedValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BitmapTypedValue, IBitmapTypedValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BitmapTypedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BitmapTypedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapTypedValue {}
impl ::core::fmt::Debug for BitmapTypedValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapTypedValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BitmapTypedValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
    const IID: ::windows_core::GUID = <IBitmapTypedValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTypedValue";
}
impl ::core::convert::From<BitmapTypedValue> for ::windows_core::IUnknown {
    fn from(value: BitmapTypedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapTypedValue> for ::windows_core::IUnknown {
    fn from(value: &BitmapTypedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BitmapTypedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BitmapTypedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BitmapTypedValue> for ::windows_core::IInspectable {
    fn from(value: BitmapTypedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapTypedValue> for ::windows_core::IInspectable {
    fn from(value: &BitmapTypedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BitmapTypedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BitmapTypedValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BitmapTypedValue {}
unsafe impl ::core::marker::Sync for BitmapTypedValue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: Self = Self(0i32);
    pub const ColorManageToSRgb: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorManagementMode {}
impl ::core::clone::Clone for ColorManagementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorManagementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ColorManagementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ColorManagementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorManagementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorManagementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ColorManagementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: Self = Self(0i32);
    pub const RespectExifOrientation: Self = Self(1i32);
}
impl ::core::marker::Copy for ExifOrientationMode {}
impl ::core::clone::Clone for ExifOrientationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExifOrientationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExifOrientationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExifOrientationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExifOrientationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExifOrientationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ExifOrientationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapBuffer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPlaneCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetPlaneDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapCodecInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCodecInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CodecId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FileExtensions: usize,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MimeTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MimeTypes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacef22ba_1d74_4c91_9dfc_9620745233e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DecoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetPreviewAsync: usize,
    pub GetFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameindex: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoderStatics {
    type Vtable = IBitmapDecoderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x438ccb26_bcef_4e95_bad6_23a822e58d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BmpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PngDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TiffDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegXRDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IcoDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetDecoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetDecoderInformationEnumerator: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateWithIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, decoderid: ::windows_core::GUID, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateWithIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoderStatics2 {
    type Vtable = IBitmapDecoderStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50ba68ea_99a1_40c4_80d9_aef0dafa6c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub WebpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EncoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetGeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub GeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetGeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BitmapTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows_core::HRESULT,
    pub GoToNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GoToNextFrameWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GoToNextFrameWithEncodingOptionsAsync: usize,
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderStatics {
    type Vtable = IBitmapEncoderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74356a7_a4e4_4eb9_8e40_564de7e1ccb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BmpEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PngEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TiffEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegXREncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetEncoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetEncoderInformationEnumerator: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows_core::GUID, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub CreateWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows_core::GUID, stream: ::windows_core::RawPtr, encodingoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    CreateWithEncodingOptionsAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateForTranscodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, bitmapdecoder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateForTranscodingAsync: usize,
    pub CreateForInPlacePropertyEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapdecoder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderStatics2 {
    type Vtable = IBitmapEncoderStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33cbc259_fe31_41b1_b812_086d21e87e16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderWithSoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderWithSoftwareBitmap {
    type Vtable = IBitmapEncoderWithSoftwareBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x686cd241_4330_4c77_ace4_0334968b1768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBitmapFrame(::windows_core::IUnknown);
impl IBitmapFrame {
    #[cfg(feature = "winrt-storage")]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
}
impl ::core::convert::From<IBitmapFrame> for ::windows_core::IUnknown {
    fn from(value: IBitmapFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapFrame> for ::windows_core::IUnknown {
    fn from(value: &IBitmapFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBitmapFrame> for ::windows_core::IInspectable {
    fn from(value: IBitmapFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapFrame> for ::windows_core::IInspectable {
    fn from(value: &IBitmapFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBitmapFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitmapFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapFrame {}
impl ::core::fmt::Debug for IBitmapFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBitmapFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{72a49a1c-8081-438d-91bc-94ecfc8185c6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetThumbnailAsync: usize,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OrientedPixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OrientedPixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetPixelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPixelDataTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows_core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBitmapFrameWithSoftwareBitmap(::windows_core::IUnknown);
impl IBitmapFrameWithSoftwareBitmap {
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows_core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
}
impl ::core::convert::From<IBitmapFrameWithSoftwareBitmap> for ::windows_core::IUnknown {
    fn from(value: IBitmapFrameWithSoftwareBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapFrameWithSoftwareBitmap> for ::windows_core::IUnknown {
    fn from(value: &IBitmapFrameWithSoftwareBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBitmapFrameWithSoftwareBitmap> for ::windows_core::IInspectable {
    fn from(value: IBitmapFrameWithSoftwareBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapFrameWithSoftwareBitmap> for ::windows_core::IInspectable {
    fn from(value: &IBitmapFrameWithSoftwareBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBitmapFrameWithSoftwareBitmap> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: IBitmapFrameWithSoftwareBitmap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBitmapFrameWithSoftwareBitmap> for IBitmapFrame {
    type Error = ::windows_core::Error;
    fn try_from(value: &IBitmapFrameWithSoftwareBitmap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBitmapFrame> for &IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, IBitmapFrame> {
        ::core::convert::TryInto::<IBitmapFrame>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IBitmapFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitmapFrameWithSoftwareBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapFrameWithSoftwareBitmap {}
impl ::core::fmt::Debug for IBitmapFrameWithSoftwareBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapFrameWithSoftwareBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBitmapFrameWithSoftwareBitmap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fe287c9a-420c-4963-87ad-691436e08383}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBitmapFrameWithSoftwareBitmap {
    type Vtable = IBitmapFrameWithSoftwareBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe287c9a_420c_4963_87ad_691436e08383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetSoftwareBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSoftwareBitmapConvertedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSoftwareBitmapTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows_core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoset: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPropertiesAsync: usize,
}
#[repr(transparent)]
pub struct IBitmapPropertiesView(::windows_core::IUnknown);
impl IBitmapPropertiesView {
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
impl ::core::convert::From<IBitmapPropertiesView> for ::windows_core::IUnknown {
    fn from(value: IBitmapPropertiesView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapPropertiesView> for ::windows_core::IUnknown {
    fn from(value: &IBitmapPropertiesView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBitmapPropertiesView> for ::windows_core::IInspectable {
    fn from(value: IBitmapPropertiesView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapPropertiesView> for ::windows_core::IInspectable {
    fn from(value: &IBitmapPropertiesView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBitmapPropertiesView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitmapPropertiesView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapPropertiesView {}
impl ::core::fmt::Debug for IBitmapPropertiesView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapPropertiesView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBitmapPropertiesView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{7e0fe87a-3a70-48f8-9c55-196cf5a545f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapPropertiesView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoretrieve: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae755344_e268_4d35_adcf_e995d31a8d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub InterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapFlip) -> ::windows_core::HRESULT,
    pub SetFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapFlip) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapRotation) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapRotation) -> ::windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapBounds) -> ::windows_core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapBounds) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd8044a9_2443_4000_b0cd_79316c56f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::PropertyType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTypedValueFactory {
    type Vtable = IBitmapTypedValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92dbb599_ce13_46bb_9545_cb3a3f63eb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: ::winrt_foundation::PropertyType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPixelDataProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelDataProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DetachPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x689e0708_7eef_483f_963f_da938818e073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: BitmapBufferAccessMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CopyFromBuffer: usize,
    #[cfg(feature = "winrt-storage")]
    pub CopyToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapFactory {
    type Vtable = ISoftwareBitmapFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapStatics {
    type Vtable = ISoftwareBitmapStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf0385db_672f_4a9d_806e_c2442f343e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Convert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, format: BitmapPixelFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConvertWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateCopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateCopyFromBuffer: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateCopyWithAlphaFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateCopyWithAlphaFromBuffer: usize,
    #[cfg(feature = "winrt-graphics")]
    pub CreateCopyFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CreateCopyFromSurfaceAsync: usize,
    #[cfg(feature = "winrt-graphics")]
    pub CreateCopyWithAlphaFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows_core::RawPtr, alpha: BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    CreateCopyWithAlphaFromSurfaceAsync: usize,
}
#[cfg(feature = "winrt-storage")]
#[repr(transparent)]
pub struct ImageStream(::windows_core::IUnknown);
#[cfg(feature = "winrt-storage")]
impl ImageStream {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: ::winrt_storage::Streams::InputStreamOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), count, options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn WriteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CloneStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::clone::Clone for ImageStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::cmp::PartialEq for ImageStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::cmp::Eq for ImageStream {}
#[cfg(feature = "winrt-storage")]
impl ::core::fmt::Debug for ImageStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageStream").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::windows_core::RuntimeType for ImageStream {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.ImageStream;{cc254827-4b3d-438f-9232-10c76bc7e038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::windows_core::Interface for ImageStream {
    type Vtable = ::winrt_storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_storage::Streams::IRandomAccessStreamWithContentType as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-storage")]
impl ::windows_core::RuntimeName for ImageStream {
    const NAME: &'static str = "Windows.Graphics.Imaging.ImageStream";
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<ImageStream> for ::windows_core::IUnknown {
    fn from(value: ImageStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<&ImageStream> for ::windows_core::IUnknown {
    fn from(value: &ImageStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<ImageStream> for ::windows_core::IInspectable {
    fn from(value: ImageStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::From<&ImageStream> for ::windows_core::IInspectable {
    fn from(value: &ImageStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IContentTypeProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IInputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IOutputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<ImageStream> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageStream) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-storage")]
impl ::core::convert::TryFrom<&ImageStream> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageStream) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-storage")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for &ImageStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-storage")]
unsafe impl ::core::marker::Send for ImageStream {}
#[cfg(feature = "winrt-storage")]
unsafe impl ::core::marker::Sync for ImageStream {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: Self = Self(0i32);
    pub const Y4Cb2Cr0: Self = Self(1i32);
    pub const Y4Cb2Cr2: Self = Self(2i32);
    pub const Y4Cb4Cr4: Self = Self(3i32);
}
impl ::core::marker::Copy for JpegSubsamplingMode {}
impl ::core::clone::Clone for JpegSubsamplingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JpegSubsamplingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for JpegSubsamplingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for JpegSubsamplingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JpegSubsamplingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JpegSubsamplingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.JpegSubsamplingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PixelDataProvider(::windows_core::IUnknown);
impl PixelDataProvider {
    pub fn DetachPixelData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).DetachPixelData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for PixelDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PixelDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PixelDataProvider {}
impl ::core::fmt::Debug for PixelDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PixelDataProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PixelDataProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.PixelDataProvider;{dd831f25-185c-4595-9fb9-ccbe6ec18a6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
    const IID: ::windows_core::GUID = <IPixelDataProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.PixelDataProvider";
}
impl ::core::convert::From<PixelDataProvider> for ::windows_core::IUnknown {
    fn from(value: PixelDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PixelDataProvider> for ::windows_core::IUnknown {
    fn from(value: &PixelDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PixelDataProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PixelDataProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PixelDataProvider> for ::windows_core::IInspectable {
    fn from(value: PixelDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PixelDataProvider> for ::windows_core::IInspectable {
    fn from(value: &PixelDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PixelDataProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PixelDataProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PixelDataProvider {}
unsafe impl ::core::marker::Sync for PixelDataProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Sub: Self = Self(2i32);
    pub const Up: Self = Self(3i32);
    pub const Average: Self = Self(4i32);
    pub const Paeth: Self = Self(5i32);
    pub const Adaptive: Self = Self(6i32);
}
impl ::core::marker::Copy for PngFilterMode {}
impl ::core::clone::Clone for PngFilterMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PngFilterMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PngFilterMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PngFilterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PngFilterMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PngFilterMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.PngFilterMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SoftwareBitmap(::windows_core::IUnknown);
impl SoftwareBitmap {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BitmapAlphaMode>(result__)
        }
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
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDpiX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDpiY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows_core::Result<BitmapBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LockBuffer)(::windows_core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<BitmapBuffer>(result__)
        }
    }
    pub fn CopyTo<'a, Param0: ::windows_core::IntoParam<'a, SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyTo)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CopyFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyFromBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CopyToBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyToBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> ::windows_core::Result<SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReadOnlyView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        }
    }
    pub fn Create(format: BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), format, width, height, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn CreateWithAlpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAlpha)(::windows_core::Interface::as_raw(this), format, width, height, alpha, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn Copy<'a, Param0: ::windows_core::IntoParam<'a, SoftwareBitmap>>(source: Param0) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn Convert<'a, Param0: ::windows_core::IntoParam<'a, SoftwareBitmap>>(source: Param0, format: BitmapPixelFormat) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Convert)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn ConvertWithAlpha<'a, Param0: ::windows_core::IntoParam<'a, SoftwareBitmap>>(source: Param0, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertWithAlpha)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, alpha, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateCopyFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(source: Param0, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyFromBuffer)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, width, height, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateCopyWithAlphaFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(source: Param0, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyWithAlphaFromBuffer)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, width, height, alpha, result__.as_mut_ptr()).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CreateCopyFromSurfaceAsync<'a, Param0: ::windows_core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyFromSurfaceAsync)(::windows_core::Interface::as_raw(this), surface.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        })
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn CreateCopyWithAlphaFromSurfaceAsync<'a, Param0: ::windows_core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0, alpha: BitmapAlphaMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SoftwareBitmap>> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyWithAlphaFromSurfaceAsync)(::windows_core::Interface::as_raw(this), surface.into_param().abi(), alpha, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        })
    }
    pub fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SoftwareBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SoftwareBitmap {}
impl ::core::fmt::Debug for SoftwareBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoftwareBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.SoftwareBitmap;{689e0708-7eef-483f-963f-da938818e073})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
    const IID: ::windows_core::GUID = <ISoftwareBitmap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
impl ::core::convert::From<SoftwareBitmap> for ::windows_core::IUnknown {
    fn from(value: SoftwareBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SoftwareBitmap> for ::windows_core::IUnknown {
    fn from(value: &SoftwareBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SoftwareBitmap> for ::windows_core::IInspectable {
    fn from(value: SoftwareBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SoftwareBitmap> for ::windows_core::IInspectable {
    fn from(value: &SoftwareBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SoftwareBitmap> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SoftwareBitmap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SoftwareBitmap> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SoftwareBitmap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SoftwareBitmap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SoftwareBitmap {}
unsafe impl ::core::marker::Sync for SoftwareBitmap {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Ccitt3: Self = Self(2i32);
    pub const Ccitt4: Self = Self(3i32);
    pub const Lzw: Self = Self(4i32);
    pub const Rle: Self = Self(5i32);
    pub const Zip: Self = Self(6i32);
    pub const LzwhDifferencing: Self = Self(7i32);
}
impl ::core::marker::Copy for TiffCompressionMode {}
impl ::core::clone::Clone for TiffCompressionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TiffCompressionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TiffCompressionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TiffCompressionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TiffCompressionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TiffCompressionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.TiffCompressionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
