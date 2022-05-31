pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct DRendezvousSessionEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl DRendezvousSessionEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<DRendezvousSessionEvents> for ::windows_core::IUnknown {
    fn from(value: DRendezvousSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&DRendezvousSessionEvents> for ::windows_core::IUnknown {
    fn from(value: &DRendezvousSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DRendezvousSessionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<DRendezvousSessionEvents> for super::Com::IDispatch {
    fn from(value: DRendezvousSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&DRendezvousSessionEvents> for super::Com::IDispatch {
    fn from(value: &DRendezvousSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for DRendezvousSessionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a DRendezvousSessionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for DRendezvousSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for DRendezvousSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for DRendezvousSessionEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for DRendezvousSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRendezvousSessionEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for DRendezvousSessionEvents {
    type Vtable = DRendezvousSessionEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fa19cf8_64c4_4f53_ae60_635b3806eca6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct DRendezvousSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[repr(transparent)]
pub struct IRendezvousApplication(::windows_core::IUnknown);
impl IRendezvousApplication {
    pub unsafe fn SetRendezvousSession<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, prendezvoussession: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRendezvousSession)(::windows_core::Interface::as_raw(self), prendezvoussession.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRendezvousApplication> for ::windows_core::IUnknown {
    fn from(value: IRendezvousApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRendezvousApplication> for ::windows_core::IUnknown {
    fn from(value: &IRendezvousApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRendezvousApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRendezvousApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRendezvousApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRendezvousApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRendezvousApplication {}
impl ::core::fmt::Debug for IRendezvousApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRendezvousApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRendezvousApplication {
    type Vtable = IRendezvousApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f4d070b_a275_49fb_b10d_8ec26387b50d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousApplication_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetRendezvousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRendezvousSession(::windows_core::IUnknown);
impl IRendezvousSession {
    pub unsafe fn State(&self) -> ::windows_core::Result<RENDEZVOUS_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<RENDEZVOUS_SESSION_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RENDEZVOUS_SESSION_STATE>(result__)
    }
    pub unsafe fn RemoteUser(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemoteUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Flags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Flags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SendContextData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendContextData)(::windows_core::Interface::as_raw(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Terminate<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, hr: ::windows_core::HRESULT, bstrappdata: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), bstrappdata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRendezvousSession> for ::windows_core::IUnknown {
    fn from(value: IRendezvousSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRendezvousSession> for ::windows_core::IUnknown {
    fn from(value: &IRendezvousSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRendezvousSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRendezvousSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRendezvousSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRendezvousSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRendezvousSession {}
impl ::core::fmt::Debug for IRendezvousSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRendezvousSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRendezvousSession {
    type Vtable = IRendezvousSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ba4b1dd_8b0c_48b7_9e7c_2f25857c8df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRendezvousSession_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows_core::HRESULT,
    pub RemoteUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT,
    pub SendContextData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RENDEZVOUS_SESSION_FLAGS(pub i32);
pub const RSF_NONE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(0i32);
pub const RSF_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(1i32);
pub const RSF_INVITEE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(2i32);
pub const RSF_ORIGINAL_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(4i32);
pub const RSF_REMOTE_LEGACYSESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(8i32);
pub const RSF_REMOTE_WIN7SESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(16i32);
impl ::core::marker::Copy for RENDEZVOUS_SESSION_FLAGS {}
impl ::core::clone::Clone for RENDEZVOUS_SESSION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RENDEZVOUS_SESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RENDEZVOUS_SESSION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RENDEZVOUS_SESSION_STATE(pub i32);
pub const RSS_UNKNOWN: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(0i32);
pub const RSS_READY: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(1i32);
pub const RSS_INVITATION: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(2i32);
pub const RSS_ACCEPTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(3i32);
pub const RSS_CONNECTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(4i32);
pub const RSS_CANCELLED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(5i32);
pub const RSS_DECLINED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(6i32);
pub const RSS_TERMINATED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(7i32);
impl ::core::marker::Copy for RENDEZVOUS_SESSION_STATE {}
impl ::core::clone::Clone for RENDEZVOUS_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RENDEZVOUS_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RENDEZVOUS_SESSION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RENDEZVOUS_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_STATE").field(&self.0).finish()
    }
}
pub const RendezvousApplication: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b7e019a_b5de_47fa_8966_9082f82fb192);
