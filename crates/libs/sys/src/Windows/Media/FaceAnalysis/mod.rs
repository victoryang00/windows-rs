pub type DetectedFace = *mut ::core::ffi::c_void;
pub type FaceDetector = *mut ::core::ffi::c_void;
pub type FaceTracker = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDetectedFace {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub FaceBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    FaceBox: usize,
}
#[repr(C)]
pub struct IFaceDetector {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesAsync: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub DetectFacesWithSearchAreaAsync: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    DetectFacesWithSearchAreaAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[repr(C)]
pub struct IFaceDetectorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFaceTracker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessNextFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, videoframe: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessNextFrameAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MinDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMinDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMinDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub MaxDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    MaxDetectableFaceSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetMaxDetectableFaceSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetMaxDetectableFaceSize: usize,
}
#[repr(C)]
pub struct IFaceTrackerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub GetSupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    GetSupportedBitmapPixelFormats: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub IsBitmapPixelFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    IsBitmapPixelFormatSupported: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
