#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IRetailModeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IRetailModeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621703721, data2: 64986, data3: 17383, data4: [147, 251, 229, 58, 182, 232, 158, 195] };
}
