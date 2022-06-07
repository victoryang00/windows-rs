#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
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
pub type BitmapBuffer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
pub type BitmapCodecInformation = *mut ::core::ffi::c_void;
pub type BitmapDecoder = *mut ::core::ffi::c_void;
pub type BitmapEncoder = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
pub type BitmapFrame = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
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
pub type BitmapProperties = *mut ::core::ffi::c_void;
pub type BitmapPropertiesView = *mut ::core::ffi::c_void;
pub type BitmapPropertySet = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
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
pub type BitmapTransform = *mut ::core::ffi::c_void;
pub type BitmapTypedValue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
#[repr(C)]
pub struct IBitmapBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPlaneCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetPlaneDescription: unsafe extern "system" fn(this: *mut *mut Self, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBitmapBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2772305092, data2: 14748, data3: 17292, data4: [178, 143, 166, 58, 107, 131, 209, 161] };
}
#[repr(C)]
pub struct IBitmapCodecInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub CodecId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileExtensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileExtensions: usize,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MimeTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MimeTypes: usize,
}
impl ::windows_sys::core::Interface for IBitmapCodecInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1074572018, data2: 50352, data3: 17298, data4: [163, 176, 111, 111, 155, 169, 92, 180] };
}
#[repr(C)]
pub struct IBitmapDecoder {
    pub base__: ::windows_sys::core::IInspectable,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DecoderInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPreviewAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPreviewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, frameindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFrameAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapDecoder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2901353146, data2: 7540, data3: 19601, data4: [157, 252, 150, 32, 116, 82, 51, 230] };
}
#[repr(C)]
pub struct IBitmapDecoderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BmpDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub JpegDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PngDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TiffDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GifDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub JpegXRDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IcoDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDecoderInformationEnumerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDecoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateWithIdAsync: unsafe extern "system" fn(this: *mut *mut Self, decoderid: ::windows_sys::core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateWithIdAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapDecoderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133300518, data2: 48367, data3: 20117, data4: [186, 214, 35, 168, 34, 229, 141, 1] };
}
#[repr(C)]
pub struct IBitmapDecoderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeifDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub WebpDecoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBitmapDecoderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1354393834, data2: 39329, data3: 16580, data4: [128, 217, 174, 240, 218, 250, 108, 63] };
}
#[repr(C)]
pub struct IBitmapEncoder {
    pub base__: ::windows_sys::core::IInspectable,
    pub EncoderInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsThumbnailGenerated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsThumbnailGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetGeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub GeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetGeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub BitmapTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPixelData: unsafe extern "system" fn(this: *mut *mut Self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GoToNextFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GoToNextFrameAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GoToNextFrameWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GoToNextFrameWithEncodingOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapEncoder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734292195, data2: 57848, data3: 19284, data4: [149, 232, 50, 145, 149, 81, 206, 98] };
}
#[repr(C)]
pub struct IBitmapEncoderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BmpEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub JpegEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PngEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub TiffEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GifEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub JpegXREncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEncoderInformationEnumerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEncoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, encoderid: ::windows_sys::core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, encoderid: ::windows_sys::core::GUID, stream: *mut ::core::ffi::c_void, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateWithEncodingOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateForTranscodingAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateForTranscodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateForInPlacePropertyEncodingAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateForInPlacePropertyEncodingAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapEncoderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2806208167, data2: 42212, data3: 20153, data4: [142, 64, 86, 77, 231, 225, 204, 178] };
}
#[repr(C)]
pub struct IBitmapEncoderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeifEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBitmapEncoderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 868991577, data2: 65073, data3: 16817, data4: [184, 18, 8, 109, 33, 232, 126, 22] };
}
#[repr(C)]
pub struct IBitmapEncoderWithSoftwareBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBitmapEncoderWithSoftwareBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1751962177, data2: 17200, data3: 19575, data4: [172, 228, 3, 52, 150, 139, 23, 104] };
}
#[repr(C)]
pub struct IBitmapFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapAlphaMode) -> ::windows_sys::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OrientedPixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OrientedPixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataTransformedAsync: unsafe extern "system" fn(this: *mut *mut Self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataTransformedAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1923389980, data2: 32897, data3: 17293, data4: [145, 188, 148, 236, 252, 129, 133, 198] };
}
#[repr(C)]
pub struct IBitmapFrameWithSoftwareBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapConvertedAsync: unsafe extern "system" fn(this: *mut *mut Self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapConvertedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapTransformedAsync: unsafe extern "system" fn(this: *mut *mut Self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapTransformedAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapFrameWithSoftwareBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4264066202, data2: 16908, data3: 18787, data4: [135, 173, 105, 20, 54, 224, 131, 131] };
}
#[repr(C)]
pub struct IBitmapProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, propertiestoset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3936309019, data2: 46341, data3: 17488, data4: [164, 209, 232, 202, 148, 82, 157, 141] };
}
#[repr(C)]
pub struct IBitmapPropertiesView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IBitmapPropertiesView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2114971770, data2: 14960, data3: 18680, data4: [156, 85, 25, 108, 245, 165, 69, 245] };
}
#[repr(C)]
pub struct IBitmapTransform {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScaledWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetScaledWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ScaledHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetScaledHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub InterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, value: BitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapFlip) -> ::windows_sys::core::HRESULT,
    pub SetFlip: unsafe extern "system" fn(this: *mut *mut Self, value: BitmapFlip) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapRotation) -> ::windows_sys::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: BitmapRotation) -> ::windows_sys::core::HRESULT,
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapBounds) -> ::windows_sys::core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: BitmapBounds) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBitmapTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2926924612, data2: 57960, data3: 19765, data4: [173, 207, 233, 149, 211, 26, 141, 52] };
}
#[repr(C)]
pub struct IBitmapTypedValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::PropertyType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Type: usize,
}
impl ::windows_sys::core::Interface for IBitmapTypedValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3447735465, data2: 9283, data3: 16384, data4: [176, 205, 121, 49, 108, 86, 245, 137] };
}
#[repr(C)]
pub struct IBitmapTypedValueFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, r#type: super::super::Foundation::PropertyType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IBitmapTypedValueFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2463872409, data2: 52755, data3: 18107, data4: [149, 69, 203, 58, 63, 99, 235, 139] };
}
#[repr(C)]
pub struct IPixelDataProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub DetachPixelData: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPixelDataProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3716357925, data2: 6236, data3: 17813, data4: [159, 185, 204, 190, 110, 193, 138, 111] };
}
#[repr(C)]
pub struct ISoftwareBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BitmapAlphaMode) -> ::windows_sys::core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub LockBuffer: unsafe extern "system" fn(this: *mut *mut Self, mode: BitmapBufferAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CopyFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISoftwareBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1755186952, data2: 32495, data3: 18495, data4: [150, 63, 218, 147, 136, 24, 224, 115] };
}
#[repr(C)]
pub struct ISoftwareBitmapFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut *mut Self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISoftwareBitmapFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3382700905, data2: 11618, data3: 19783, data4: [166, 179, 79, 219, 106, 7, 253, 248] };
}
#[repr(C)]
pub struct ISoftwareBitmapStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Convert: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertWithAlpha: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyWithAlphaFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyWithAlphaFromBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyFromSurfaceAsync: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyFromSurfaceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyWithAlphaFromSurfaceAsync: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyWithAlphaFromSurfaceAsync: usize,
}
impl ::windows_sys::core::Interface for ISoftwareBitmapStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3741550043, data2: 26415, data3: 19101, data4: [128, 110, 194, 68, 47, 52, 62, 134] };
}
pub type ImageStream = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
pub type PixelDataProvider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
pub type SoftwareBitmap = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
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
