pub type ApplicationDataManager = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IApplicationDataManager {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IApplicationDataManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1959855154, data2: 11929, data3: 16384, data4: [154, 58, 100, 48, 126, 133, 129, 41] };
}
#[repr(C)]
pub struct IApplicationDataManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
impl ::windows_sys::core::Interface for IApplicationDataManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 504914659, data2: 27022, data3: 18849, data4: [151, 82, 222, 233, 73, 37, 185, 179] };
}
