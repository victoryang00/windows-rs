#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[repr(C)]
pub struct IResourceLoader {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, resource: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IResourceLoader2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetStringForUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForUri: usize,
}
#[repr(C)]
pub struct IResourceLoaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateResourceLoaderByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IResourceLoaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetStringForReference: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForReference: usize,
}
#[repr(C)]
pub struct IResourceLoaderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentViewWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForViewIndependentUseWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IResourceLoaderStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
#[repr(C)]
pub struct IResourceLoaderStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultPriPath: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type ResourceLoader = *mut ::core::ffi::c_void;
