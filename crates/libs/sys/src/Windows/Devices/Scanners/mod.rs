#[repr(C)]
pub struct IImageScanner {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultScanSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerScanSource) -> ::windows_sys::core::HRESULT,
    pub IsScanSourceSupported: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerScanSource, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FlatbedConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FeederConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPreviewSupported: unsafe extern "system" fn(this: *mut *mut Self, scansource: ImageScannerScanSource, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ScanPreviewToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, scansource: ImageScannerScanSource, targetstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ScanPreviewToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ScanFilesToFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, scansource: ImageScannerScanSource, storagefolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ScanFilesToFolderAsync: usize,
}
#[repr(C)]
pub struct IImageScannerFeederConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanAutoDetectPageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoDetectPageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoDetectPageSize: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Printing::PrintMediaSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Printing::PrintMediaSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub PageOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Printing::PrintOrientation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageOrientation: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Printing::PrintOrientation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageOrientation: usize,
    #[cfg(feature = "Foundation")]
    pub PageSizeDimensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSizeDimensions: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub IsPageSizeSupported: unsafe extern "system" fn(this: *mut *mut Self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    IsPageSizeSupported: usize,
    pub MaxNumberOfPages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxNumberOfPages: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CanScanDuplex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanScanAhead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ScanAhead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetScanAhead: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageScannerFormatConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerFormat) -> ::windows_sys::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerFormat) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerFormat) -> ::windows_sys::core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerFormat, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageScannerPreviewResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerFormat) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageScannerScanResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ScannedFiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ScannedFiles: usize,
}
#[repr(C)]
pub struct IImageScannerSourceConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MinScanArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinScanArea: usize,
    #[cfg(feature = "Foundation")]
    pub MaxScanArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxScanArea: usize,
    #[cfg(feature = "Foundation")]
    pub SelectedScanRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedScanRegion: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectedScanRegion: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectedScanRegion: usize,
    pub AutoCroppingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerAutoCroppingMode) -> ::windows_sys::core::HRESULT,
    pub SetAutoCroppingMode: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerAutoCroppingMode) -> ::windows_sys::core::HRESULT,
    pub IsAutoCroppingModeSupported: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MinResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub OpticalResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub DesiredResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub SetDesiredResolution: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub ActualResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerResolution) -> ::windows_sys::core::HRESULT,
    pub DefaultColorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerColorMode) -> ::windows_sys::core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ImageScannerColorMode) -> ::windows_sys::core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerColorMode) -> ::windows_sys::core::HRESULT,
    pub IsColorModeSupported: unsafe extern "system" fn(this: *mut *mut Self, value: ImageScannerColorMode, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MinBrightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxBrightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BrightnessStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DefaultBrightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MinContrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxContrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ContrastStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DefaultContrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageScannerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type ImageScanner = *mut ::core::ffi::c_void;
pub type ImageScannerAutoConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: Self = Self(0i32);
    pub const SingleRegion: Self = Self(1i32);
    pub const MultipleRegion: Self = Self(2i32);
}
impl ::core::marker::Copy for ImageScannerAutoCroppingMode {}
impl ::core::clone::Clone for ImageScannerAutoCroppingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: Self = Self(0i32);
    pub const Grayscale: Self = Self(1i32);
    pub const Monochrome: Self = Self(2i32);
    pub const AutoColor: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerColorMode {}
impl ::core::clone::Clone for ImageScannerColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ImageScannerFeederConfiguration = *mut ::core::ffi::c_void;
pub type ImageScannerFlatbedConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const DeviceIndependentBitmap: Self = Self(2i32);
    pub const Tiff: Self = Self(3i32);
    pub const Xps: Self = Self(4i32);
    pub const OpenXps: Self = Self(5i32);
    pub const Pdf: Self = Self(6i32);
}
impl ::core::marker::Copy for ImageScannerFormat {}
impl ::core::clone::Clone for ImageScannerFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ImageScannerPreviewResult = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl ::core::marker::Copy for ImageScannerResolution {}
impl ::core::clone::Clone for ImageScannerResolution {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ImageScannerScanResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Scanners\"`*"]
#[repr(transparent)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: Self = Self(0i32);
    pub const Flatbed: Self = Self(1i32);
    pub const Feeder: Self = Self(2i32);
    pub const AutoConfigured: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerScanSource {}
impl ::core::clone::Clone for ImageScannerScanSource {
    fn clone(&self) -> Self {
        *self
    }
}
