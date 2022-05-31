#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserNotificationListener {
    type Vtable = IUserNotificationListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62553e41_8a06_4cef_8215_6033a5be4b03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationListenerAccessStatus) -> ::windows_core::HRESULT,
    pub NotificationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNotificationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNotificationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kinds: super::NotificationKinds, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNotificationsAsync: usize,
    pub GetNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClearNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationid: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserNotificationListenerStatics {
    type Vtable = IUserNotificationListenerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff6123cf_4386_4aa3_b73d_b804e5b63b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UserNotificationListener(::windows_core::IUnknown);
impl UserNotificationListener {
    pub fn RequestAccessAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserNotificationListenerAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserNotificationListenerAccessStatus>>(result__)
        }
    }
    pub fn GetAccessStatus(&self) -> ::windows_core::Result<UserNotificationListenerAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserNotificationListenerAccessStatus>::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserNotificationListenerAccessStatus>(result__)
        }
    }
    pub fn NotificationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NotificationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNotificationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNotificationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNotificationsAsync(&self, kinds: super::NotificationKinds) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::UserNotification>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNotificationsAsync)(::windows_core::Interface::as_raw(this), kinds, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::UserNotification>>>(result__)
        }
    }
    pub fn GetNotification(&self, notificationid: u32) -> ::windows_core::Result<super::UserNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNotification)(::windows_core::Interface::as_raw(this), notificationid, result__.as_mut_ptr()).from_abi::<super::UserNotification>(result__)
        }
    }
    pub fn ClearNotifications(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearNotifications)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveNotification(&self, notificationid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNotification)(::windows_core::Interface::as_raw(this), notificationid).ok() }
    }
    pub fn Current() -> ::windows_core::Result<UserNotificationListener> {
        Self::IUserNotificationListenerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserNotificationListener>(result__)
        })
    }
    pub fn IUserNotificationListenerStatics<R, F: FnOnce(&IUserNotificationListenerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserNotificationListener, IUserNotificationListenerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserNotificationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotificationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationListener {}
impl ::core::fmt::Debug for UserNotificationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserNotificationListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Management.UserNotificationListener;{62553e41-8a06-4cef-8215-6033a5be4b03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserNotificationListener {
    type Vtable = IUserNotificationListener_Vtbl;
    const IID: ::windows_core::GUID = <IUserNotificationListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserNotificationListener {
    const NAME: &'static str = "Windows.UI.Notifications.Management.UserNotificationListener";
}
impl ::core::convert::From<UserNotificationListener> for ::windows_core::IUnknown {
    fn from(value: UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows_core::IUnknown {
    fn from(value: &UserNotificationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserNotificationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserNotificationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserNotificationListener> for ::windows_core::IInspectable {
    fn from(value: UserNotificationListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationListener> for ::windows_core::IInspectable {
    fn from(value: &UserNotificationListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserNotificationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserNotificationListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserNotificationListener {}
unsafe impl ::core::marker::Sync for UserNotificationListener {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserNotificationListenerAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserNotificationListenerAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserNotificationListenerAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationListenerAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserNotificationListenerAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
