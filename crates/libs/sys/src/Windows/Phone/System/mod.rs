#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[repr(C)]
pub struct ISystemProtectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemProtectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1237542240, data2: 38881, data3: 19865, data4: [139, 251, 190, 254, 170, 106, 206, 109] };
}
#[repr(C)]
pub struct ISystemProtectionUnlockStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemProtectionUnlockStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 110295615, data2: 36625, data3: 19531, data4: [170, 13, 135, 215, 175, 123, 23, 121] };
}
