#[repr(C)]
pub struct IRadio {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetStateAsync: unsafe extern "system" fn(this: *mut *mut Self, value: RadioState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RadioState) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RadioKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRadioStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRadiosAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRadiosAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
pub type Radio = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Radios\"`*"]
#[repr(transparent)]
pub struct RadioAccessStatus(pub i32);
impl RadioAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioAccessStatus {}
impl ::core::clone::Clone for RadioAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Radios\"`*"]
#[repr(transparent)]
pub struct RadioKind(pub i32);
impl RadioKind {
    pub const Other: Self = Self(0i32);
    pub const WiFi: Self = Self(1i32);
    pub const MobileBroadband: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const FM: Self = Self(4i32);
}
impl ::core::marker::Copy for RadioKind {}
impl ::core::clone::Clone for RadioKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Radios\"`*"]
#[repr(transparent)]
pub struct RadioState(pub i32);
impl RadioState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioState {}
impl ::core::clone::Clone for RadioState {
    fn clone(&self) -> Self {
        *self
    }
}
