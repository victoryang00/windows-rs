#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[repr(C)]
pub struct IResourceLoader {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 139610376, data2: 5871, data3: 17837, data4: [166, 2, 41, 54, 55, 215, 230, 26] };
}
#[repr(C)]
pub struct IResourceLoader2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetStringForUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForUri: usize,
}
impl ::windows_sys::core::Interface for IResourceLoader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 283864774, data2: 33080, data3: 18625, data4: [188, 101, 225, 241, 66, 7, 54, 124] };
}
#[repr(C)]
pub struct IResourceLoaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateResourceLoaderByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceLoaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3275372035, data2: 27100, data3: 17029, data4: [160, 119, 213, 192, 228, 124, 203, 232] };
}
#[repr(C)]
pub struct IResourceLoaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetStringForReference: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForReference: usize,
}
impl ::windows_sys::core::Interface for IResourceLoaderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3212279009, data2: 6600, data3: 18882, data4: [149, 60, 71, 233, 34, 123, 51, 78] };
}
#[repr(C)]
pub struct IResourceLoaderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentViewWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForViewIndependentUseWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceLoaderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 213926209, data2: 25702, data3: 18825, data4: [148, 148, 11, 130, 223, 197, 63, 31] };
}
#[repr(C)]
pub struct IResourceLoaderStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
impl ::windows_sys::core::Interface for IResourceLoaderStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1684053499, data2: 25772, data3: 18715, data4: [129, 0, 14, 85, 141, 97, 193, 208] };
}
#[repr(C)]
pub struct IResourceLoaderStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultPriPath: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceLoaderStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2679335986, data2: 27788, data3: 17174, data4: [150, 46, 144, 149, 57, 181, 194, 89] };
}
pub type ResourceLoader = *mut ::core::ffi::c_void;
