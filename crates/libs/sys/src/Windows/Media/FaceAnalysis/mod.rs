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
impl ::windows_sys::core::Interface for IDetectedFace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2181092436, data2: 26300, data3: 13535, data4: [148, 16, 232, 148, 0, 25, 84, 20] };
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
impl ::windows_sys::core::Interface for IFaceDetector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 381055708, data2: 65135, data3: 12567, data4: [141, 149, 195, 240, 77, 81, 99, 12] };
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
impl ::windows_sys::core::Interface for IFaceDetectorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3154390375, data2: 36935, data3: 13302, data4: [136, 27, 103, 70, 193, 178, 24, 184] };
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
impl ::windows_sys::core::Interface for IFaceTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1806073228, data2: 43073, data3: 17440, data4: [147, 230, 36, 32, 161, 136, 79, 207] };
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
impl ::windows_sys::core::Interface for IFaceTrackerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3915551128, data2: 6145, data3: 16293, data4: [147, 46, 49, 215, 103, 175, 108, 77] };
}
