pub type CustomXamlResourceLoader = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICustomXamlResourceLoader {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICustomXamlResourceLoaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomXamlResourceLoaderOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetResource: unsafe extern "system" fn(this: *mut *mut Self, resourceid: ::windows_sys::core::HSTRING, objecttype: ::windows_sys::core::HSTRING, propertyname: ::windows_sys::core::HSTRING, propertytype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomXamlResourceLoaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCurrent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
