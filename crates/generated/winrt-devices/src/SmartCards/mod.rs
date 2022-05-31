#[repr(transparent)]
pub struct CardAddedEventArgs(::windows_core::IUnknown);
impl CardAddedEventArgs {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCard>(result__)
        }
    }
}
impl ::core::clone::Clone for CardAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CardAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CardAddedEventArgs {}
impl ::core::fmt::Debug for CardAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CardAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CardAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardAddedEventArgs;{18bbef98-f18b-4dd3-b118-dfb2c8e23cc6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICardAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CardAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardAddedEventArgs";
}
impl ::core::convert::From<CardAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CardAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CardAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CardAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CardAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CardAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CardAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CardAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CardAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CardAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CardAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CardAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CardAddedEventArgs {}
unsafe impl ::core::marker::Sync for CardAddedEventArgs {}
#[repr(transparent)]
pub struct CardRemovedEventArgs(::windows_core::IUnknown);
impl CardRemovedEventArgs {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCard>(result__)
        }
    }
}
impl ::core::clone::Clone for CardRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CardRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CardRemovedEventArgs {}
impl ::core::fmt::Debug for CardRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CardRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CardRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.CardRemovedEventArgs;{15331aaf-22d7-4945-afc9-03b46f42a6cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICardRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CardRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardRemovedEventArgs";
}
impl ::core::convert::From<CardRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CardRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CardRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CardRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CardRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CardRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CardRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CardRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CardRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CardRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CardRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CardRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CardRemovedEventArgs {}
unsafe impl ::core::marker::Sync for CardRemovedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICardAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18bbef98_f18b_4dd3_b118_dfb2c8e23cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICardRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15331aaf_22d7_4945_afc9_03b46f42a6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICardRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSmartCardAppletIds(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownSmartCardAppletIds {
    type Vtable = IKnownSmartCardAppletIds_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b04d8d8_95b4_4c88_8cea_411e55511efc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSmartCardAppletIds_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PaymentSystemEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PaymentSystemEnvironment: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProximityPaymentSystemEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProximityPaymentSystemEnvironment: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCard(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCard {
    type Vtable = ISmartCard_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b718871_6434_43f4_b55a_6a29623870aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCard_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAnswerToResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAnswerToResetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7db165e6_6264_56f4_5e03_c86385395eb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppletIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppletIds: usize,
    pub SmartCardEmulationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationCategory) -> ::windows_core::HRESULT,
    pub SetSmartCardEmulationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardEmulationCategory) -> ::windows_core::HRESULT,
    pub SmartCardEmulationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationType) -> ::windows_core::HRESULT,
    pub SetSmartCardEmulationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardEmulationType) -> ::windows_core::HRESULT,
    pub AutomaticEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticEnablement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroup2 {
    type Vtable = ISmartCardAppletIdGroup2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b0ef9dc_9956_4a62_8d4e_d37a68ebc3a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroup2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub SecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSecureUserAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupFactory {
    type Vtable = ISmartCardAppletIdGroupFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9105eb4d_4a65_4e41_8061_cbe83f3695e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appletids: ::windows_core::RawPtr, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf1208d1_31bb_5596_43b1_6d69a0257b3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAppletIdGroupActivationPolicy) -> ::windows_core::HRESULT,
    pub AppletIdGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestActivationPolicyChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: SmartCardAppletIdGroupActivationPolicy, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAutomaticResponseApdusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apdus: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAutomaticResponseApdusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupRegistration2 {
    type Vtable = ISmartCardAppletIdGroupRegistration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f5508d8_98a7_4f2e_91d9_6cfcceda407f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupRegistration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmartCardReaderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAppletIdGroupStatics {
    type Vtable = ISmartCardAppletIdGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2899a9_e76c_45cf_bf1d_90eaa6205927);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAppletIdGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxAppletIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52152bab_c63e_4531_a857_d756d99b986a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApduBitMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApduBitMask: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApduBitMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApduBitMask: usize,
    pub ShouldMatchLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldMatchLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponseApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponseApdu: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu2 {
    type Vtable = ISmartCardAutomaticResponseApdu2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44aebb14_559d_4531_4e51_89db6fa8a57a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OutputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOutputState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApdu3 {
    type Vtable = ISmartCardAutomaticResponseApdu3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf43da74_6576_4392_9367_fe3bc9e2d496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApdu3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApduFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardAutomaticResponseApduFactory {
    type Vtable = ISmartCardAutomaticResponseApduFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe97ea2fa_d02c_4c55_b02a_8cff7fa9f05b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardAutomaticResponseApduFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandapdu: ::windows_core::RawPtr, responseapdu: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardChallengeContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x192a5319_c9c4_4947_81cc_44794a61ef91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardChallengeContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifyResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifyResponseAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows_core::RawPtr, formatcard: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProvisionAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProvisionAsyncWithNewCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows_core::RawPtr, formatcard: bool, newcardid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProvisionAsyncWithNewCardId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ChangeAdministrativeKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows_core::RawPtr, newadministrativekey: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ChangeAdministrativeKeyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardConnect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardConnect {
    type Vtable = ISmartCardConnect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fdf87e5_028d_491e_a058_3382c3986f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardConnection {
    type Vtable = ISmartCardConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7edb991a_a81a_47bc_a649_156be6b7f231);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TransmitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TransmitAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe39f587b_edd3_4e49_b594_0ff5e4d0c76f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageConfirmationResponseFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageConfirmationResponseFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSmartCardCryptogramStorageKeyCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSmartCardCryptogramStorageKeyCapabilities: usize,
    pub DeleteCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagekeyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub RequestCryptogramMaterialStorageKeyInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, format: ::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    RequestCryptogramMaterialStorageKeyInfoAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, materialpackagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, cryptogrammaterialpackage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportCryptogramMaterialPackageAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TryProvePossessionOfCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, materialname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, challenge: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryProvePossessionOfCryptogramMaterialPackageAsync: usize,
    pub RequestUnlockCryptogramMaterialForUseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteCryptogramMaterialPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialpackagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGenerator2 {
    type Vtable = ISmartCardCryptogramGenerator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7116aa34_5d6d_4b4a_96a3_efa47d2a7e25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGenerator2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ValidateRequestApduAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: ::windows_core::RawPtr, cryptogramplacementsteps: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ValidateRequestApduAsync: usize,
    pub GetAllCryptogramStorageKeyCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAllCryptogramMaterialPackageCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagekeyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAllCryptogramMaterialCharacteristicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGeneratorStatics {
    type Vtable = ISmartCardCryptogramGeneratorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09933910_cb9c_4015_967d_5234f3b02900);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetSmartCardCryptogramGeneratorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGeneratorStatics2 {
    type Vtable = ISmartCardCryptogramGeneratorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09bdf5e5_b4bd_4e23_a588_74469204c128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGeneratorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2798e029_d687_4c92_86c6_399e9a0ecb09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e6a8a5c_9773_46c4_a32f_b1e543159e04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c7ce857_a7e7_489d_b9d6_368061515012);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc9ac5cc_c1d7_4153_923b_a2d43c6c8d49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedProofOfPossessionAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedProofOfPossessionAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedValidations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedValidations: usize,
    pub MaterialType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialType) -> ::windows_core::HRESULT,
    pub ProtectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialProtectionMethod) -> ::windows_core::HRESULT,
    pub ProtectionVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaterialLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffb58e1f_0692_4c47_93cf_34d91f9dcd00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DateImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub PackageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialPackageFormat) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPossessionProof(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5b9ab8c_a141_4135_9add_b0d2e3aa1fc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramMaterialPossessionProof_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Proof: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Proof: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramPlacementStep(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x947b03eb_8342_4792_a2e5_925636378a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramPlacementStep_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Algorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramAlgorithm) -> ::windows_core::HRESULT,
    pub SetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramAlgorithm) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceData: usize,
    pub CryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCryptogramMaterialPackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CryptogramMaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCryptogramMaterialName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TemplateOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTemplateOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCryptogramOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCryptogramLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub CryptogramPlacementOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramPlacementOptions) -> ::windows_core::HRESULT,
    pub SetCryptogramPlacementOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramPlacementOptions) -> ::windows_core::HRESULT,
    pub ChainedOutputStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetChainedOutputStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8552546e_4457_4825_b464_635471a39f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StorageKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DateCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Algorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyAlgorithm) -> ::windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77b0f00d_b097_4f61_a26a_9561639c9c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub PublicKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    PublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublicKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublicKey: usize,
    pub AttestationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptographicKeyAttestationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Attestation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Attestation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationCertificateChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationCertificateChain: usize,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardCryptogramStorageKeyInfo2 {
    type Vtable = ISmartCardCryptogramStorageKeyInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000440f9_f7fd_417d_89e1_fbb0382adc4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardCryptogramStorageKeyInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OperationalRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulator {
    type Vtable = ISmartCardEmulator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfb906b2_875e_47e5_8077_e8bff1b1c6fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnablementPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulator2 {
    type Vtable = ISmartCardEmulator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe3fc0b8_8529_411a_807b_48edc2a0ab44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulator2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ApduReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveApduReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ConnectionDeactivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveConnectionDeactivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsHostCardEmulationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd55d1576_69d2_5333_5b5f_f8c0d6e9f09f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryRespondAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseapdu: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryRespondAsync: usize,
    pub AutomaticResponseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAutomaticResponseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgs2 {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bf93df0_22e1_4238_8610_94ce4a965425);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryRespondWithStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseapdu: ::windows_core::RawPtr, nextstate: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryRespondWithStateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd550bac7_b7bf_4e29_9294_0c4ac3c941bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responsetemplate: ::windows_core::RawPtr, cryptogramplacementsteps: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAndStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responsetemplate: ::windows_core::RawPtr, cryptogramplacementsteps: ::windows_core::RawPtr, nextstate: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAndStateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2186d8d3_c5eb_5262_43df_62a0a1b55557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e2ca5ee_f969_507d_6cf9_34e2d18df311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorConnectionProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics {
    type Vtable = ISmartCardEmulatorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a9bfc4b_c4d3_494f_b8a2_6215d81e85b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics2 {
    type Vtable = ISmartCardEmulatorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69ae9f8a_b775_488b_8436_6c1e28ed731f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppletIdGroupRegistrationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppletIdGroupRegistrationsAsync: usize,
    pub RegisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appletidgroup: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnregisterAppletIdGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxAppletIdGroupRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardEmulatorStatics3 {
    type Vtable = ISmartCardEmulatorStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59ea142a_9f09_43f5_8565_cfa8148e4cb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardEmulatorStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinPolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x183ce184_4db6_4841_ac9e_2ac1f39b7304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinPolicy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub UppercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetUppercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub LowercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetLowercaseLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub Digits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SpecialCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
    pub SetSpecialCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinResetDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18c94aac_7805_4004_85e4_bbefac8f6884);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardPinResetRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12fe3c4d_5fb9_4e8e_9ff6_61f475124fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardPinResetRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioning(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19eeedbd_1fab_477c_b712_1a2c5af1fd6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetChallengeContextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPinChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPinResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioning2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioning2 {
    type Vtable = ISmartCardProvisioning2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10fd28eb_3f79_4b66_9b7c_11c149b7d0bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioning2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAuthorityKeyContainerNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioningStatics {
    type Vtable = ISmartCardProvisioningStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13882848_0d13_4e70_9735_51daeca5254f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromSmartCardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, card: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RequestVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, administrativekey: ::windows_core::RawPtr, pinpolicy: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestVirtualSmartCardCreationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RequestVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, administrativekey: ::windows_core::RawPtr, pinpolicy: ::windows_core::RawPtr, cardid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestVirtualSmartCardCreationAsyncWithCardId: usize,
    pub RequestVirtualSmartCardDeletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, card: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardProvisioningStatics2 {
    type Vtable = ISmartCardProvisioningStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3447c6a8_c9a0_4bd6_b50d_251f4e8d3a62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardProvisioningStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub RequestAttestedVirtualSmartCardCreationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, administrativekey: ::windows_core::RawPtr, pinpolicy: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestAttestedVirtualSmartCardCreationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RequestAttestedVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, administrativekey: ::windows_core::RawPtr, pinpolicy: ::windows_core::RawPtr, cardid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestAttestedVirtualSmartCardCreationAsyncWithCardId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardReader {
    type Vtable = ISmartCardReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1074b4e0_54c2_4df0_817a_14c14378f06c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardReaderKind) -> ::windows_core::HRESULT,
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllCardsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllCardsAsync: usize,
    pub CardAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCardAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CardRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCardRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardReaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardReaderStatics {
    type Vtable = ISmartCardReaderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x103c04e1_a1ca_48f2_a281_5b6f669af107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardReaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SmartCardReaderKind, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f9bf11e_39ef_4f2b_b44f_0a9155b177bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmartCardTriggerType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceAppletId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TriggerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TriggerData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails2 {
    type Vtable = ISmartCardTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2945c569_8975_4a51_9e1a_5f8a76ee51af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Emulator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryLaunchCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryLaunchCurrentAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, behavior: SmartCardLaunchBehavior, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerDetails3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerDetails3 {
    type Vtable = ISmartCardTriggerDetails3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3e2c27d_18c6_4ba8_8376_ef03d4912666);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerDetails3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub struct KnownSmartCardAppletIds;
impl KnownSmartCardAppletIds {
    #[cfg(feature = "Storage_Streams")]
    pub fn PaymentSystemEnvironment() -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PaymentSystemEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProximityPaymentSystemEnvironment() -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProximityPaymentSystemEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        })
    }
    pub fn IKnownSmartCardAppletIds<R, F: FnOnce(&IKnownSmartCardAppletIds) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownSmartCardAppletIds, IKnownSmartCardAppletIds> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownSmartCardAppletIds {
    const NAME: &'static str = "Windows.Devices.SmartCards.KnownSmartCardAppletIds";
}
#[repr(transparent)]
pub struct SmartCard(::windows_core::IUnknown);
impl SmartCard {
    pub fn Reader(&self) -> ::windows_core::Result<SmartCardReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardReader>(result__)
        }
    }
    pub fn GetStatusAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardStatus>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAnswerToResetAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAnswerToResetAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn ConnectAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardConnection>> {
        let this = &::windows_core::Interface::cast::<ISmartCardConnect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardConnection>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCard {}
impl ::core::fmt::Debug for SmartCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCard").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCard {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCard;{1b718871-6434-43f4-b55a-6a29623870aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCard {
    type Vtable = ISmartCard_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCard as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCard {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCard";
}
impl ::core::convert::From<SmartCard> for ::windows_core::IUnknown {
    fn from(value: SmartCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCard> for ::windows_core::IUnknown {
    fn from(value: &SmartCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCard> for ::windows_core::IInspectable {
    fn from(value: SmartCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCard> for ::windows_core::IInspectable {
    fn from(value: &SmartCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCard {}
unsafe impl ::core::marker::Sync for SmartCard {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardActivationPolicyChangeResult {}
impl ::core::clone::Clone for SmartCardActivationPolicyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardActivationPolicyChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardActivationPolicyChangeResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardActivationPolicyChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardActivationPolicyChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardActivationPolicyChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardActivationPolicyChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroup(::windows_core::IUnknown);
impl SmartCardAppletIdGroup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardAppletIdGroup, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn AppletIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppletIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn SmartCardEmulationCategory(&self) -> ::windows_core::Result<SmartCardEmulationCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardEmulationCategory>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardEmulationCategory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulationCategory>(result__)
        }
    }
    pub fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmartCardEmulationCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmartCardEmulationType(&self) -> ::windows_core::Result<SmartCardEmulationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardEmulationType>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardEmulationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulationType>(result__)
        }
    }
    pub fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmartCardEmulationType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticEnablement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticEnablement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticEnablement(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticEnablement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn SecureUserAuthenticationRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SecureUserAuthenticationRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSecureUserAuthenticationRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSecureUserAuthenticationRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::winrt_storage::Streams::IBuffer>>>(displayname: Param0, appletids: Param1, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> ::windows_core::Result<SmartCardAppletIdGroup> {
        Self::ISmartCardAppletIdGroupFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), displayname.into_param().abi(), appletids.into_param().abi(), emulationcategory, emulationtype, result__.as_mut_ptr()).from_abi::<SmartCardAppletIdGroup>(result__)
        })
    }
    pub fn MaxAppletIds() -> ::windows_core::Result<u16> {
        Self::ISmartCardAppletIdGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAppletIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        })
    }
    pub fn ISmartCardAppletIdGroupFactory<R, F: FnOnce(&ISmartCardAppletIdGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardAppletIdGroupStatics<R, F: FnOnce(&ISmartCardAppletIdGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardAppletIdGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardAppletIdGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAppletIdGroup {}
impl ::core::fmt::Debug for SmartCardAppletIdGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardAppletIdGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroup;{7db165e6-6264-56f4-5e03-c86385395eb1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardAppletIdGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAppletIdGroup {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroup";
}
impl ::core::convert::From<SmartCardAppletIdGroup> for ::windows_core::IUnknown {
    fn from(value: SmartCardAppletIdGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroup> for ::windows_core::IUnknown {
    fn from(value: &SmartCardAppletIdGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardAppletIdGroup> for ::windows_core::IInspectable {
    fn from(value: SmartCardAppletIdGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroup> for ::windows_core::IInspectable {
    fn from(value: &SmartCardAppletIdGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardAppletIdGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardAppletIdGroup {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroup {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: Self = Self(0i32);
    pub const ForegroundOverride: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAppletIdGroupActivationPolicy {}
impl ::core::clone::Clone for SmartCardAppletIdGroupActivationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardAppletIdGroupActivationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardAppletIdGroupActivationPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardAppletIdGroupActivationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroupActivationPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardAppletIdGroupActivationPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAppletIdGroupActivationPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroupRegistration(::windows_core::IUnknown);
impl SmartCardAppletIdGroupRegistration {
    pub fn ActivationPolicy(&self) -> ::windows_core::Result<SmartCardAppletIdGroupActivationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardAppletIdGroupActivationPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).ActivationPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardAppletIdGroupActivationPolicy>(result__)
        }
    }
    pub fn AppletIdGroup(&self) -> ::windows_core::Result<SmartCardAppletIdGroup> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppletIdGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardAppletIdGroup>(result__)
        }
    }
    pub fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestActivationPolicyChangeAsync)(::windows_core::Interface::as_raw(this), policy, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAutomaticResponseApdusAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>>(&self, apdus: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAutomaticResponseApdusAsync)(::windows_core::Interface::as_raw(this), apdus.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SmartCardReaderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCardReaderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>>(&self, props: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertiesAsync)(::windows_core::Interface::as_raw(this), props.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardAppletIdGroupRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardAppletIdGroupRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAppletIdGroupRegistration {}
impl ::core::fmt::Debug for SmartCardAppletIdGroupRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroupRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardAppletIdGroupRegistration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration;{df1208d1-31bb-5596-43b1-6d69a0257b3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardAppletIdGroupRegistration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAppletIdGroupRegistration {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration";
}
impl ::core::convert::From<SmartCardAppletIdGroupRegistration> for ::windows_core::IUnknown {
    fn from(value: SmartCardAppletIdGroupRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroupRegistration> for ::windows_core::IUnknown {
    fn from(value: &SmartCardAppletIdGroupRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardAppletIdGroupRegistration> for ::windows_core::IInspectable {
    fn from(value: SmartCardAppletIdGroupRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAppletIdGroupRegistration> for ::windows_core::IInspectable {
    fn from(value: &SmartCardAppletIdGroupRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardAppletIdGroupRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardAppletIdGroupRegistration {}
unsafe impl ::core::marker::Sync for SmartCardAppletIdGroupRegistration {}
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(::windows_core::IUnknown);
impl SmartCardAutomaticResponseApdu {
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandApdu)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApdu<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommandApdu)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApduBitMask(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandApduBitMask)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApduBitMask<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommandApduBitMask)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ShouldMatchLength(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldMatchLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldMatchLength(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldMatchLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppletId(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppletId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAppletId<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppletId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseApdu(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseApdu)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponseApdu<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponseApdu)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn InputState(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetInputState<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInputState)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OutputState(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetOutputState<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOutputState)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowWhenCryptogramGeneratorNotPrepared)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowWhenCryptogramGeneratorNotPrepared)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(commandapdu: Param0, responseapdu: Param1) -> ::windows_core::Result<SmartCardAutomaticResponseApdu> {
        Self::ISmartCardAutomaticResponseApduFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), commandapdu.into_param().abi(), responseapdu.into_param().abi(), result__.as_mut_ptr()).from_abi::<SmartCardAutomaticResponseApdu>(result__)
        })
    }
    pub fn ISmartCardAutomaticResponseApduFactory<R, F: FnOnce(&ISmartCardAutomaticResponseApduFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardAutomaticResponseApdu, ISmartCardAutomaticResponseApduFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardAutomaticResponseApdu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardAutomaticResponseApdu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardAutomaticResponseApdu {}
impl ::core::fmt::Debug for SmartCardAutomaticResponseApdu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAutomaticResponseApdu").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardAutomaticResponseApdu {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu;{52152bab-c63e-4531-a857-d756d99b986a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardAutomaticResponseApdu as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardAutomaticResponseApdu {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu";
}
impl ::core::convert::From<SmartCardAutomaticResponseApdu> for ::windows_core::IUnknown {
    fn from(value: SmartCardAutomaticResponseApdu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAutomaticResponseApdu> for ::windows_core::IUnknown {
    fn from(value: &SmartCardAutomaticResponseApdu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardAutomaticResponseApdu> for ::windows_core::IInspectable {
    fn from(value: SmartCardAutomaticResponseApdu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardAutomaticResponseApdu> for ::windows_core::IInspectable {
    fn from(value: &SmartCardAutomaticResponseApdu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardAutomaticResponseApdu {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardAutomaticResponseApdu {}
unsafe impl ::core::marker::Sync for SmartCardAutomaticResponseApdu {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const UnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAutomaticResponseStatus {}
impl ::core::clone::Clone for SmartCardAutomaticResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardAutomaticResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardAutomaticResponseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardAutomaticResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardAutomaticResponseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardAutomaticResponseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAutomaticResponseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardChallengeContext(::windows_core::IUnknown);
impl SmartCardChallengeContext {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Challenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifyResponseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, response: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerifyResponseAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProvisionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, response: Param0, formatcard: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), formatcard, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProvisionAsyncWithNewCardId<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, response: Param0, formatcard: bool, newcardid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionAsyncWithNewCardId)(::windows_core::Interface::as_raw(this), response.into_param().abi(), formatcard, newcardid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ChangeAdministrativeKeyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, response: Param0, newadministrativekey: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAdministrativeKeyAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), newadministrativekey.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardChallengeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardChallengeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardChallengeContext {}
impl ::core::fmt::Debug for SmartCardChallengeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardChallengeContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardChallengeContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardChallengeContext;{192a5319-c9c4-4947-81cc-44794a61ef91})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardChallengeContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardChallengeContext {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardChallengeContext";
}
impl ::core::convert::From<SmartCardChallengeContext> for ::windows_core::IUnknown {
    fn from(value: SmartCardChallengeContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardChallengeContext> for ::windows_core::IUnknown {
    fn from(value: &SmartCardChallengeContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardChallengeContext> for ::windows_core::IInspectable {
    fn from(value: SmartCardChallengeContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardChallengeContext> for ::windows_core::IInspectable {
    fn from(value: &SmartCardChallengeContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmartCardChallengeContext> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SmartCardChallengeContext) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmartCardChallengeContext> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmartCardChallengeContext) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SmartCardChallengeContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmartCardChallengeContext {}
unsafe impl ::core::marker::Sync for SmartCardChallengeContext {}
#[repr(transparent)]
pub struct SmartCardConnection(::windows_core::IUnknown);
impl SmartCardConnection {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TransmitAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, command: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransmitAsync)(::windows_core::Interface::as_raw(this), command.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IBuffer>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardConnection {}
impl ::core::fmt::Debug for SmartCardConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardConnection;{7edb991a-a81a-47bc-a649-156be6b7f231})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardConnection {
    type Vtable = ISmartCardConnection_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardConnection {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardConnection";
}
impl ::core::convert::From<SmartCardConnection> for ::windows_core::IUnknown {
    fn from(value: SmartCardConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardConnection> for ::windows_core::IUnknown {
    fn from(value: &SmartCardConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardConnection> for ::windows_core::IInspectable {
    fn from(value: SmartCardConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardConnection> for ::windows_core::IInspectable {
    fn from(value: &SmartCardConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SmartCardConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SmartCardConnection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SmartCardConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SmartCardConnection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SmartCardConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SmartCardConnection {}
unsafe impl ::core::marker::Sync for SmartCardConnection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: Self = Self(0i32);
    pub const CbcMac: Self = Self(1i32);
    pub const Cvc3Umd: Self = Self(2i32);
    pub const DecimalizedMsd: Self = Self(3i32);
    pub const Cvc3MD: Self = Self(4i32);
    pub const Sha1: Self = Self(5i32);
    pub const SignedDynamicApplicationData: Self = Self(6i32);
    pub const RsaPkcs1: Self = Self(7i32);
    pub const Sha256Hmac: Self = Self(8i32);
}
impl ::core::marker::Copy for SmartCardCryptogramAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGenerator(::windows_core::IUnknown);
impl SmartCardCryptogramGenerator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialTypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramAlgorithms(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramAlgorithms)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageFormats(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageConfirmationResponseFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedSmartCardCryptogramStorageKeyCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>>(result__)
        }
    }
    pub fn DeleteCryptogramMaterialStorageKeyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storagekeyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteCryptogramMaterialStorageKeyAsync)(::windows_core::Interface::as_raw(this), storagekeyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    pub fn CreateCryptogramMaterialStorageKeyAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: Param1, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCryptogramMaterialStorageKeyAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, storagekeyname.into_param().abi(), algorithm, capabilities, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn RequestCryptogramMaterialStorageKeyInfoAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: Param1, format: ::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestCryptogramMaterialStorageKeyInfoAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, storagekeyname.into_param().abi(), format, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportCryptogramMaterialPackageAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: Param1, materialpackagename: Param2, cryptogrammaterialpackage: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), format, storagekeyname.into_param().abi(), materialpackagename.into_param().abi(), cryptogrammaterialpackage.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryProvePossessionOfCryptogramMaterialPackageAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: Param2, materialname: Param3, challenge: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryProvePossessionOfCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, responseformat, materialpackagename.into_param().abi(), materialname.into_param().abi(), challenge.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>(result__)
        }
    }
    pub fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnlockCryptogramMaterialForUseAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    pub fn DeleteCryptogramMaterialPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, materialpackagename: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteCryptogramMaterialPackageAsync)(::windows_core::Interface::as_raw(this), materialpackagename.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ValidateRequestApduAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: Param1, cryptogramplacementsteps: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateRequestApduAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, apdutovalidate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    pub fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramStorageKeyCharacteristicsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>>(result__)
        }
    }
    pub fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>(result__)
        }
    }
    pub fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storagekeyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync)(::windows_core::Interface::as_raw(this), storagekeyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>(result__)
        }
    }
    pub fn GetAllCryptogramMaterialCharacteristicsAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllCryptogramMaterialCharacteristicsAsync)(::windows_core::Interface::as_raw(this), promptingbehavior, materialpackagename.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>>(result__)
        }
    }
    pub fn GetSmartCardCryptogramGeneratorAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGenerator>> {
        Self::ISmartCardCryptogramGeneratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSmartCardCryptogramGeneratorAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGenerator>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISmartCardCryptogramGeneratorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISmartCardCryptogramGeneratorStatics<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardCryptogramGeneratorStatics2<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGenerator {}
impl ::core::fmt::Debug for SmartCardCryptogramGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramGenerator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGenerator;{e39f587b-edd3-4e49-b594-0ff5e4d0c76f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGenerator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGenerator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGenerator";
}
impl ::core::convert::From<SmartCardCryptogramGenerator> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGenerator> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramGenerator> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGenerator> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGenerator {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGenerator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const AuthorizationFailed: Self = Self(1i32);
    pub const AuthorizationCanceled: Self = Self(2i32);
    pub const AuthorizationRequired: Self = Self(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: Self = Self(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: Self = Self(5i32);
    pub const NoCryptogramMaterialPackage: Self = Self(6i32);
    pub const UnsupportedCryptogramMaterialPackage: Self = Self(7i32);
    pub const UnknownCryptogramMaterialName: Self = Self(8i32);
    pub const InvalidCryptogramMaterialUsage: Self = Self(9i32);
    pub const ApduResponseNotSent: Self = Self(10i32);
    pub const OtherError: Self = Self(11i32);
    pub const ValidationFailed: Self = Self(12i32);
    pub const NotSupported: Self = Self(13i32);
}
impl ::core::marker::Copy for SmartCardCryptogramGeneratorOperationStatus {}
impl ::core::clone::Clone for SmartCardCryptogramGeneratorOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramGeneratorOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramGeneratorOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramGeneratorOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGeneratorOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramGeneratorOperationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramGeneratorOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramGeneratorOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult;{2798e029-d687-4c92-86c6-399e9a0ecb09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramGeneratorOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult;{4e6a8a5c-9773-46c4-a32f-b1e543159e04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(::windows_core::IUnknown);
impl SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramGeneratorOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Characteristics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
impl ::core::fmt::Debug for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult;{8c7ce857-a7e7-489d-b9d6-368061515012})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult";
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramMaterialCharacteristics, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MaterialName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MaterialName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedAlgorithms(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedAlgorithms)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedProofOfPossessionAlgorithms(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedProofOfPossessionAlgorithms)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedValidations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedValidations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>(result__)
        }
    }
    pub fn MaterialType(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramMaterialType>::zeroed();
            (::windows_core::Interface::vtable(this).MaterialType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramMaterialType>(result__)
        }
    }
    pub fn ProtectionMethod(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialProtectionMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramMaterialProtectionMethod>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionMethod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramMaterialProtectionMethod>(result__)
        }
    }
    pub fn ProtectionVersion(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaterialLength(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaterialLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialCharacteristics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialCharacteristics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics;{fc9ac5cc-c1d7-4153-923b-a2d43c6c8d49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialCharacteristics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramMaterialCharacteristics> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramMaterialCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialCharacteristics> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialCharacteristics> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramMaterialCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialCharacteristics> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramMaterialCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialCharacteristics {}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialPackageCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramMaterialPackageCharacteristics, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PackageName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn StorageKeyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StorageKeyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DateImported(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DateImported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn PackageFormat(&self) -> ::windows_core::Result<SmartCardCryptogramMaterialPackageFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramMaterialPackageFormat>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramMaterialPackageFormat>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialPackageCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialPackageCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageCharacteristics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageCharacteristics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics;{ffb58e1f-0692-4c47-93cf-34d91f9dcd00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialPackageCharacteristics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialPackageCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramMaterialPackageCharacteristics> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPackageCharacteristics> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialPackageCharacteristics> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPackageCharacteristics> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialPackageCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramMaterialPackageCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPackageCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPackageCharacteristics {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: Self = Self(0i32);
    pub const VisaHmac: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageConfirmationResponseFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageConfirmationResponseFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: Self = Self(0i32);
    pub const JweRsaPki: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialPackageFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramMaterialPackageFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPackageFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPackageFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPossessionProof(::windows_core::IUnknown);
impl SmartCardCryptogramMaterialPossessionProof {
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramGeneratorOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Proof(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Proof)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPossessionProof {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramMaterialPossessionProof {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramMaterialPossessionProof {}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialPossessionProof {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPossessionProof").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialPossessionProof {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof;{e5b9ab8c-a141-4135-9add-b0d2e3aa1fc9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramMaterialPossessionProof as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramMaterialPossessionProof {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof";
}
impl ::core::convert::From<SmartCardCryptogramMaterialPossessionProof> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramMaterialPossessionProof) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPossessionProof> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramMaterialPossessionProof) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramMaterialPossessionProof> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramMaterialPossessionProof) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramMaterialPossessionProof> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramMaterialPossessionProof) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramMaterialPossessionProof {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramMaterialPossessionProof {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramMaterialPossessionProof {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: Self = Self(0i32);
    pub const WhiteBoxing: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialProtectionMethod {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialProtectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialProtectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramMaterialProtectionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialProtectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialProtectionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialProtectionMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialProtectionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: Self = Self(0i32);
    pub const StaticDataAuthentication: Self = Self(1i32);
    pub const TripleDes112: Self = Self(2i32);
    pub const Aes: Self = Self(3i32);
    pub const RsaPkcs1: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialType {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramMaterialType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramMaterialType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramMaterialType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramMaterialType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: Self = Self(0u32);
    pub const UnitsAreInNibbles: Self = Self(1u32);
    pub const ChainOutput: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramPlacementOptions {}
impl ::core::clone::Clone for SmartCardCryptogramPlacementOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramPlacementOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramPlacementOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramPlacementOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramPlacementOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramPlacementOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramPlacementOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramPlacementOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramPlacementOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementStep(::windows_core::IUnknown);
impl SmartCardCryptogramPlacementStep {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramPlacementStep, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Algorithm(&self) -> ::windows_core::Result<SmartCardCryptogramAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).Algorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramAlgorithm>(result__)
        }
    }
    pub fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceData<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CryptogramMaterialPackageName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramMaterialPackageName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCryptogramMaterialPackageName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramMaterialPackageName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CryptogramMaterialName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramMaterialName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCryptogramMaterialName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramMaterialName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TemplateOffset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TemplateOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetTemplateOffset(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTemplateOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramOffset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetCryptogramOffset(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramLength(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetCryptogramLength(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramPlacementOptions(&self) -> ::windows_core::Result<SmartCardCryptogramPlacementOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramPlacementOptions>::zeroed();
            (::windows_core::Interface::vtable(this).CryptogramPlacementOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramPlacementOptions>(result__)
        }
    }
    pub fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCryptogramPlacementOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChainedOutputStep(&self) -> ::windows_core::Result<SmartCardCryptogramPlacementStep> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChainedOutputStep)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramPlacementStep>(result__)
        }
    }
    pub fn SetChainedOutputStep<'a, Param0: ::windows_core::IntoParam<'a, SmartCardCryptogramPlacementStep>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChainedOutputStep)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramPlacementStep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramPlacementStep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramPlacementStep {}
impl ::core::fmt::Debug for SmartCardCryptogramPlacementStep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramPlacementStep").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramPlacementStep {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep;{947b03eb-8342-4792-a2e5-925636378a53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramPlacementStep as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramPlacementStep {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep";
}
impl ::core::convert::From<SmartCardCryptogramPlacementStep> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramPlacementStep) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramPlacementStep> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramPlacementStep) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramPlacementStep> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramPlacementStep) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramPlacementStep> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramPlacementStep) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramPlacementStep {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramPlacementStep {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramPlacementStep {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: Self = Self(0i32);
    pub const Rsa2048: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramStorageKeyAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramStorageKeyAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: Self = Self(0u32);
    pub const HardwareProtection: Self = Self(1u32);
    pub const UnlockPrompt: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyCapabilities {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptogramStorageKeyCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptogramStorageKeyCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(::windows_core::IUnknown);
impl SmartCardCryptogramStorageKeyCharacteristics {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardCryptogramStorageKeyCharacteristics, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StorageKeyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StorageKeyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DateCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DateCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Algorithm(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramStorageKeyAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).Algorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramStorageKeyAlgorithm>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramStorageKeyCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramStorageKeyCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCharacteristics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramStorageKeyCharacteristics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramStorageKeyCharacteristics {}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyCharacteristics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyCharacteristics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics;{8552546e-4457-4825-b464-635471a39f5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramStorageKeyCharacteristics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramStorageKeyCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics";
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyCharacteristics> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyCharacteristics> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyCharacteristics> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyCharacteristics> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramStorageKeyCharacteristics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramStorageKeyCharacteristics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyCharacteristics {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyCharacteristics {}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(::windows_core::IUnknown);
impl SmartCardCryptogramStorageKeyInfo {
    pub fn OperationStatus(&self) -> ::windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramGeneratorOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).OperationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramGeneratorOperationStatus>(result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn PublicKeyBlobType(&self) -> ::windows_core::Result<::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType>::zeroed();
            (::windows_core::Interface::vtable(this).PublicKeyBlobType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Cryptography::Core::CryptographicPublicKeyBlobType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublicKey(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PublicKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn AttestationStatus(&self) -> ::windows_core::Result<SmartCardCryptographicKeyAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptographicKeyAttestationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).AttestationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptographicKeyAttestationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Attestation(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attestation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationCertificateChain(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttestationCertificateChain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Capabilities(&self) -> ::windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardCryptogramStorageKeyCapabilities>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardCryptogramStorageKeyCapabilities>(result__)
        }
    }
    pub fn OperationalRequirements(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISmartCardCryptogramStorageKeyInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OperationalRequirements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardCryptogramStorageKeyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardCryptogramStorageKeyInfo {}
impl ::core::fmt::Debug for SmartCardCryptogramStorageKeyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptogramStorageKeyInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo;{77b0f00d-b097-4f61-a26a-9561639c9c3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardCryptogramStorageKeyInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardCryptogramStorageKeyInfo {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo";
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyInfo> for ::windows_core::IUnknown {
    fn from(value: SmartCardCryptogramStorageKeyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyInfo> for ::windows_core::IUnknown {
    fn from(value: &SmartCardCryptogramStorageKeyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardCryptogramStorageKeyInfo> for ::windows_core::IInspectable {
    fn from(value: SmartCardCryptogramStorageKeyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardCryptogramStorageKeyInfo> for ::windows_core::IInspectable {
    fn from(value: &SmartCardCryptogramStorageKeyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardCryptogramStorageKeyInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardCryptogramStorageKeyInfo {}
unsafe impl ::core::marker::Sync for SmartCardCryptogramStorageKeyInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: Self = Self(0i32);
    pub const SoftwareKeyWithoutTpm: Self = Self(1i32);
    pub const SoftwareKeyWithTpm: Self = Self(2i32);
    pub const TpmKeyUnknownAttestationStatus: Self = Self(3i32);
    pub const TpmKeyWithoutAttestationCapability: Self = Self(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: Self = Self(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: Self = Self(6i32);
    pub const TpmKeyWithAttestation: Self = Self(7i32);
}
impl ::core::marker::Copy for SmartCardCryptographicKeyAttestationStatus {}
impl ::core::clone::Clone for SmartCardCryptographicKeyAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardCryptographicKeyAttestationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardCryptographicKeyAttestationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardCryptographicKeyAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardCryptographicKeyAttestationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardCryptographicKeyAttestationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptographicKeyAttestationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: Self = Self(0i32);
    pub const Payment: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulationCategory {}
impl ::core::clone::Clone for SmartCardEmulationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulationCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardEmulationCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardEmulationCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulationCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulationCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: Self = Self(0i32);
    pub const Uicc: Self = Self(1i32);
    pub const EmbeddedSE: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardEmulationType {}
impl ::core::clone::Clone for SmartCardEmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardEmulationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardEmulationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardEmulator(::windows_core::IUnknown);
impl SmartCardEmulator {
    pub fn EnablementPolicy(&self) -> ::windows_core::Result<SmartCardEmulatorEnablementPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardEmulatorEnablementPolicy>::zeroed();
            (::windows_core::Interface::vtable(this).EnablementPolicy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulatorEnablementPolicy>(result__)
        }
    }
    pub fn ApduReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ApduReceived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveApduReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveApduReceived)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ConnectionDeactivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionDeactivated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveConnectionDeactivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionDeactivated)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsHostCardEmulationSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHostCardEmulationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardEmulator>> {
        Self::ISmartCardEmulatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardEmulator>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppletIdGroupRegistrationsAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppletIdGroupRegistrationsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>>(result__)
        })
    }
    pub fn RegisterAppletIdGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, SmartCardAppletIdGroup>>(appletidgroup: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAppletIdGroupAsync)(::windows_core::Interface::as_raw(this), appletidgroup.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>(result__)
        })
    }
    pub fn UnregisterAppletIdGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, SmartCardAppletIdGroupRegistration>>(registration: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnregisterAppletIdGroupAsync)(::windows_core::Interface::as_raw(this), registration.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn MaxAppletIdGroupRegistrations() -> ::windows_core::Result<u16> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAppletIdGroupRegistrations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISmartCardEmulatorStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISmartCardEmulatorStatics<R, F: FnOnce(&ISmartCardEmulatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardEmulatorStatics2<R, F: FnOnce(&ISmartCardEmulatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardEmulatorStatics3<R, F: FnOnce(&ISmartCardEmulatorStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardEmulator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulator {}
impl ::core::fmt::Debug for SmartCardEmulator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulator;{dfb906b2-875e-47e5-8077-e8bff1b1c6fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulator {
    type Vtable = ISmartCardEmulator_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardEmulator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulator";
}
impl ::core::convert::From<SmartCardEmulator> for ::windows_core::IUnknown {
    fn from(value: SmartCardEmulator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulator> for ::windows_core::IUnknown {
    fn from(value: &SmartCardEmulator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardEmulator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardEmulator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardEmulator> for ::windows_core::IInspectable {
    fn from(value: SmartCardEmulator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulator> for ::windows_core::IInspectable {
    fn from(value: &SmartCardEmulator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardEmulator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardEmulator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulator {}
unsafe impl ::core::marker::Sync for SmartCardEmulator {}
#[repr(transparent)]
pub struct SmartCardEmulatorApduReceivedEventArgs(::windows_core::IUnknown);
impl SmartCardEmulatorApduReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandApdu)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn ConnectionProperties(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulatorConnectionProperties>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryRespondAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, responseapdu: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondAsync)(::windows_core::Interface::as_raw(this), responseapdu.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AutomaticResponseStatus(&self) -> ::windows_core::Result<SmartCardAutomaticResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardAutomaticResponseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticResponseStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardAutomaticResponseStatus>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryRespondWithStateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, responseapdu: Param0, nextstate: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithStateAsync)(::windows_core::Interface::as_raw(this), responseapdu.into_param().abi(), nextstate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>>(&self, responsetemplate: Param0, cryptogramplacementsteps: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithCryptogramsAsync)(::windows_core::Interface::as_raw(this), responsetemplate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAndStateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, responsetemplate: Param0, cryptogramplacementsteps: Param1, nextstate: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = &::windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryRespondWithCryptogramsAndStateAsync)(::windows_core::Interface::as_raw(this), responsetemplate.into_param().abi(), cryptogramplacementsteps.into_param().abi(), nextstate.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardEmulatorApduReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorApduReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorApduReceivedEventArgs {}
impl ::core::fmt::Debug for SmartCardEmulatorApduReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorApduReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorApduReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs;{d55d1576-69d2-5333-5b5f-f8c0d6e9f09f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardEmulatorApduReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorApduReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs";
}
impl ::core::convert::From<SmartCardEmulatorApduReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SmartCardEmulatorApduReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorApduReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SmartCardEmulatorApduReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardEmulatorApduReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SmartCardEmulatorApduReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorApduReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SmartCardEmulatorApduReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardEmulatorApduReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorApduReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorApduReceivedEventArgs {}
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(::windows_core::IUnknown);
impl SmartCardEmulatorConnectionDeactivatedEventArgs {
    pub fn ConnectionProperties(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulatorConnectionProperties>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionDeactivatedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardEmulatorConnectionDeactivatedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulatorConnectionDeactivatedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorConnectionDeactivatedEventArgs {}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionDeactivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs;{2186d8d3-c5eb-5262-43df-62a0a1b55557})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardEmulatorConnectionDeactivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs";
}
impl ::core::convert::From<SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionDeactivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SmartCardEmulatorConnectionDeactivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionDeactivatedEventArgs {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionDeactivatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: Self = Self(0i32);
    pub const ConnectionRedirected: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionDeactivatedReason {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorConnectionDeactivatedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardEmulatorConnectionDeactivatedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionDeactivatedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionDeactivatedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionProperties(::windows_core::IUnknown);
impl SmartCardEmulatorConnectionProperties {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<SmartCardEmulatorConnectionSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardEmulatorConnectionSource>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulatorConnectionSource>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardEmulatorConnectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardEmulatorConnectionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardEmulatorConnectionProperties {}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties;{4e2ca5ee-f969-507d-6cf9-34e2d18df311})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardEmulatorConnectionProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardEmulatorConnectionProperties {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties";
}
impl ::core::convert::From<SmartCardEmulatorConnectionProperties> for ::windows_core::IUnknown {
    fn from(value: SmartCardEmulatorConnectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionProperties> for ::windows_core::IUnknown {
    fn from(value: &SmartCardEmulatorConnectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardEmulatorConnectionProperties> for ::windows_core::IInspectable {
    fn from(value: SmartCardEmulatorConnectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardEmulatorConnectionProperties> for ::windows_core::IInspectable {
    fn from(value: &SmartCardEmulatorConnectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardEmulatorConnectionProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardEmulatorConnectionProperties {}
unsafe impl ::core::marker::Sync for SmartCardEmulatorConnectionProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: Self = Self(0i32);
    pub const NfcReader: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionSource {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorConnectionSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardEmulatorConnectionSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardEmulatorConnectionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorConnectionSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const ScreenOn: Self = Self(2i32);
    pub const ScreenUnlocked: Self = Self(3i32);
}
impl ::core::marker::Copy for SmartCardEmulatorEnablementPolicy {}
impl ::core::clone::Clone for SmartCardEmulatorEnablementPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardEmulatorEnablementPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardEmulatorEnablementPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardEmulatorEnablementPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorEnablementPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardEmulatorEnablementPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorEnablementPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: Self = Self(0i32);
    pub const AboveLock: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardLaunchBehavior {}
impl ::core::clone::Clone for SmartCardLaunchBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardLaunchBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardLaunchBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardLaunchBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardLaunchBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardLaunchBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardLaunchBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: Self = Self(0i32);
    pub const RequireAtLeastOne: Self = Self(1i32);
    pub const Disallow: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardPinCharacterPolicyOption {}
impl ::core::clone::Clone for SmartCardPinCharacterPolicyOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardPinCharacterPolicyOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardPinCharacterPolicyOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardPinCharacterPolicyOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinCharacterPolicyOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardPinCharacterPolicyOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardPinCharacterPolicyOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardPinPolicy(::windows_core::IUnknown);
impl SmartCardPinPolicy {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardPinPolicy, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MinLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMinLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UppercaseLetters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardPinCharacterPolicyOption>::zeroed();
            (::windows_core::Interface::vtable(this).UppercaseLetters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    pub fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUppercaseLetters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowercaseLetters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardPinCharacterPolicyOption>::zeroed();
            (::windows_core::Interface::vtable(this).LowercaseLetters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    pub fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLowercaseLetters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Digits(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardPinCharacterPolicyOption>::zeroed();
            (::windows_core::Interface::vtable(this).Digits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    pub fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecialCharacters(&self) -> ::windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardPinCharacterPolicyOption>::zeroed();
            (::windows_core::Interface::vtable(this).SpecialCharacters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardPinCharacterPolicyOption>(result__)
        }
    }
    pub fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecialCharacters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SmartCardPinPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardPinPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinPolicy {}
impl ::core::fmt::Debug for SmartCardPinPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardPinPolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinPolicy;{183ce184-4db6-4841-ac9e-2ac1f39b7304})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardPinPolicy as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinPolicy {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinPolicy";
}
impl ::core::convert::From<SmartCardPinPolicy> for ::windows_core::IUnknown {
    fn from(value: SmartCardPinPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinPolicy> for ::windows_core::IUnknown {
    fn from(value: &SmartCardPinPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardPinPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardPinPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardPinPolicy> for ::windows_core::IInspectable {
    fn from(value: SmartCardPinPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinPolicy> for ::windows_core::IInspectable {
    fn from(value: &SmartCardPinPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardPinPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardPinPolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardPinPolicy {}
unsafe impl ::core::marker::Sync for SmartCardPinPolicy {}
#[repr(transparent)]
pub struct SmartCardPinResetDeferral(::windows_core::IUnknown);
impl SmartCardPinResetDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SmartCardPinResetDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetDeferral {}
impl ::core::fmt::Debug for SmartCardPinResetDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardPinResetDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetDeferral;{18c94aac-7805-4004-85e4-bbefac8f6884})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardPinResetDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinResetDeferral {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetDeferral";
}
impl ::core::convert::From<SmartCardPinResetDeferral> for ::windows_core::IUnknown {
    fn from(value: SmartCardPinResetDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinResetDeferral> for ::windows_core::IUnknown {
    fn from(value: &SmartCardPinResetDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardPinResetDeferral> for ::windows_core::IInspectable {
    fn from(value: SmartCardPinResetDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinResetDeferral> for ::windows_core::IInspectable {
    fn from(value: &SmartCardPinResetDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardPinResetDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardPinResetDeferral {}
unsafe impl ::core::marker::Sync for SmartCardPinResetDeferral {}
#[repr(transparent)]
pub struct SmartCardPinResetHandler(pub ::windows_core::IUnknown);
impl SmartCardPinResetHandler {
    pub fn new<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmartCardPinResetHandlerBox::<F> { vtable: &SmartCardPinResetHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, SmartCardProvisioning>, Param1: ::windows_core::IntoParam<'a, SmartCardPinResetRequest>>(&self, sender: Param0, request: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), request.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct SmartCardPinResetHandlerBox<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmartCardPinResetHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SmartCardProvisioning>, &::core::option::Option<SmartCardPinResetRequest>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SmartCardPinResetHandlerBox<F> {
    const VTABLE: SmartCardPinResetHandler_Vtbl = SmartCardPinResetHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<SmartCardPinResetHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, request: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&request)).into()
    }
}
impl ::core::clone::Clone for SmartCardPinResetHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetHandler {}
impl ::core::fmt::Debug for SmartCardPinResetHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetHandler {
    type Vtable = SmartCardPinResetHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x138d5e40_f3bc_4a5c_b41d_4b4ef684e237);
}
unsafe impl ::windows_core::RuntimeType for SmartCardPinResetHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{138d5e40-f3bc-4a5c-b41d-4b4ef684e237}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SmartCardPinResetHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, request: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SmartCardPinResetRequest(::windows_core::IUnknown);
impl SmartCardPinResetRequest {
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Challenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Deadline(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<SmartCardPinResetDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardPinResetDeferral>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponse<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, response: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponse)(::windows_core::Interface::as_raw(this), response.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SmartCardPinResetRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardPinResetRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardPinResetRequest {}
impl ::core::fmt::Debug for SmartCardPinResetRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardPinResetRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardPinResetRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardPinResetRequest;{12fe3c4d-5fb9-4e8e-9ff6-61f475124fef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardPinResetRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardPinResetRequest {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetRequest";
}
impl ::core::convert::From<SmartCardPinResetRequest> for ::windows_core::IUnknown {
    fn from(value: SmartCardPinResetRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinResetRequest> for ::windows_core::IUnknown {
    fn from(value: &SmartCardPinResetRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardPinResetRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardPinResetRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardPinResetRequest> for ::windows_core::IInspectable {
    fn from(value: SmartCardPinResetRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardPinResetRequest> for ::windows_core::IInspectable {
    fn from(value: &SmartCardPinResetRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardPinResetRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardPinResetRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardPinResetRequest {}
unsafe impl ::core::marker::Sync for SmartCardPinResetRequest {}
#[repr(transparent)]
pub struct SmartCardProvisioning(::windows_core::IUnknown);
impl SmartCardProvisioning {
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCard>(result__)
        }
    }
    pub fn GetIdAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::GUID>>(result__)
        }
    }
    pub fn GetNameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GetChallengeContextAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardChallengeContext>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChallengeContextAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardChallengeContext>>(result__)
        }
    }
    pub fn RequestPinChangeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinChangeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestPinResetAsync<'a, Param0: ::windows_core::IntoParam<'a, SmartCardPinResetHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinResetAsync)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetAuthorityKeyContainerNameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<ISmartCardProvisioning2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAuthorityKeyContainerNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn FromSmartCardAsync<'a, Param0: ::windows_core::IntoParam<'a, SmartCard>>(card: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromSmartCardAsync)(::windows_core::Interface::as_raw(this), card.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestVirtualSmartCardCreationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, SmartCardPinPolicy>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsync)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestVirtualSmartCardCreationAsyncWithCardId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, SmartCardPinPolicy>, Param3: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2, cardid: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsyncWithCardId)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), cardid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    pub fn RequestVirtualSmartCardDeletionAsync<'a, Param0: ::windows_core::IntoParam<'a, SmartCard>>(card: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestVirtualSmartCardDeletionAsync)(::windows_core::Interface::as_raw(this), card.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestAttestedVirtualSmartCardCreationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, SmartCardPinPolicy>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsync)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, SmartCardPinPolicy>, Param3: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(friendlyname: Param0, administrativekey: Param1, pinpolicy: Param2, cardid: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>> {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsyncWithCardId)(::windows_core::Interface::as_raw(this), friendlyname.into_param().abi(), administrativekey.into_param().abi(), pinpolicy.into_param().abi(), cardid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardProvisioning>>(result__)
        })
    }
    pub fn ISmartCardProvisioningStatics<R, F: FnOnce(&ISmartCardProvisioningStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmartCardProvisioningStatics2<R, F: FnOnce(&ISmartCardProvisioningStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardProvisioning {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardProvisioning {}
impl ::core::fmt::Debug for SmartCardProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardProvisioning").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardProvisioning {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardProvisioning;{19eeedbd-1fab-477c-b712-1a2c5af1fd6e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardProvisioning as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardProvisioning {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardProvisioning";
}
impl ::core::convert::From<SmartCardProvisioning> for ::windows_core::IUnknown {
    fn from(value: SmartCardProvisioning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardProvisioning> for ::windows_core::IUnknown {
    fn from(value: &SmartCardProvisioning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardProvisioning> for ::windows_core::IInspectable {
    fn from(value: SmartCardProvisioning) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardProvisioning> for ::windows_core::IInspectable {
    fn from(value: &SmartCardProvisioning) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardProvisioning {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardProvisioning {}
unsafe impl ::core::marker::Sync for SmartCardProvisioning {}
#[repr(transparent)]
pub struct SmartCardReader(::windows_core::IUnknown);
impl SmartCardReader {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<SmartCardReaderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardReaderKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardReaderKind>(result__)
        }
    }
    pub fn GetStatusAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardReaderStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardReaderStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllCardsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<SmartCard>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllCardsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<SmartCard>>>(result__)
        }
    }
    pub fn CardAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CardAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCardAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCardAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CardRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CardRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCardRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCardRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithKind(kind: SmartCardReaderKind) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithKind)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SmartCardReader>> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SmartCardReader>>(result__)
        })
    }
    pub fn ISmartCardReaderStatics<R, F: FnOnce(&ISmartCardReaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SmartCardReader, ISmartCardReaderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SmartCardReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardReader {}
impl ::core::fmt::Debug for SmartCardReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardReader;{1074b4e0-54c2-4df0-817a-14c14378f06c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardReader {
    type Vtable = ISmartCardReader_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardReader {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardReader";
}
impl ::core::convert::From<SmartCardReader> for ::windows_core::IUnknown {
    fn from(value: SmartCardReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardReader> for ::windows_core::IUnknown {
    fn from(value: &SmartCardReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardReader> for ::windows_core::IInspectable {
    fn from(value: SmartCardReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardReader> for ::windows_core::IInspectable {
    fn from(value: &SmartCardReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardReader {}
unsafe impl ::core::marker::Sync for SmartCardReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Tpm: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Uicc: Self = Self(4i32);
    pub const EmbeddedSE: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardReaderKind {}
impl ::core::clone::Clone for SmartCardReaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardReaderKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardReaderKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardReaderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReaderKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardReaderKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Exclusive: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardReaderStatus {}
impl ::core::clone::Clone for SmartCardReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardReaderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardReaderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardReaderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardReaderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardReaderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Shared: Self = Self(2i32);
    pub const Exclusive: Self = Self(3i32);
    pub const Unresponsive: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardStatus {}
impl ::core::clone::Clone for SmartCardStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SmartCardTriggerDetails(::windows_core::IUnknown);
impl SmartCardTriggerDetails {
    pub fn TriggerType(&self) -> ::windows_core::Result<SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SmartCardTriggerType>::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardTriggerType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceAppletId(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppletId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TriggerData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TriggerData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Emulator(&self) -> ::windows_core::Result<SmartCardEmulator> {
        let this = &::windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Emulator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCardEmulator>(result__)
        }
    }
    pub fn TryLaunchCurrentAppAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, arguments: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryLaunchCurrentAppAsync)(::windows_core::Interface::as_raw(this), arguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryLaunchCurrentAppWithBehaviorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, arguments: Param0, behavior: SmartCardLaunchBehavior) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryLaunchCurrentAppWithBehaviorAsync)(::windows_core::Interface::as_raw(this), arguments.into_param().abi(), behavior, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SmartCard(&self) -> ::windows_core::Result<SmartCard> {
        let this = &::windows_core::Interface::cast::<ISmartCardTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmartCard)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SmartCard>(result__)
        }
    }
}
impl ::core::clone::Clone for SmartCardTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SmartCardTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardTriggerDetails {}
impl ::core::fmt::Debug for SmartCardTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SmartCards.SmartCardTriggerDetails;{5f9bf11e-39ef-4f2b-b44f-0a9155b177bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISmartCardTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardTriggerDetails {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardTriggerDetails";
}
impl ::core::convert::From<SmartCardTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: SmartCardTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &SmartCardTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SmartCardTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SmartCardTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SmartCardTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: SmartCardTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SmartCardTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &SmartCardTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SmartCardTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SmartCardTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SmartCardTriggerDetails {}
unsafe impl ::core::marker::Sync for SmartCardTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: Self = Self(0i32);
    pub const EmulatorNearFieldEntry: Self = Self(1i32);
    pub const EmulatorNearFieldExit: Self = Self(2i32);
    pub const EmulatorHostApplicationActivated: Self = Self(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: Self = Self(4i32);
    pub const ReaderCardAdded: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardTriggerType {}
impl ::core::clone::Clone for SmartCardTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardTriggerType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardTriggerType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardTriggerType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardTriggerType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: Self = Self(0i32);
    pub const RequireUnlockPrompt: Self = Self(1i32);
    pub const PreventUnlockPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardUnlockPromptingBehavior {}
impl ::core::clone::Clone for SmartCardUnlockPromptingBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmartCardUnlockPromptingBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SmartCardUnlockPromptingBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for SmartCardUnlockPromptingBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardUnlockPromptingBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SmartCardUnlockPromptingBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardUnlockPromptingBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
