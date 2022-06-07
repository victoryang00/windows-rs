#[repr(C)]
pub struct IOcrEngine {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    RecognizeAsync: usize,
    #[cfg(feature = "Globalization")]
    pub RecognizerLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    RecognizerLanguage: usize,
}
impl ::windows_sys::core::Interface for IOcrEngine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1511308353, data2: 23414, data3: 12608, data4: [182, 128, 136, 37, 86, 38, 131, 172] };
}
#[repr(C)]
pub struct IOcrEngineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxImageDimension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub AvailableRecognizerLanguages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    AvailableRecognizerLanguages: usize,
    #[cfg(feature = "Globalization")]
    pub IsLanguageSupported: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    IsLanguageSupported: usize,
    #[cfg(feature = "Globalization")]
    pub TryCreateFromLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    TryCreateFromLanguage: usize,
    pub TryCreateFromUserProfileLanguages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOcrEngineStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1543481434, data2: 13188, data3: 13632, data4: [153, 64, 105, 145, 32, 212, 40, 168] };
}
#[repr(C)]
pub struct IOcrLine {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Words: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Words: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOcrLine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4432239, data2: 58143, data3: 14884, data4: [137, 156, 212, 68, 189, 8, 129, 36] };
}
#[repr(C)]
pub struct IOcrResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
    #[cfg(feature = "Foundation")]
    pub TextAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextAngle: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOcrResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2614244786, data2: 5979, data3: 15722, data4: [146, 226, 56, 140, 32, 110, 47, 99] };
}
#[repr(C)]
pub struct IOcrWord {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOcrWord {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1009403770, data2: 23769, data3: 13605, data4: [186, 42, 35, 209, 224, 166, 138, 29] };
}
pub type OcrEngine = *mut ::core::ffi::c_void;
pub type OcrLine = *mut ::core::ffi::c_void;
pub type OcrResult = *mut ::core::ffi::c_void;
pub type OcrWord = *mut ::core::ffi::c_void;
