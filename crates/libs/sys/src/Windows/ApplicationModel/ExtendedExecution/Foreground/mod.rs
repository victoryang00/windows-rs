#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: Self = Self(0i32);
    pub const SavingData: Self = Self(1i32);
    pub const BackgroundAudio: Self = Self(2i32);
    pub const Unconstrained: Self = Self(3i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundResult {}
impl ::core::clone::Clone for ExtendedExecutionForegroundResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExtendedExecutionForegroundRevokedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExtendedExecutionForegroundSession = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IExtendedExecutionForegroundRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExtendedExecutionForegroundRevokedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExtendedExecutionForegroundRevokedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2960972096, data2: 38231, data3: 44708, data4: [44, 153, 189, 213, 109, 155, 228, 97] };
}
#[repr(C)]
pub struct IExtendedExecutionForegroundSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Revoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Revoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRevoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RequestExtensionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestExtensionAsync: usize,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ExtendedExecutionForegroundReason) -> ::windows_sys::core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut *mut Self, value: ExtendedExecutionForegroundReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExtendedExecutionForegroundSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4227088609, data2: 40208, data3: 16897, data4: [176, 30, 200, 50, 117, 41, 111, 46] };
}
