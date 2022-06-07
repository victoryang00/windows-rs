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
impl ::windows_sys::core::Interface for IBackgroundEnergyDiagnosticsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3613800194, data2: 54182, data3: 18144, data4: [143, 155, 80, 185, 91, 180, 249, 197] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IForegroundEnergyDiagnosticsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 600443159, data2: 52487, data3: 17929, data4: [190, 21, 143, 232, 148, 197, 228, 30] };
}
