pub type AppCapability = *mut ::core::ffi::c_void;
pub type AppCapabilityAccessChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authorization_AppCapabilityAccess\"`*"]
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
}
impl ::core::marker::Copy for AppCapabilityAccessStatus {}
impl ::core::clone::Clone for AppCapabilityAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAppCapability {
    pub base__: ::windows_sys::core::IInspectable,
    pub CapabilityName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub CheckAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCapabilityAccessStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AccessChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccessChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessChanged: usize,
}
#[repr(C)]
pub struct IAppCapabilityAccessChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppCapabilityStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAccessForCapabilitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, capabilitynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAccessForCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub RequestAccessForCapabilitiesForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, capabilitynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    RequestAccessForCapabilitiesForUserAsync: usize,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, capabilityname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub CreateWithProcessIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, capabilityname: ::windows_sys::core::HSTRING, pid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateWithProcessIdForUser: usize,
}
