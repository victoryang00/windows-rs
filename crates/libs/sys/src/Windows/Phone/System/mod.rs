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
#[repr(C)]
pub struct ISystemProtectionUnlockStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
