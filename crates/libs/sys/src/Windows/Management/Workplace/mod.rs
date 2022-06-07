#[repr(C)]
pub struct IMdmAllowPolicyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBrowserAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCameraAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrosoftAccountAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStoreAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMdmAllowPolicyStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3281455591, data2: 29724, data3: 16882, data4: [164, 182, 49, 76, 49, 80, 37, 134] };
}
#[repr(C)]
pub struct IMdmPolicyStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetMessagingSyncPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MessagingSyncPolicy) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMdmPolicyStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3382474022, data2: 980, data3: 18937, data4: [169, 147, 67, 239, 204, 210, 101, 196] };
}
#[repr(C)]
pub struct IWorkplaceSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMicrosoftAccountOptional: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWorkplaceSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3831984125, data2: 11666, data3: 19464, data4: [186, 212, 246, 89, 11, 84, 166, 211] };
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
