#[repr(C)]
pub struct IMdmAllowPolicyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBrowserAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCameraAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrosoftAccountAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStoreAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMdmPolicyStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetMessagingSyncPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MessagingSyncPolicy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWorkplaceSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMicrosoftAccountOptional: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Management_Workplace\"`*"]
#[repr(transparent)]
pub struct MessagingSyncPolicy(pub i32);
impl MessagingSyncPolicy {
    pub const Disallowed: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MessagingSyncPolicy {}
impl ::core::clone::Clone for MessagingSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
