#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl ::core::marker::Copy for BatteryStatus {}
impl ::core::clone::Clone for BatteryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Disabled: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for EnergySaverStatus {}
impl ::core::clone::Clone for EnergySaverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundEnergyManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearTerminationUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearTerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub TerminationUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TerminationUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IBackgroundEnergyManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3004571029, data2: 4480, data3: 17270, data4: [150, 225, 64, 149, 86, 129, 71, 206] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IForegroundEnergyManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub LowUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LowUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub NearMaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NearMaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub MaxAcceptableUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxAcceptableUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub ExcessiveUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExcessiveUsageLevel: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsage: usize,
    #[cfg(feature = "deprecated")]
    pub RecentEnergyUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecentEnergyUsageLevel: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageIncreased: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecentEnergyUsageReturnedToLow: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecentEnergyUsageReturnedToLow: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecentEnergyUsageReturnedToLow: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IForegroundEnergyManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2683857010, data2: 58999, data3: 18452, data4: [154, 32, 83, 55, 202, 115, 43, 152] };
}
#[repr(C)]
pub struct IPowerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnergySaverStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EnergySaverStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnergySaverStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnergySaverStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnergySaverStatusChanged: usize,
    pub BatteryStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BatteryStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BatteryStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BatteryStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBatteryStatusChanged: usize,
    pub PowerSupplyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PowerSupplyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSupplyStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSupplyStatusChanged: usize,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTimeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingDischargeTimeChanged: usize,
}
impl ::windows_sys::core::Interface for IPowerManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 328499805, data2: 25294, data3: 17252, data4: [152, 213, 170, 40, 199, 251, 209, 91] };
}
#[doc = "*Required features: `\"System_Power\"`*"]
#[repr(transparent)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl ::core::marker::Copy for PowerSupplyStatus {}
impl ::core::clone::Clone for PowerSupplyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
