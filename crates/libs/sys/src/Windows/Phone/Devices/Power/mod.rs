pub type Battery = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBattery {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemainingChargePercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
}
impl ::windows_sys::core::Interface for IBattery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2536168413, data2: 26400, data3: 18178, data4: [164, 118, 185, 211, 138, 0, 112, 227] };
}
#[repr(C)]
pub struct IBatteryStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBatteryStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4210408560, data2: 25449, data3: 4577, data4: [184, 108, 8, 0, 32, 12, 154, 102] };
}
