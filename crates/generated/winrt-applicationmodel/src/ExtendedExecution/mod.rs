#[cfg(feature = "Foreground")]
pub mod Foreground;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: Self = Self(0i32);
    pub const LocationTracking: Self = Self(1i32);
    pub const SavingData: Self = Self(2i32);
}
impl ::core::marker::Copy for ExtendedExecutionReason {}
impl ::core::clone::Clone for ExtendedExecutionReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionResult {}
impl ::core::clone::Clone for ExtendedExecutionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionRevokedEventArgs(::windows_core::IUnknown);
impl ExtendedExecutionRevokedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionRevokedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ExtendedExecutionRevokedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExtendedExecutionRevokedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for ExtendedExecutionRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionRevokedEventArgs {}
impl ::core::fmt::Debug for ExtendedExecutionRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionRevokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs;{bfbc9f16-63b5-4c0b-aad6-828af5373ec3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionRevokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs";
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionRevokedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionRevokedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionRevokedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionRevokedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionRevokedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionRevokedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionSession(::windows_core::IUnknown);
impl ExtendedExecutionSession {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ExtendedExecutionSession, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ExtendedExecutionReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExtendedExecutionReason>(result__)
        }
    }
    pub fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReason)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PercentProgress(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PercentProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetPercentProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPercentProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Revoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::windows_core::IInspectable, ExtendedExecutionRevokedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Revoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRevoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRevoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RequestExtensionAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ExtendedExecutionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestExtensionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ExtendedExecutionResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for ExtendedExecutionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionSession {}
impl ::core::fmt::Debug for ExtendedExecutionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession;{af908a2d-118b-48f1-9308-0c4fc41e200f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession";
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows_core::IUnknown {
    fn from(value: ExtendedExecutionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows_core::IUnknown {
    fn from(value: &ExtendedExecutionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows_core::IInspectable {
    fn from(value: ExtendedExecutionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows_core::IInspectable {
    fn from(value: &ExtendedExecutionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ExtendedExecutionSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ExtendedExecutionSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ExtendedExecutionSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ExtendedExecutionSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ExtendedExecutionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionSession {}
unsafe impl ::core::marker::Sync for ExtendedExecutionSession {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfbc9f16_63b5_4c0b_aad6_828af5373ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionRevokedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf908a2d_118b_48f1_9308_0c4fc41e200f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionReason) -> ::windows_core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ExtendedExecutionReason) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PercentProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetPercentProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Revoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RequestExtensionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
