#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IRetailModeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
