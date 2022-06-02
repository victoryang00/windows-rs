#[repr(C)]
pub struct INamedPolicyData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Area: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NamedPolicyKind) -> ::windows_sys::core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUserPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub GetBoolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBinary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBinary: usize,
    pub GetInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[repr(C)]
pub struct INamedPolicyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPolicyFromPath: unsafe extern "system" fn(this: *mut *mut Self, area: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetPolicyFromPathForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, area: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetPolicyFromPathForUser: usize,
}
pub type NamedPolicyData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Policies\"`*"]
#[repr(transparent)]
pub struct NamedPolicyKind(pub i32);
impl NamedPolicyKind {
    pub const Invalid: Self = Self(0i32);
    pub const Binary: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Int32: Self = Self(3i32);
    pub const Int64: Self = Self(4i32);
    pub const String: Self = Self(5i32);
}
impl ::core::marker::Copy for NamedPolicyKind {}
impl ::core::clone::Clone for NamedPolicyKind {
    fn clone(&self) -> Self {
        *self
    }
}
