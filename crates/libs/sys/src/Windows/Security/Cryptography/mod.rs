#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[doc = "*Required features: `\"Security_Cryptography\"`*"]
#[repr(transparent)]
pub struct BinaryStringEncoding(pub i32);
impl BinaryStringEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for BinaryStringEncoding {}
impl ::core::clone::Clone for BinaryStringEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICryptographicBufferStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, object1: *mut ::core::ffi::c_void, object2: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Compare: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GenerateRandom: unsafe extern "system" fn(this: *mut *mut Self, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GenerateRandom: usize,
    pub GenerateRandomNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromByteArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToByteArray: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToByteArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromHexString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToHexString: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToHexString: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecodeFromBase64String: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecodeFromBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncodeToBase64String: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncodeToBase64String: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertStringToBinary: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, encoding: BinaryStringEncoding, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertStringToBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ConvertBinaryToString: unsafe extern "system" fn(this: *mut *mut Self, encoding: BinaryStringEncoding, buffer: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ConvertBinaryToString: usize,
}
