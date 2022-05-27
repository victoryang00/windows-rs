#[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
#[repr(transparent)]
pub struct INotificationActivationCallback(::windows_core::IUnknown);
impl INotificationActivationCallback {
    #[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
    pub unsafe fn Activate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, appusermodelid: Param0, invokedargs: Param1, data: &[NOTIFICATION_USER_INPUT_DATA]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self), appusermodelid.into_param().abi(), invokedargs.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(data)), data.len() as _).ok()
    }
}
impl ::core::convert::From<INotificationActivationCallback> for ::windows_core::IUnknown {
    fn from(value: INotificationActivationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotificationActivationCallback> for ::windows_core::IUnknown {
    fn from(value: &INotificationActivationCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INotificationActivationCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INotificationActivationCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotificationActivationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotificationActivationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotificationActivationCallback {}
impl ::core::fmt::Debug for INotificationActivationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotificationActivationCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INotificationActivationCallback {
    type Vtable = INotificationActivationCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53e31837_6600_4a81_9395_75cffe746f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationActivationCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, invokedargs: ::windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows_core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Notifications\"`*"]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: ::windows_core::PCWSTR,
    pub Value: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NOTIFICATION_USER_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFICATION_USER_INPUT_DATA").field("Key", &self.Key).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for NOTIFICATION_USER_INPUT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NOTIFICATION_USER_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFICATION_USER_INPUT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::default::Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
