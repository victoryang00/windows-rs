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
impl ::windows_sys::core::Interface for IPushNotificationChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 724045870, data2: 61195, data3: 20281, data4: [155, 138, 163, 193, 148, 222, 112, 129] };
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
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2764330756, data2: 4482, data3: 17095, data4: [136, 144, 245, 99, 196, 137, 13, 196] };
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
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerForUser2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3280668266, data2: 31937, data3: 19884, data4: [135, 253, 190, 110, 146, 4, 20, 164] };
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
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2343541605, data2: 30625, data3: 17800, data4: [189, 25, 134, 21, 41, 169, 220, 240] };
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024397917, data2: 42985, data3: 19240, data4: [149, 14, 243, 117, 169, 7, 249, 223] };
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1191313150, data2: 3806, data3: 19007, data4: [174, 120, 191, 164, 113, 73, 105, 37] };
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
impl ::windows_sys::core::Interface for IPushNotificationChannelManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159625467, data2: 30752, data3: 23131, data4: [156, 1, 180, 117, 127, 119, 64, 37] };
}
#[repr(C)]
pub struct IPushNotificationChannelsRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPushNotificationChannelsRevokedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 551658060, data2: 6708, data3: 23531, data4: [170, 226, 64, 194, 50, 200, 193, 64] };
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
impl ::windows_sys::core::Interface for IPushNotificationReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3506855436, data2: 14029, data3: 18508, data4: [185, 53, 10, 153, 183, 83, 207, 0] };
}
#[repr(C)]
pub struct IRawNotification {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRawNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 438465153, data2: 15225, data3: 17068, data4: [153, 99, 34, 171, 0, 212, 240, 183] };
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
impl ::windows_sys::core::Interface for IRawNotification2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872444185, data2: 3183, data3: 19677, data4: [148, 36, 238, 197, 190, 1, 77, 38] };
}
#[repr(C)]
pub struct IRawNotification3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
impl ::windows_sys::core::Interface for IRawNotification3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1651736030, data2: 35443, data3: 16972, data4: [171, 68, 86, 53, 244, 10, 150, 229] };
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
