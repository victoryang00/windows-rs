#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialPromptType {}
impl ::core::clone::Clone for CredentialPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CredentialPromptType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CredentialPromptType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CredentialPromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPromptType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CredentialPromptType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdAuthenticator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AuthenticateUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AuthenticateUserAsyncAdvanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: ::windows_core::RawPtr, credentialprompttype: CredentialPromptType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AuthenticateUserAsyncAdvanced: usize,
    pub SignOutUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CanSignOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AuthenticatedSafeCustomerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicket(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdServiceTicketRequestFactory {
    type Vtable = IOnlineIdServiceTicketRequestFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbebb0a08_9e73_4077_9614_08614c0bc245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateOnlineIdServiceTicketRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateOnlineIdServiceTicketRequestAdvanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdSystemAuthenticatorStatics {
    type Vtable = IOnlineIdSystemAuthenticatorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85047792_f634_41e3_96a4_5164e902c740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemIdentity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Ticket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemTicketResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OnlineIdSystemTicketStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserIdentity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserIdentity {
    type Vtable = IUserIdentity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserIdentity_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Tickets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Tickets: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SafeCustomerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SignInName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsBetaAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsConfirmedPC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct OnlineIdAuthenticator(::windows_core::IUnknown);
impl OnlineIdAuthenticator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OnlineIdAuthenticator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AuthenticateUserAsync<'a, Param0: ::windows_core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows_core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticateUserAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AuthenticateUserAsyncAdvanced<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>>(&self, requests: Param0, credentialprompttype: CredentialPromptType) -> ::windows_core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticateUserAsyncAdvanced)(::windows_core::Interface::as_raw(this), requests.into_param().abi(), credentialprompttype, result__.as_mut_ptr()).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    pub fn SignOutUserAsync(&self) -> ::windows_core::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignOutUserAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SignOutUserOperation>(result__)
        }
    }
    pub fn SetApplicationId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetApplicationId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn CanSignOut(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSignOut)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AuthenticatedSafeCustomerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticatedSafeCustomerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdAuthenticator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdAuthenticator {}
impl ::core::fmt::Debug for OnlineIdAuthenticator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdAuthenticator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator;{a003f58a-29ab-4817-b884-d7516dad18b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdAuthenticator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows_core::IUnknown {
    fn from(value: OnlineIdAuthenticator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdAuthenticator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows_core::IInspectable {
    fn from(value: OnlineIdAuthenticator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdAuthenticator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdAuthenticator {}
unsafe impl ::core::marker::Sync for OnlineIdAuthenticator {}
#[repr(transparent)]
pub struct OnlineIdServiceTicket(::windows_core::IUnknown);
impl OnlineIdServiceTicket {
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Request(&self) -> ::windows_core::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OnlineIdServiceTicketRequest>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdServiceTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicket {}
impl ::core::fmt::Debug for OnlineIdServiceTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicket").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket;{c95c547f-d781-4a94-acb8-c59874238c26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdServiceTicket as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows_core::IUnknown {
    fn from(value: OnlineIdServiceTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdServiceTicket {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows_core::IInspectable {
    fn from(value: OnlineIdServiceTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdServiceTicket {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicket {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicket {}
#[repr(transparent)]
pub struct OnlineIdServiceTicketRequest(::windows_core::IUnknown);
impl OnlineIdServiceTicketRequest {
    pub fn Service(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Service)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Policy(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Policy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateOnlineIdServiceTicketRequest<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(service: Param0, policy: Param1) -> ::windows_core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnlineIdServiceTicketRequest)(::windows_core::Interface::as_raw(this), service.into_param().abi(), policy.into_param().abi(), result__.as_mut_ptr()).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    pub fn CreateOnlineIdServiceTicketRequestAdvanced<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(service: Param0) -> ::windows_core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnlineIdServiceTicketRequestAdvanced)(::windows_core::Interface::as_raw(this), service.into_param().abi(), result__.as_mut_ptr()).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OnlineIdServiceTicketRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicketRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicketRequest {}
impl ::core::fmt::Debug for OnlineIdServiceTicketRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicketRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest;{297445d3-fb63-4135-8909-4e354c061466})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdServiceTicketRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows_core::IUnknown {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows_core::IInspectable {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicketRequest {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicketRequest {}
pub struct OnlineIdSystemAuthenticator;
impl OnlineIdSystemAuthenticator {
    pub fn Default() -> ::windows_core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Default)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[repr(transparent)]
pub struct OnlineIdSystemAuthenticatorForUser(::windows_core::IUnknown);
impl OnlineIdSystemAuthenticatorForUser {
    pub fn GetTicketAsync<'a, Param0: ::windows_core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<OnlineIdSystemTicketResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTicketAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<OnlineIdSystemTicketResult>>(result__)
        }
    }
    pub fn SetApplicationId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetApplicationId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemAuthenticatorForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemAuthenticatorForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemAuthenticatorForUser {}
impl ::core::fmt::Debug for OnlineIdSystemAuthenticatorForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemAuthenticatorForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser;{5798befb-1de4-4186-a2e6-b563f86aaf44})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdSystemAuthenticatorForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows_core::IUnknown {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows_core::IInspectable {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::core::marker::Sync for OnlineIdSystemAuthenticatorForUser {}
#[repr(transparent)]
pub struct OnlineIdSystemIdentity(::windows_core::IUnknown);
impl OnlineIdSystemIdentity {
    pub fn Ticket(&self) -> ::windows_core::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Ticket)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OnlineIdServiceTicket>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemIdentity {}
impl ::core::fmt::Debug for OnlineIdSystemIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity;{743cd20d-b6ca-434d-8124-53ea12685307})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdSystemIdentity as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows_core::IUnknown {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows_core::IInspectable {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemIdentity {}
unsafe impl ::core::marker::Sync for OnlineIdSystemIdentity {}
#[repr(transparent)]
pub struct OnlineIdSystemTicketResult(::windows_core::IUnknown);
impl OnlineIdSystemTicketResult {
    pub fn Identity(&self) -> ::windows_core::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OnlineIdSystemIdentity>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<OnlineIdSystemTicketStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OnlineIdSystemTicketStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for OnlineIdSystemTicketResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemTicketResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemTicketResult {}
impl ::core::fmt::Debug for OnlineIdSystemTicketResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult;{db0a5ff8-b098-4acd-9d13-9e640652b5b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_Vtbl;
    const IID: ::windows_core::GUID = <IOnlineIdSystemTicketResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows_core::IUnknown {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows_core::IUnknown {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows_core::IInspectable {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows_core::IInspectable {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemTicketResult {}
unsafe impl ::core::marker::Sync for OnlineIdSystemTicketResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl ::core::marker::Copy for OnlineIdSystemTicketStatus {}
impl ::core::clone::Clone for OnlineIdSystemTicketStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OnlineIdSystemTicketStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OnlineIdSystemTicketStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for OnlineIdSystemTicketStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SignOutUserOperation(::windows_core::IUnknown);
impl SignOutUserOperation {
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SignOutUserOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignOutUserOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignOutUserOperation {}
impl ::core::fmt::Debug for SignOutUserOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignOutUserOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SignOutUserOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.SignOutUserOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SignOutUserOperation {
    type Vtable = ::winrt_foundation::IAsyncAction_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncAction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
impl SignOutUserOperation {
    pub fn get(&self) -> ::windows_core::Result<()> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl ::std::future::Future for SignOutUserOperation {
    type Output = ::windows_core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl ::core::convert::From<SignOutUserOperation> for ::windows_core::IUnknown {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignOutUserOperation> for ::windows_core::IUnknown {
    fn from(value: &SignOutUserOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SignOutUserOperation> for ::windows_core::IInspectable {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignOutUserOperation> for ::windows_core::IInspectable {
    fn from(value: &SignOutUserOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SignOutUserOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SignOutUserOperation> for ::winrt_foundation::IAsyncAction {
    type Error = ::windows_core::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncAction> for &SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncAction> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncAction>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SignOutUserOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SignOutUserOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &SignOutUserOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SignOutUserOperation {}
unsafe impl ::core::marker::Sync for SignOutUserOperation {}
#[repr(transparent)]
pub struct UserAuthenticationOperation(::windows_core::IUnknown);
impl UserAuthenticationOperation {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::winrt_foundation::AsyncStatus> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::AsyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncStatus>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IAsyncInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::AsyncOperationCompletedHandler<UserIdentity>>>(&self, handler: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi()).ok() }
    }
    pub fn Completed(&self) -> ::windows_core::Result<::winrt_foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::AsyncOperationCompletedHandler<UserIdentity>>(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows_core::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserIdentity>(result__)
        }
    }
}
impl ::core::clone::Clone for UserAuthenticationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationOperation {}
impl ::core::fmt::Debug for UserAuthenticationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserAuthenticationOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserAuthenticationOperation {
    type Vtable = ::winrt_foundation::IAsyncOperation_Vtbl<UserIdentity>;
    const IID: ::windows_core::GUID = <::winrt_foundation::IAsyncOperation<UserIdentity> as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
impl UserAuthenticationOperation {
    pub fn get(&self) -> ::windows_core::Result<UserIdentity> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows_core::Waiter::new()?;
            self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl ::std::future::Future for UserAuthenticationOperation {
    type Output = ::windows_core::Result<UserIdentity>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == ::winrt_foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(::winrt_foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl ::core::convert::From<UserAuthenticationOperation> for ::windows_core::IUnknown {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows_core::IUnknown {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserAuthenticationOperation> for ::windows_core::IInspectable {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows_core::IInspectable {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserAuthenticationOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserAuthenticationOperation> for ::winrt_foundation::IAsyncInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncInfo> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncInfo> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<UserAuthenticationOperation> for ::winrt_foundation::IAsyncOperation<UserIdentity> {
    type Error = ::windows_core::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserAuthenticationOperation> for ::winrt_foundation::IAsyncOperation<UserIdentity> {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<UserIdentity>> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<UserIdentity>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IAsyncOperation<UserIdentity>> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IAsyncOperation<UserIdentity>> {
        ::core::convert::TryInto::<::winrt_foundation::IAsyncOperation<UserIdentity>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationOperation {}
unsafe impl ::core::marker::Sync for UserAuthenticationOperation {}
#[repr(transparent)]
pub struct UserIdentity(::windows_core::IUnknown);
impl UserIdentity {
    #[cfg(feature = "winrt-foundation")]
    pub fn Tickets(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Tickets)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<OnlineIdServiceTicket>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SafeCustomerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SafeCustomerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SignInName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SignInName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LastName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LastName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsBetaAccount(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBetaAccount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsConfirmedPC(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConfirmedPC)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for UserIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserIdentity {}
impl ::core::fmt::Debug for UserIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserIdentity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserIdentity {
    type Vtable = IUserIdentity_Vtbl;
    const IID: ::windows_core::GUID = <IUserIdentity as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
impl ::core::convert::From<UserIdentity> for ::windows_core::IUnknown {
    fn from(value: UserIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows_core::IUnknown {
    fn from(value: &UserIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserIdentity> for ::windows_core::IInspectable {
    fn from(value: UserIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows_core::IInspectable {
    fn from(value: &UserIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserIdentity {}
unsafe impl ::core::marker::Sync for UserIdentity {}
