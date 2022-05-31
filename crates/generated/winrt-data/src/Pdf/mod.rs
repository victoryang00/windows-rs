#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfDocument {
    type Vtable = IPdfDocument_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac7ebedd_80fa_4089_846e_81b77ff5a86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfDocument_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsPasswordProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfDocumentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfDocumentStatics {
    type Vtable = IPdfDocumentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x433a0b5f_c007_4788_90f2_08143d922599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfDocumentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromFileAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromFileWithPasswordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, password: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromFileWithPasswordAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromStreamAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub LoadFromStreamWithPasswordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: ::windows_core::RawPtr, password: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadFromStreamWithPasswordAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfPage {
    type Vtable = IPdfPage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9db4b0c8_5320_4cfc_ad76_493fdad0e594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub RenderToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    RenderToStreamAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub RenderWithOptionsToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    RenderWithOptionsToStreamAsync: usize,
    pub PreparePageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub Dimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PdfPageRotation) -> ::windows_core::HRESULT,
    pub PreferredZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPageDimensions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfPageDimensions {
    type Vtable = IPdfPageDimensions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22170471_313e_44e8_835d_63a3e7624a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPageDimensions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MediaBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub CropBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub BleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub TrimBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub ArtBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPageRenderOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfPageRenderOptions {
    type Vtable = IPdfPageRenderOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c98056f_b7cf_4c29_9a04_52d90267f425);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPageRenderOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub DestinationWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDestinationWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DestinationHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDestinationHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    BackgroundColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetBackgroundColor: usize,
    pub IsIgnoringHighContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsIgnoringHighContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BitmapEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetBitmapEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PdfDocument(::windows_core::IUnknown);
impl PdfDocument {
    pub fn GetPage(&self, pageindex: u32) -> ::windows_core::Result<PdfPage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPage)(::windows_core::Interface::as_raw(this), pageindex, result__.as_mut_ptr()).from_abi::<PdfPage>(result__)
        }
    }
    pub fn PageCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PageCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn IsPasswordProtected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPasswordProtected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromFileWithPasswordAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(file: Param0, password: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFileWithPasswordAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), password.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(inputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStreamAsync)(::windows_core::Interface::as_raw(this), inputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadFromStreamWithPasswordAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(inputstream: Param0, password: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStreamWithPasswordAsync)(::windows_core::Interface::as_raw(this), inputstream.into_param().abi(), password.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    pub fn IPdfDocumentStatics<R, F: FnOnce(&IPdfDocumentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PdfDocument, IPdfDocumentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PdfDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfDocument {}
impl ::core::fmt::Debug for PdfDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PdfDocument {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfDocument;{ac7ebedd-80fa-4089-846e-81b77ff5a86c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PdfDocument {
    type Vtable = IPdfDocument_Vtbl;
    const IID: ::windows_core::GUID = <IPdfDocument as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PdfDocument {
    const NAME: &'static str = "Windows.Data.Pdf.PdfDocument";
}
impl ::core::convert::From<PdfDocument> for ::windows_core::IUnknown {
    fn from(value: PdfDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfDocument> for ::windows_core::IUnknown {
    fn from(value: &PdfDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PdfDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PdfDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfDocument> for ::windows_core::IInspectable {
    fn from(value: PdfDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfDocument> for ::windows_core::IInspectable {
    fn from(value: &PdfDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PdfDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PdfDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfDocument {}
unsafe impl ::core::marker::Sync for PdfDocument {}
#[repr(transparent)]
pub struct PdfPage(::windows_core::IUnknown);
impl PdfPage {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn RenderToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenderToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn RenderWithOptionsToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>, Param1: ::windows_core::IntoParam<'a, PdfPageRenderOptions>>(&self, outputstream: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenderWithOptionsToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn PreparePageAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreparePageAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn Index(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Index)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn Dimensions(&self) -> ::windows_core::Result<PdfPageDimensions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dimensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PdfPageDimensions>(result__)
        }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<PdfPageRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PdfPageRotation>::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PdfPageRotation>(result__)
        }
    }
    pub fn PreferredZoom(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredZoom)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for PdfPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPage {}
impl ::core::fmt::Debug for PdfPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PdfPage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPage;{9db4b0c8-5320-4cfc-ad76-493fdad0e594})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PdfPage {
    type Vtable = IPdfPage_Vtbl;
    const IID: ::windows_core::GUID = <IPdfPage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PdfPage {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPage";
}
impl ::core::convert::From<PdfPage> for ::windows_core::IUnknown {
    fn from(value: PdfPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPage> for ::windows_core::IUnknown {
    fn from(value: &PdfPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPage> for ::windows_core::IInspectable {
    fn from(value: PdfPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPage> for ::windows_core::IInspectable {
    fn from(value: &PdfPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PdfPage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PdfPage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PdfPage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PdfPage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PdfPage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PdfPage {}
unsafe impl ::core::marker::Sync for PdfPage {}
#[repr(transparent)]
pub struct PdfPageDimensions(::windows_core::IUnknown);
impl PdfPageDimensions {
    pub fn MediaBox(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).MediaBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CropBox(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).CropBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn BleedBox(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BleedBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn TrimBox(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).TrimBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn ArtBox(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ArtBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for PdfPageDimensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPageDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageDimensions {}
impl ::core::fmt::Debug for PdfPageDimensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageDimensions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PdfPageDimensions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPageDimensions;{22170471-313e-44e8-835d-63a3e7624a10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PdfPageDimensions {
    type Vtable = IPdfPageDimensions_Vtbl;
    const IID: ::windows_core::GUID = <IPdfPageDimensions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PdfPageDimensions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageDimensions";
}
impl ::core::convert::From<PdfPageDimensions> for ::windows_core::IUnknown {
    fn from(value: PdfPageDimensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageDimensions> for ::windows_core::IUnknown {
    fn from(value: &PdfPageDimensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PdfPageDimensions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PdfPageDimensions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPageDimensions> for ::windows_core::IInspectable {
    fn from(value: PdfPageDimensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageDimensions> for ::windows_core::IInspectable {
    fn from(value: &PdfPageDimensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PdfPageDimensions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PdfPageDimensions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfPageDimensions {}
unsafe impl ::core::marker::Sync for PdfPageDimensions {}
#[repr(transparent)]
pub struct PdfPageRenderOptions(::windows_core::IUnknown);
impl PdfPageRenderOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PdfPageRenderOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SourceRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).SourceRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetSourceRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DestinationWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DestinationWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDestinationWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDestinationWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DestinationHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DestinationHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetDestinationHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDestinationHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsIgnoringHighContrast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIgnoringHighContrast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsIgnoringHighContrast(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsIgnoringHighContrast)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitmapEncoderId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapEncoderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetBitmapEncoderId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapEncoderId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PdfPageRenderOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPageRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageRenderOptions {}
impl ::core::fmt::Debug for PdfPageRenderOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRenderOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PdfPageRenderOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPageRenderOptions;{3c98056f-b7cf-4c29-9a04-52d90267f425})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PdfPageRenderOptions {
    type Vtable = IPdfPageRenderOptions_Vtbl;
    const IID: ::windows_core::GUID = <IPdfPageRenderOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PdfPageRenderOptions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageRenderOptions";
}
impl ::core::convert::From<PdfPageRenderOptions> for ::windows_core::IUnknown {
    fn from(value: PdfPageRenderOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageRenderOptions> for ::windows_core::IUnknown {
    fn from(value: &PdfPageRenderOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PdfPageRenderOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PdfPageRenderOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPageRenderOptions> for ::windows_core::IInspectable {
    fn from(value: PdfPageRenderOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageRenderOptions> for ::windows_core::IInspectable {
    fn from(value: &PdfPageRenderOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PdfPageRenderOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PdfPageRenderOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfPageRenderOptions {}
unsafe impl ::core::marker::Sync for PdfPageRenderOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PdfPageRotation(pub i32);
impl PdfPageRotation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(1i32);
    pub const Rotate180: Self = Self(2i32);
    pub const Rotate270: Self = Self(3i32);
}
impl ::core::marker::Copy for PdfPageRotation {}
impl ::core::clone::Clone for PdfPageRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PdfPageRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PdfPageRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PdfPageRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PdfPageRotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Data.Pdf.PdfPageRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
