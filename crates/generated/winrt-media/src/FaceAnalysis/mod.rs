#[repr(transparent)]
pub struct DetectedFace(::windows_core::IUnknown);
impl DetectedFace {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn FaceBox(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapBounds>::zeroed();
            (::windows_core::Interface::vtable(this).FaceBox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapBounds>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectedFace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectedFace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectedFace {}
impl ::core::fmt::Debug for DetectedFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectedFace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DetectedFace {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.DetectedFace;{8200d454-66bc-34df-9410-e89400195414})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DetectedFace {
    type Vtable = IDetectedFace_Vtbl;
    const IID: ::windows_core::GUID = <IDetectedFace as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.DetectedFace";
}
impl ::core::convert::From<DetectedFace> for ::windows_core::IUnknown {
    fn from(value: DetectedFace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows_core::IUnknown {
    fn from(value: &DetectedFace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DetectedFace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DetectedFace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectedFace> for ::windows_core::IInspectable {
    fn from(value: DetectedFace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows_core::IInspectable {
    fn from(value: &DetectedFace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DetectedFace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DetectedFace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectedFace {}
unsafe impl ::core::marker::Sync for DetectedFace {}
#[repr(transparent)]
pub struct FaceDetector(::windows_core::IUnknown);
impl FaceDetector {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>>(&self, image: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DetectFacesAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn DetectFacesWithSearchAreaAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::SoftwareBitmap>, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapBounds>>(&self, image: Param0, searcharea: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DetectFacesWithSearchAreaAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), searcharea.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapSize>::zeroed();
            (::windows_core::Interface::vtable(this).MinDetectableFaceSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinDetectableFaceSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapSize>::zeroed();
            (::windows_core::Interface::vtable(this).MaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FaceDetector>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FaceDetector>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::windows_core::Interface::as_raw(this), bitmappixelformat, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IFaceDetectorStatics<R, F: FnOnce(&IFaceDetectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FaceDetector, IFaceDetectorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FaceDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetector {}
impl ::core::fmt::Debug for FaceDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceDetector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceDetector;{16b672dc-fe6f-3117-8d95-c3f04d51630c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FaceDetector {
    type Vtable = IFaceDetector_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceDetector";
}
impl ::core::convert::From<FaceDetector> for ::windows_core::IUnknown {
    fn from(value: FaceDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows_core::IUnknown {
    fn from(value: &FaceDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetector> for ::windows_core::IInspectable {
    fn from(value: FaceDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows_core::IInspectable {
    fn from(value: &FaceDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceDetector {}
unsafe impl ::core::marker::Sync for FaceDetector {}
#[repr(transparent)]
pub struct FaceTracker(::windows_core::IUnknown);
impl FaceTracker {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessNextFrameAsync<'a, Param0: ::windows_core::IntoParam<'a, super::VideoFrame>>(&self, videoframe: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessNextFrameAsync)(::windows_core::Interface::as_raw(this), videoframe.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MinDetectableFaceSize(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapSize>::zeroed();
            (::windows_core::Interface::vtable(this).MinDetectableFaceSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinDetectableFaceSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn MaxDetectableFaceSize(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapSize>::zeroed();
            (::windows_core::Interface::vtable(this).MaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDetectableFaceSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FaceTracker>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FaceTracker>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBitmapPixelFormatSupported)(::windows_core::Interface::as_raw(this), bitmappixelformat, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IFaceTrackerStatics<R, F: FnOnce(&IFaceTrackerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FaceTracker, IFaceTrackerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FaceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceTracker {}
impl ::core::fmt::Debug for FaceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FaceTracker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceTracker;{6ba67d8c-a841-4420-93e6-2420a1884fcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FaceTracker {
    type Vtable = IFaceTracker_Vtbl;
    const IID: ::windows_core::GUID = <IFaceTracker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceTracker";
}
impl ::core::convert::From<FaceTracker> for ::windows_core::IUnknown {
    fn from(value: FaceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows_core::IUnknown {
    fn from(value: &FaceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FaceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FaceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceTracker> for ::windows_core::IInspectable {
    fn from(value: FaceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows_core::IInspectable {
    fn from(value: &FaceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FaceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FaceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceTracker {}
unsafe impl ::core::marker::Sync for FaceTracker {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectedFace(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDetectedFace {
    type Vtable = IDetectedFace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8200d454_66bc_34df_9410_e89400195414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectedFace_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub FaceBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapBounds) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    FaceBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetector {
    type Vtable = IFaceDetector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16b672dc_fe6f_3117_8d95_c3f04d51630c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesWithSearchAreaAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows_core::RawPtr, searcharea: ::winrt_graphics::Imaging::BitmapBounds, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesWithSearchAreaAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectorStatics {
    type Vtable = IFaceDetectorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc042d67_9047_33f6_881b_6746c1b218b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: ::winrt_graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceTracker {
    type Vtable = IFaceTracker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ba67d8c_a841_4420_93e6_2420a1884fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTracker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videoframe: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessNextFrameAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceTrackerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceTrackerStatics {
    type Vtable = IFaceTrackerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9629198_1801_3fa5_932e_31d767af6c4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTrackerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmappixelformat: ::winrt_graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
