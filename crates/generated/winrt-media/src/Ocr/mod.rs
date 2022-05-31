#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrEngine {
    type Vtable = IOcrEngine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a14bc41_5b76_3140_b680_8825562683ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngine_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Globalization")]
    pub RecognizerLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    RecognizerLanguage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngineStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrEngineStatics {
    type Vtable = IOcrEngineStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bffa85a_3384_3540_9940_699120d428a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngineStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxImageDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub AvailableRecognizerLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    AvailableRecognizerLanguages: usize,
    #[cfg(feature = "Globalization")]
    pub IsLanguageSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    IsLanguageSupported: usize,
    #[cfg(feature = "Globalization")]
    pub TryCreateFromLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    TryCreateFromLanguage: usize,
    pub TryCreateFromUserProfileLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrLine {
    type Vtable = IOcrLine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0043a16f_e31f_3a24_899c_d444bd088124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrLine_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Words: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Words: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrResult {
    type Vtable = IOcrResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bd235b2_175b_3d6a_92e2_388c206e2f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
    pub TextAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrWord(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrWord {
    type Vtable = IOcrWord_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c2a477a_5cd9_3525_ba2a_23d1e0a68a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrWord_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct OcrEngine(::windows_core::IUnknown);
impl OcrEngine {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn RecognizeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<OcrResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizeAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<OcrResult>>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn RecognizerLanguage(&self) -> ::windows_core::Result<::winrt_globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizerLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_globalization::Language>(result__)
        }
    }
    pub fn MaxImageDimension() -> ::windows_core::Result<u32> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxImageDimension)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn AvailableRecognizerLanguages() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_globalization::Language>> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableRecognizerLanguages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_globalization::Language>>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn IsLanguageSupported<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(language: Param0) -> ::windows_core::Result<bool> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLanguageSupported)(::windows_core::Interface::as_raw(this), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn TryCreateFromLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(language: Param0) -> ::windows_core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromLanguage)(::windows_core::Interface::as_raw(this), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<OcrEngine>(result__)
        })
    }
    pub fn TryCreateFromUserProfileLanguages() -> ::windows_core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromUserProfileLanguages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OcrEngine>(result__)
        })
    }
    pub fn IOcrEngineStatics<R, F: FnOnce(&IOcrEngineStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OcrEngine, IOcrEngineStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OcrEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrEngine {}
impl ::core::fmt::Debug for OcrEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OcrEngine {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrEngine;{5a14bc41-5b76-3140-b680-8825562683ac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OcrEngine {
    type Vtable = IOcrEngine_Vtbl;
    const IID: ::windows_core::GUID = <IOcrEngine as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrEngine";
}
impl ::core::convert::From<OcrEngine> for ::windows_core::IUnknown {
    fn from(value: OcrEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrEngine> for ::windows_core::IUnknown {
    fn from(value: &OcrEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OcrEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OcrEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrEngine> for ::windows_core::IInspectable {
    fn from(value: OcrEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrEngine> for ::windows_core::IInspectable {
    fn from(value: &OcrEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OcrEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OcrEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrEngine {}
unsafe impl ::core::marker::Sync for OcrEngine {}
#[repr(transparent)]
pub struct OcrLine(::windows_core::IUnknown);
impl OcrLine {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Words(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<OcrWord>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Words)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<OcrWord>>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrLine {}
impl ::core::fmt::Debug for OcrLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrLine").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OcrLine {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrLine;{0043a16f-e31f-3a24-899c-d444bd088124})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OcrLine {
    type Vtable = IOcrLine_Vtbl;
    const IID: ::windows_core::GUID = <IOcrLine as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrLine";
}
impl ::core::convert::From<OcrLine> for ::windows_core::IUnknown {
    fn from(value: OcrLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrLine> for ::windows_core::IUnknown {
    fn from(value: &OcrLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OcrLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OcrLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrLine> for ::windows_core::IInspectable {
    fn from(value: OcrLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrLine> for ::windows_core::IInspectable {
    fn from(value: &OcrLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OcrLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OcrLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrLine {}
unsafe impl ::core::marker::Sync for OcrLine {}
#[repr(transparent)]
pub struct OcrResult(::windows_core::IUnknown);
impl OcrResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<OcrLine>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<OcrLine>>(result__)
        }
    }
    pub fn TextAngle(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrResult {}
impl ::core::fmt::Debug for OcrResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OcrResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrResult;{9bd235b2-175b-3d6a-92e2-388c206e2f63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OcrResult {
    type Vtable = IOcrResult_Vtbl;
    const IID: ::windows_core::GUID = <IOcrResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.OcrResult";
}
impl ::core::convert::From<OcrResult> for ::windows_core::IUnknown {
    fn from(value: OcrResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrResult> for ::windows_core::IUnknown {
    fn from(value: &OcrResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OcrResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OcrResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrResult> for ::windows_core::IInspectable {
    fn from(value: OcrResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrResult> for ::windows_core::IInspectable {
    fn from(value: &OcrResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OcrResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OcrResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrResult {}
unsafe impl ::core::marker::Sync for OcrResult {}
#[repr(transparent)]
pub struct OcrWord(::windows_core::IUnknown);
impl OcrWord {
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrWord {}
impl ::core::fmt::Debug for OcrWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrWord").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OcrWord {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrWord;{3c2a477a-5cd9-3525-ba2a-23d1e0a68a1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OcrWord {
    type Vtable = IOcrWord_Vtbl;
    const IID: ::windows_core::GUID = <IOcrWord as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.OcrWord";
}
impl ::core::convert::From<OcrWord> for ::windows_core::IUnknown {
    fn from(value: OcrWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrWord> for ::windows_core::IUnknown {
    fn from(value: &OcrWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OcrWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OcrWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrWord> for ::windows_core::IInspectable {
    fn from(value: OcrWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrWord> for ::windows_core::IInspectable {
    fn from(value: &OcrWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OcrWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OcrWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrWord {}
unsafe impl ::core::marker::Sync for OcrWord {}
