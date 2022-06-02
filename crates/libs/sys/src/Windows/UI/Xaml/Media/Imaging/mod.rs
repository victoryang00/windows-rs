#[doc = "*Required features: `\"UI_Xaml_Media_Imaging\"`*"]
#[repr(transparent)]
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
pub type BitmapImage = *mut ::core::ffi::c_void;
pub type BitmapSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Imaging\"`*"]
#[repr(transparent)]
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
pub type DownloadProgressEventArgs = *mut ::core::ffi::c_void;
pub type DownloadProgressEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBitmapImage {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapCreateOptions) -> ::windows_sys::core::HRESULT,
    pub SetCreateOptions: unsafe extern "system" fn(this: *mut *mut Self, value: BitmapCreateOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UriSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Foundation")]
    pub SetUriSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUriSource: usize,
    pub DecodePixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDecodePixelWidth: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub DecodePixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDecodePixelHeight: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProgress: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadProgress: usize,
    #[cfg(feature = "Foundation")]
    pub ImageOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageOpened: usize,
    #[cfg(feature = "Foundation")]
    pub ImageFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageFailed: usize,
}
#[repr(C)]
pub struct IBitmapImage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecodePixelType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DecodePixelType) -> ::windows_sys::core::HRESULT,
    pub SetDecodePixelType: unsafe extern "system" fn(this: *mut *mut Self, value: DecodePixelType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapImage3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAnimatedBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlaying: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapImageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(this: *mut *mut Self, urisource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithUriSource: usize,
}
#[repr(C)]
pub struct IBitmapImageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateOptionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DecodePixelWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DecodePixelHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapImageStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecodePixelTypeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapImageStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAnimatedBitmapProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPlayingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, streamsource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSource: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetSourceAsync: unsafe extern "system" fn(this: *mut *mut Self, streamsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetSourceAsync: usize,
}
#[repr(C)]
pub struct IBitmapSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBitmapSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDownloadProgressEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRenderTargetBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RenderAsync: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RenderToSizeAsync: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, scaledwidth: i32, scaledheight: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderToSizeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPixelsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPixelsAsync: usize,
}
#[repr(C)]
pub struct IRenderTargetBitmapStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISoftwareBitmapSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetBitmapAsync: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetBitmapAsync: usize,
}
#[repr(C)]
pub struct ISurfaceImageSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISurfaceImageSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut *mut Self, pixelwidth: i32, pixelheight: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity: unsafe extern "system" fn(this: *mut *mut Self, pixelwidth: i32, pixelheight: i32, isopaque: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISvgImageSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UriSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Foundation")]
    pub SetUriSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUriSource: usize,
    pub RasterizePixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRasterizePixelWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RasterizePixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRasterizePixelHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub OpenFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetSourceAsync: unsafe extern "system" fn(this: *mut *mut Self, streamsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetSourceAsync: usize,
}
#[repr(C)]
pub struct ISvgImageSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(this: *mut *mut Self, urisource: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithUriSource: usize,
}
#[repr(C)]
pub struct ISvgImageSourceFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SvgImageSourceLoadStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISvgImageSourceOpenedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISvgImageSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UriSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RasterizePixelWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RasterizePixelHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVirtualSurfaceImageSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVirtualSurfaceImageSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut *mut Self, pixelwidth: i32, pixelheight: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity: unsafe extern "system" fn(this: *mut *mut Self, pixelwidth: i32, pixelheight: i32, isopaque: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWriteableBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub PixelBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelBuffer: usize,
    pub Invalidate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWriteableBitmapFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(this: *mut *mut Self, pixelwidth: i32, pixelheight: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlRenderingBackgroundTask {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IXamlRenderingBackgroundTaskFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlRenderingBackgroundTaskOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Background")]
    pub OnRun: unsafe extern "system" fn(this: *mut *mut Self, taskinstance: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    OnRun: usize,
}
pub type RenderTargetBitmap = *mut ::core::ffi::c_void;
pub type SoftwareBitmapSource = *mut ::core::ffi::c_void;
pub type SurfaceImageSource = *mut ::core::ffi::c_void;
pub type SvgImageSource = *mut ::core::ffi::c_void;
pub type SvgImageSourceFailedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Media_Imaging\"`*"]
#[repr(transparent)]
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
pub type SvgImageSourceOpenedEventArgs = *mut ::core::ffi::c_void;
pub type VirtualSurfaceImageSource = *mut ::core::ffi::c_void;
pub type WriteableBitmap = *mut ::core::ffi::c_void;
pub type XamlRenderingBackgroundTask = *mut ::core::ffi::c_void;
