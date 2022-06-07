pub type DataProtectionProvider = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDataProtectionProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, src: *mut ::core::ffi::c_void, dest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, src: *mut ::core::ffi::c_void, dest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
}
impl ::windows_sys::core::Interface for IDataProtectionProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 157522248, data2: 60706, data3: 17008, data4: [189, 28, 109, 114, 192, 15, 135, 135] };
}
#[repr(C)]
pub struct IDataProtectionProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateOverloadExplicit: unsafe extern "system" fn(this: *mut *mut Self, protectiondescriptor: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataProtectionProviderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2918399404, data2: 18738, data3: 19679, data4: [172, 65, 114, 20, 51, 53, 20, 202] };
}
