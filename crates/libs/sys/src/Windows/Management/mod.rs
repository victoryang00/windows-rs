#[cfg(feature = "Management_Core")]
pub mod Core;
#[cfg(feature = "Management_Deployment")]
pub mod Deployment;
#[cfg(feature = "Management_Policies")]
pub mod Policies;
#[cfg(feature = "Management_Update")]
pub mod Update;
#[cfg(feature = "Management_Workplace")]
pub mod Workplace;
#[repr(C)]
pub struct IMdmAlert {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MdmAlertDataType) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: MdmAlertDataType) -> ::windows_sys::core::HRESULT,
    pub Mark: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MdmAlertMark) -> ::windows_sys::core::HRESULT,
    pub SetMark: unsafe extern "system" fn(this: *mut *mut Self, value: MdmAlertMark) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMdmSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Alerts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Alerts: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MdmSessionState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithAlertsAsync: unsafe extern "system" fn(this: *mut *mut Self, alerts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithAlertsAsync: usize,
}
#[repr(C)]
pub struct IMdmSessionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SessionIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SessionIds: usize,
    pub TryCreateSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteSessionById: unsafe extern "system" fn(this: *mut *mut Self, sessionid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetSessionById: unsafe extern "system" fn(this: *mut *mut Self, sessionid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MdmAlert = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
pub struct MdmAlertDataType(pub i32);
impl MdmAlertDataType {
    pub const String: Self = Self(0i32);
    pub const Base64: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
}
impl ::core::marker::Copy for MdmAlertDataType {}
impl ::core::clone::Clone for MdmAlertDataType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
pub struct MdmAlertMark(pub i32);
impl MdmAlertMark {
    pub const None: Self = Self(0i32);
    pub const Fatal: Self = Self(1i32);
    pub const Critical: Self = Self(2i32);
    pub const Warning: Self = Self(3i32);
    pub const Informational: Self = Self(4i32);
}
impl ::core::marker::Copy for MdmAlertMark {}
impl ::core::clone::Clone for MdmAlertMark {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MdmSession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
pub struct MdmSessionState(pub i32);
impl MdmSessionState {
    pub const NotStarted: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Communicating: Self = Self(3i32);
    pub const AlertStatusAvailable: Self = Self(4i32);
    pub const Retrying: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for MdmSessionState {}
impl ::core::clone::Clone for MdmSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
