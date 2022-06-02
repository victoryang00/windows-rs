#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundEnergyDiagnosticsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceSpecificConversionFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceSpecificConversionFactor: usize,
    #[cfg(feature = "deprecated")]
    pub ComputeTotalEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ComputeTotalEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub ResetTotalEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetTotalEnergyUsage: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IForegroundEnergyDiagnosticsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceSpecificConversionFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceSpecificConversionFactor: usize,
    #[cfg(feature = "deprecated")]
    pub ComputeTotalEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ComputeTotalEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub ResetTotalEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetTotalEnergyUsage: usize,
}
