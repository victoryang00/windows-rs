#[repr(C)]
pub struct IPowerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerSavingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PowerSavingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSavingModeChanged: unsafe extern "system" fn(this: *mut *mut Self, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSavingModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSavingModeChanged: usize,
}
impl ::windows_sys::core::Interface for IPowerManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635342800, data2: 7259, data3: 4577, data4: [189, 219, 8, 0, 32, 12, 154, 102] };
}
#[repr(C)]
pub struct IPowerManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPowerManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1499608783, data2: 6424, data3: 17745, data4: [164, 102, 197, 26, 174, 55, 59, 248] };
}
#[doc = "*Required features: `\"Phone_System_Power\"`*"]
#[repr(transparent)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSavingMode {}
impl ::core::clone::Clone for PowerSavingMode {
    fn clone(&self) -> Self {
        *self
    }
}
