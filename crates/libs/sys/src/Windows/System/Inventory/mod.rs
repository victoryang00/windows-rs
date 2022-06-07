#[repr(C)]
pub struct IInstalledDesktopApp {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInstalledDesktopApp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1978317037, data2: 49340, data3: 21348, data4: [76, 40, 22, 110, 5, 69, 22, 122] };
}
#[repr(C)]
pub struct IInstalledDesktopAppStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInventoryAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInventoryAsync: usize,
}
impl ::windows_sys::core::Interface for IInstalledDesktopAppStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 642578254, data2: 8653, data3: 24475, data4: [96, 86, 120, 102, 173, 114, 72, 154] };
}
pub type InstalledDesktopApp = *mut ::core::ffi::c_void;
