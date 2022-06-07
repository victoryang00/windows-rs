#[repr(C)]
pub struct IPdfDocument {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPage: unsafe extern "system" fn(this: *mut *mut Self, pageindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsPasswordProtected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPdfDocument {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2893987549, data2: 33018, data3: 16521, data4: [132, 110, 129, 183, 127, 245, 168, 108] };
}
#[repr(C)]
pub struct IPdfDocumentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileWithPasswordAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, password: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileWithPasswordAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithPasswordAsync: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, password: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithPasswordAsync: usize,
}
impl ::windows_sys::core::Interface for IPdfDocumentStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1127877471, data2: 49159, data3: 18312, data4: [144, 242, 8, 20, 61, 146, 37, 153] };
}
#[repr(C)]
pub struct IPdfPage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RenderToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RenderToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RenderWithOptionsToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RenderWithOptionsToStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PreparePageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreparePageAsync: usize,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    pub Dimensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PdfPageRotation) -> ::windows_sys::core::HRESULT,
    pub PreferredZoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPdfPage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2645864648, data2: 21280, data3: 19708, data4: [173, 118, 73, 63, 218, 208, 229, 148] };
}
#[repr(C)]
pub struct IPdfPageDimensions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MediaBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaBox: usize,
    #[cfg(feature = "Foundation")]
    pub CropBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CropBox: usize,
    #[cfg(feature = "Foundation")]
    pub BleedBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BleedBox: usize,
    #[cfg(feature = "Foundation")]
    pub TrimBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimBox: usize,
    #[cfg(feature = "Foundation")]
    pub ArtBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArtBox: usize,
}
impl ::windows_sys::core::Interface for IPdfPageDimensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 571933809, data2: 12606, data3: 17640, data4: [131, 93, 99, 163, 231, 98, 74, 16] };
}
#[repr(C)]
pub struct IPdfPageRenderOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SourceRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceRect: usize,
    pub DestinationWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDestinationWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DestinationHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDestinationHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub IsIgnoringHighContrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIgnoringHighContrast: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BitmapEncoderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetBitmapEncoderId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPdfPageRenderOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1016595823, data2: 47055, data3: 19497, data4: [154, 4, 82, 217, 2, 103, 244, 37] };
}
pub type PdfDocument = *mut ::core::ffi::c_void;
pub type PdfPage = *mut ::core::ffi::c_void;
pub type PdfPageDimensions = *mut ::core::ffi::c_void;
pub type PdfPageRenderOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
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
