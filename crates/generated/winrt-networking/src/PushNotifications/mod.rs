#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b28102e_ef0b_4f39_9b8a_a3c194de7081);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4c45704_1182_42c7_8890_f563c4890dc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerForUser2 {
    type Vtable = IPushNotificationChannelManagerForUser2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc38b066a_7cc1_4dac_87fd_be6e920414a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: ::windows_core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: ::windows_core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics {
    type Vtable = IPushNotificationChannelManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8baf9b65_77a1_4588_bd19_861529a9dcf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics2 {
    type Vtable = IPushNotificationChannelManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb444a65d_a7e9_4b28_950e_f375a907f9df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics3 {
    type Vtable = IPushNotificationChannelManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4701fefe_0ede_4a3f_ae78_bfa471496925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelManagerStatics4 {
    type Vtable = IPushNotificationChannelManagerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc540efb_7820_5a5b_9c01_b4757f774025);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelsRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e1a24c_1a34_5beb_aae2_40c232c8c140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1065e0c_36cd_484c_b935_0a99b753cf00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification {
    type Vtable = IRawNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a227281_3b79_42ac_9963_22ab00d4f0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification2 {
    type Vtable = IRawNotification2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d0cf19_0c6f_4cdd_9424_eec5be014d26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRawNotification3 {
    type Vtable = IRawNotification3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62737dde_8a73_424c_ab44_5635f40a96e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
#[repr(transparent)]
pub struct PushNotificationChannel(::windows_core::IUnknown);
impl PushNotificationChannel {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PushNotificationReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PushNotificationReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePushNotificationReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePushNotificationReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannel {}
impl ::core::fmt::Debug for PushNotificationChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannel;{2b28102e-ef0b-4f39-9b8a-a3c194de7081})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows_core::GUID = <IPushNotificationChannel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannel";
}
impl ::core::convert::From<PushNotificationChannel> for ::windows_core::IUnknown {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows_core::IUnknown {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PushNotificationChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannel> for ::windows_core::IInspectable {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows_core::IInspectable {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PushNotificationChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
pub struct PushNotificationChannelManager;
impl PushNotificationChannelManager {
    pub fn CreatePushNotificationChannelForApplicationAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(applicationid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    pub fn CreatePushNotificationChannelForSecondaryTileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(tileid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<PushNotificationChannelManagerForUser>(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PushNotificationChannelManagerForUser>(result__)
        })
    }
    pub fn ChannelsRevoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelsRevoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveChannelsRevoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelsRevoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IPushNotificationChannelManagerStatics<R, F: FnOnce(&IPushNotificationChannelManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPushNotificationChannelManagerStatics2<R, F: FnOnce(&IPushNotificationChannelManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPushNotificationChannelManagerStatics3<R, F: FnOnce(&IPushNotificationChannelManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPushNotificationChannelManagerStatics4<R, F: FnOnce(&IPushNotificationChannelManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PushNotificationChannelManager {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManager";
}
#[repr(transparent)]
pub struct PushNotificationChannelManagerForUser(::windows_core::IUnknown);
impl PushNotificationChannelManagerForUser {
    pub fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    pub fn CreatePushNotificationChannelForSecondaryTileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appserverkey: Param0, channelid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = &::windows_core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(::windows_core::Interface::as_raw(this), appserverkey.into_param().abi(), channelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appserverkey: Param0, channelid: Param1, appid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = &::windows_core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(::windows_core::Interface::as_raw(this), appserverkey.into_param().abi(), channelid.into_param().abi(), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PushNotificationChannel>>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationChannelManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelManagerForUser {}
impl ::core::fmt::Debug for PushNotificationChannelManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PushNotificationChannelManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser;{a4c45704-1182-42c7-8890-f563c4890dc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IPushNotificationChannelManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
}
impl ::core::convert::From<PushNotificationChannelManagerForUser> for ::windows_core::IUnknown {
    fn from(value: PushNotificationChannelManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &PushNotificationChannelManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannelManagerForUser> for ::windows_core::IInspectable {
    fn from(value: PushNotificationChannelManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &PushNotificationChannelManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PushNotificationChannelManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannelManagerForUser {}
unsafe impl ::core::marker::Sync for PushNotificationChannelManagerForUser {}
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(::windows_core::IUnknown);
impl PushNotificationChannelsRevokedEventArgs {}
impl ::core::clone::Clone for PushNotificationChannelsRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelsRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelsRevokedEventArgs {}
impl ::core::fmt::Debug for PushNotificationChannelsRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelsRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs;{20e1a24c-1a34-5beb-aae2-40c232c8c140})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPushNotificationChannelsRevokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
}
impl ::core::convert::From<PushNotificationChannelsRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PushNotificationChannelsRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelsRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PushNotificationChannelsRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannelsRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PushNotificationChannelsRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannelsRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PushNotificationChannelsRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PushNotificationChannelsRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannelsRevokedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationChannelsRevokedEventArgs {}
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(::windows_core::IUnknown);
impl PushNotificationReceivedEventArgs {
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NotificationType(&self) -> ::windows_core::Result<PushNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PushNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).NotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PushNotificationType>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn ToastNotification(&self) -> ::windows_core::Result<::winrt_ui::Notifications::ToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::ToastNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn TileNotification(&self) -> ::windows_core::Result<::winrt_ui::Notifications::TileNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TileNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::TileNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn BadgeNotification(&self) -> ::windows_core::Result<::winrt_ui::Notifications::BadgeNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BadgeNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::BadgeNotification>(result__)
        }
    }
    pub fn RawNotification(&self) -> ::windows_core::Result<RawNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RawNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RawNotification>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationReceivedEventArgs {}
impl ::core::fmt::Debug for PushNotificationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs;{d1065e0c-36cd-484c-b935-0a99b753cf00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPushNotificationReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PushNotificationReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PushNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PushNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PushNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PushNotificationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.PushNotifications.PushNotificationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RawNotification(::windows_core::IUnknown);
impl RawNotification {
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ChannelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentBytes(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = &::windows_core::Interface::cast::<IRawNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for RawNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RawNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RawNotification {}
impl ::core::fmt::Debug for RawNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RawNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.RawNotification;{1a227281-3b79-42ac-9963-22ab00d4f0b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RawNotification {
    type Vtable = IRawNotification_Vtbl;
    const IID: ::windows_core::GUID = <IRawNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.RawNotification";
}
impl ::core::convert::From<RawNotification> for ::windows_core::IUnknown {
    fn from(value: RawNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawNotification> for ::windows_core::IUnknown {
    fn from(value: &RawNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RawNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RawNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RawNotification> for ::windows_core::IInspectable {
    fn from(value: RawNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawNotification> for ::windows_core::IInspectable {
    fn from(value: &RawNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RawNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RawNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RawNotification {}
unsafe impl ::core::marker::Sync for RawNotification {}
