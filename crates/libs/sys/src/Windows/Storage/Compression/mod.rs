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
impl ::windows_sys::core::Interface for ICompressor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 180577370, data2: 22444, data3: 20193, data4: [183, 2, 132, 211, 157, 84, 36, 224] };
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
impl ::windows_sys::core::Interface for ICompressorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1597871780, data2: 11515, data3: 17452, data4: [168, 186, 215, 209, 27, 3, 157, 160] };
}
#[repr(C)]
pub struct IDecompressor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub DetachStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DetachStream: usize,
}
impl ::windows_sys::core::Interface for IDecompressor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3095658054, data2: 54922, data3: 19595, data4: [173, 160, 78, 232, 19, 252, 82, 131] };
}
#[repr(C)]
pub struct IDecompressorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(this: *mut *mut Self, underlyingstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
impl ::windows_sys::core::Interface for IDecompressorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1396171346, data2: 7586, data3: 17121, data4: [136, 52, 3, 121, 210, 141, 116, 47] };
}
