#[repr(C)]
pub struct IUserNotificationListener {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserNotificationListenerAccessStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotificationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotificationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotificationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotificationChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNotificationsAsync: unsafe extern "system" fn(this: *mut *mut Self, kinds: super::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNotificationsAsync: usize,
    pub GetNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearNotifications: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveNotification: unsafe extern "system" fn(this: *mut *mut Self, notificationid: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserNotificationListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1649753665, data2: 35334, data3: 19695, data4: [130, 21, 96, 51, 165, 190, 75, 3] };
}
#[repr(C)]
pub struct IUserNotificationListenerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserNotificationListenerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4284556239, data2: 17286, data3: 19107, data4: [183, 61, 184, 4, 229, 182, 59, 35] };
}
pub type UserNotificationListener = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Notifications_Management\"`*"]
#[repr(transparent)]
pub struct UserNotificationListenerAccessStatus(pub i32);
impl UserNotificationListenerAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for UserNotificationListenerAccessStatus {}
impl ::core::clone::Clone for UserNotificationListenerAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
