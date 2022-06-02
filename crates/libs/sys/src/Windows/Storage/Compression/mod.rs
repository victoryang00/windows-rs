#[doc = "*Required features: `\"Storage_Compression\"`*"]
#[repr(transparent)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: Self = Self(0i32);
    pub const NullAlgorithm: Self = Self(1i32);
    pub const Mszip: Self = Self(2i32);
    pub const Xpress: Self = Self(3i32);
    pub const XpressHuff: Self = Self(4i32);
    pub const Lzms: Self = Self(5i32);
}
impl ::core::marker::Copy for CompressAlgorithm {}
impl ::core::clone::Clone for CompressAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Compressor = *mut ::core::ffi::c_void;
pub type Decompressor = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICompressor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DetachStream: usize,
}
#[repr(C)]
pub struct ICompressorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressor: unsafe extern "system" fn(this: *mut *mut Self, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressorEx: unsafe extern "system" fn(this: *mut *mut Self, underlyingstream: *mut ::core::ffi::c_void, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressorEx: usize,
}
#[repr(C)]
pub struct IDecompressor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DetachStream: usize,
}
#[repr(C)]
pub struct IDecompressorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(this: *mut *mut Self, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
