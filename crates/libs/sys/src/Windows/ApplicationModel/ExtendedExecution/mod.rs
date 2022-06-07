#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: Self = Self(0i32);
    pub const LocationTracking: Self = Self(1i32);
    pub const SavingData: Self = Self(2i32);
}
impl ::core::marker::Copy for ExtendedExecutionReason {}
impl ::core::clone::Clone for ExtendedExecutionReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionResult {}
impl ::core::clone::Clone for ExtendedExecutionResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExtendedExecutionRevokedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExtendedExecutionSession = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IExtendedExecutionRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExtendedExecutionRevokedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExtendedExecutionRevokedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3216809750, data2: 25525, data3: 19467, data4: [170, 214, 130, 138, 245, 55, 62, 195] };
}
#[repr(C)]
pub struct IExtendedExecutionSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExtendedExecutionReason) -> ::windows_sys::core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut *mut Self, value: ExtendedExecutionReason) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PercentProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPercentProgress: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Revoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Revoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRevoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RequestExtensionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestExtensionAsync: usize,
}
impl ::windows_sys::core::Interface for IExtendedExecutionSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2945485357, data2: 4491, data3: 18673, data4: [147, 8, 12, 79, 196, 30, 32, 15] };
}
