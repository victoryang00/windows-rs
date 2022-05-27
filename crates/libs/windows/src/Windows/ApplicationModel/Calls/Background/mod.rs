#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallBlockedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4a690a2_e4c1_427f_864e_e470477ddb67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallBlockedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallBlockedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallBlockedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e9b5b3f_c54b_4e82_4cc9_e329a4184592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad30276_83b6_5732_9c38_0c206546196a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DismissalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DismissalTime: usize,
    pub TextReplyMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b0e6044_9b32_5d42_8222_d2812e39fb21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineChangedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6d321e7_d11d_40d8_b2b7_e40a01d66249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineChangedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineChangeKind) -> ::windows_core::HRESULT,
    pub HasLinePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13a8c01b_b831_48d3_8ba9_8d22a6580dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub OperatorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: Self = Self(0i32);
    pub const PrivateNumber: Self = Self(1i32);
    pub const UnknownNumber: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallBlockedReason {}
impl ::core::clone::Clone for PhoneCallBlockedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallBlockedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneCallBlockedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallBlockedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallBlockedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(::windows_core::IUnknown);
impl PhoneCallBlockedTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn CallBlockedReason(&self) -> ::windows_core::Result<PhoneCallBlockedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallBlockedReason>::zeroed();
            (::windows_core::Interface::vtable(this).CallBlockedReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallBlockedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallBlockedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallBlockedTriggerDetails {}
impl ::core::fmt::Debug for PhoneCallBlockedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallBlockedTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails;{a4a690a2-e4c1-427f-864e-e470477ddb67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneCallBlockedTriggerDetails {
    type Vtable = IPhoneCallBlockedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneCallBlockedTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallBlockedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneCallBlockedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallBlockedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneCallBlockedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneCallBlockedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallBlockedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallBlockedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(::windows_core::IUnknown);
impl PhoneCallOriginDataRequestTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn RequestId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallOriginDataRequestTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallOriginDataRequestTriggerDetails {}
impl ::core::fmt::Debug for PhoneCallOriginDataRequestTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOriginDataRequestTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallOriginDataRequestTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails;{6e9b5b3f-c54b-4e82-4cc9-e329a4184592})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneCallOriginDataRequestTriggerDetails {
    type Vtable = IPhoneCallOriginDataRequestTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneCallOriginDataRequestTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallOriginDataRequestTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneCallOriginDataRequestTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallOriginDataRequestTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneCallOriginDataRequestTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneCallOriginDataRequestTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallOriginDataRequestTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneCallOriginDataRequestTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneIncomingCallDismissedReason {}
impl ::core::clone::Clone for PhoneIncomingCallDismissedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneIncomingCallDismissedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneIncomingCallDismissedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneIncomingCallDismissedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneIncomingCallDismissedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(::windows_core::IUnknown);
impl PhoneIncomingCallDismissedTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DismissalTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DismissalTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn TextReplyMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TextReplyMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn Reason(&self) -> ::windows_core::Result<PhoneIncomingCallDismissedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneIncomingCallDismissedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneIncomingCallDismissedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallDismissedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallDismissedTriggerDetails {}
impl ::core::fmt::Debug for PhoneIncomingCallDismissedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneIncomingCallDismissedTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails;{bad30276-83b6-5732-9c38-0c206546196a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneIncomingCallDismissedTriggerDetails {
    type Vtable = IPhoneIncomingCallDismissedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneIncomingCallDismissedTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneIncomingCallDismissedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneIncomingCallDismissedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallDismissedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneIncomingCallDismissedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneIncomingCallDismissedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallDismissedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallDismissedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(::windows_core::IUnknown);
impl PhoneIncomingCallNotificationTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn CallId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneIncomingCallNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneIncomingCallNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails;{2b0e6044-9b32-5d42-8222-d2812e39fb21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneIncomingCallNotificationTriggerDetails {
    type Vtable = IPhoneIncomingCallNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneIncomingCallNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneIncomingCallNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneIncomingCallNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneIncomingCallNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneIncomingCallNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneIncomingCallNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneIncomingCallNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneIncomingCallNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const PropertiesChanged: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineChangeKind {}
impl ::core::clone::Clone for PhoneLineChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneLineChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneLineChangeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(::windows_core::IUnknown);
impl PhoneLineChangedTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn ChangeType(&self) -> ::windows_core::Result<PhoneLineChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneLineChangeKind>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneLineChangeKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasLinePropertyChanged)(::windows_core::Interface::as_raw(this), lineproperty, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineChangedTriggerDetails {}
impl ::core::fmt::Debug for PhoneLineChangedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangedTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneLineChangedTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails;{c6d321e7-d11d-40d8-b2b7-e40a01d66249})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneLineChangedTriggerDetails {
    type Vtable = IPhoneLineChangedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneLineChangedTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineChangedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneLineChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineChangedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneLineChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneLineChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneLineChangedTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneLineChangedTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: Self = Self(0u32);
    pub const BrandingOptions: Self = Self(1u32);
    pub const CanDial: Self = Self(2u32);
    pub const CellularDetails: Self = Self(4u32);
    pub const DisplayColor: Self = Self(8u32);
    pub const DisplayName: Self = Self(16u32);
    pub const NetworkName: Self = Self(32u32);
    pub const NetworkState: Self = Self(64u32);
    pub const Transport: Self = Self(128u32);
    pub const Voicemail: Self = Self(256u32);
}
impl ::core::marker::Copy for PhoneLineProperties {}
impl ::core::clone::Clone for PhoneLineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineProperties {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneLineProperties {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineProperties").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PhoneLineProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PhoneLineProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneLineProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneLineProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PhoneLineProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneLineProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneLineProperties;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(::windows_core::IUnknown);
impl PhoneNewVoicemailMessageTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn VoicemailCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).VoicemailCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    pub fn OperatorMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OperatorMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNewVoicemailMessageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNewVoicemailMessageTriggerDetails {}
impl ::core::fmt::Debug for PhoneNewVoicemailMessageTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNewVoicemailMessageTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNewVoicemailMessageTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails;{13a8c01b-b831-48d3-8ba9-8d22a6580dcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneNewVoicemailMessageTriggerDetails {
    type Vtable = IPhoneNewVoicemailMessageTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneNewVoicemailMessageTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNewVoicemailMessageTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneNewVoicemailMessageTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNewVoicemailMessageTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneNewVoicemailMessageTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneNewVoicemailMessageTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneNewVoicemailMessageTriggerDetails {}
unsafe impl ::core::marker::Sync for PhoneNewVoicemailMessageTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: Self = Self(0i32);
    pub const CallHistoryChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: Self = Self(3i32);
    pub const CallOriginDataRequest: Self = Self(4i32);
    pub const CallBlocked: Self = Self(5i32);
    pub const IncomingCallDismissed: Self = Self(6i32);
    pub const IncomingCallNotification: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneTriggerType {}
impl ::core::clone::Clone for PhoneTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneTriggerType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneTriggerType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneTriggerType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.Background.PhoneTriggerType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
