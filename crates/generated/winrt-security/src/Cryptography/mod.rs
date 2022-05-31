#[cfg(feature = "Certificates")]
pub mod Certificates;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "DataProtection")]
pub mod DataProtection;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for BinaryStringEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BinaryStringEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for BinaryStringEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BinaryStringEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BinaryStringEncoding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.BinaryStringEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CryptographicBuffer;
impl CryptographicBuffer {
    #[cfg(feature = "winrt-storage")]
    pub fn Compare<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(object1: Param0, object2: Param1) -> ::windows_core::Result<bool> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(::windows_core::Interface::as_raw(this), object1.into_param().abi(), object2.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GenerateRandom(length: u32) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateRandom)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    pub fn GenerateRandomNumber() -> ::windows_core::Result<u32> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateRandomNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateFromByteArray(value: &[u8]) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromByteArray)(::windows_core::Interface::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CopyToByteArray<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(buffer: Param0, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        Self::ICryptographicBufferStatics(|this| unsafe { (::windows_core::Interface::vtable(this).CopyToByteArray)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), value.set_abi_len(), value as *mut _ as _).ok() })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn DecodeFromHexString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeFromHexString)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn EncodeToHexString<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(buffer: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EncodeToHexString)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn DecodeFromBase64String<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecodeFromBase64String)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn EncodeToBase64String<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(buffer: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EncodeToBase64String)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ConvertStringToBinary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0, encoding: BinaryStringEncoding) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertStringToBinary)(::windows_core::Interface::as_raw(this), value.into_param().abi(), encoding, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ConvertBinaryToString<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(encoding: BinaryStringEncoding, buffer: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICryptographicBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertBinaryToString)(::windows_core::Interface::as_raw(this), encoding, buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ICryptographicBufferStatics<R, F: FnOnce(&ICryptographicBufferStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CryptographicBuffer, ICryptographicBufferStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CryptographicBuffer {
    const NAME: &'static str = "Windows.Security.Cryptography.CryptographicBuffer";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicBufferStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICryptographicBufferStatics {
    type Vtable = ICryptographicBufferStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x320b7e22_3cb0_4cdf_8663_1d28910065eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicBufferStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object1: ::windows_core::RawPtr, object2: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Compare: usize,
    #[cfg(feature = "winrt-storage")]
    pub GenerateRandom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GenerateRandom: usize,
    pub GenerateRandomNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub CreateFromByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateFromByteArray: usize,
    #[cfg(feature = "winrt-storage")]
    pub CopyToByteArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CopyToByteArray: usize,
    #[cfg(feature = "winrt-storage")]
    pub DecodeFromHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    DecodeFromHexString: usize,
    #[cfg(feature = "winrt-storage")]
    pub EncodeToHexString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    EncodeToHexString: usize,
    #[cfg(feature = "winrt-storage")]
    pub DecodeFromBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    DecodeFromBase64String: usize,
    #[cfg(feature = "winrt-storage")]
    pub EncodeToBase64String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    EncodeToBase64String: usize,
    #[cfg(feature = "winrt-storage")]
    pub ConvertStringToBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, encoding: BinaryStringEncoding, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ConvertStringToBinary: usize,
    #[cfg(feature = "winrt-storage")]
    pub ConvertBinaryToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoding: BinaryStringEncoding, buffer: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ConvertBinaryToString: usize,
}
