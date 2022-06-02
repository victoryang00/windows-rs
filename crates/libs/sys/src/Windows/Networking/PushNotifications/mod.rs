#[repr(C)]
pub struct IPushNotificationChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PushNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PushNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePushNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePushNotificationReceived: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerForUser2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(this: *mut *mut Self, appserverkey: *mut ::core::ffi::c_void, channelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut *mut Self, appserverkey: *mut ::core::ffi::c_void, channelid: ::windows_sys::core::HSTRING, appid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ChannelsRevoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelsRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChannelsRevoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChannelsRevoked: usize,
}
#[repr(C)]
pub struct IPushNotificationChannelsRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPushNotificationReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PushNotificationType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRawNotification {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRawNotification2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRawNotification3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
pub type PushNotificationChannel = *mut ::core::ffi::c_void;
pub type PushNotificationChannelManagerForUser = *mut ::core::ffi::c_void;
pub type PushNotificationChannelsRevokedEventArgs = *mut ::core::ffi::c_void;
pub type PushNotificationReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
impl ::core::marker::Copy for PushNotificationType {}
impl ::core::clone::Clone for PushNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RawNotification = *mut ::core::ffi::c_void;
