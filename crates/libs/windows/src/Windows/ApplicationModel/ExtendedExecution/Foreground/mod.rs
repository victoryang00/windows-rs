#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: Self = Self(0i32);
    pub const SavingData: Self = Self(1i32);
    pub const BackgroundAudio: Self = Self(2i32);
    pub const Unconstrained: Self = Self(3i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionForegroundReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionForegroundReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionForegroundReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundResult {}
impl ::core::clone::Clone for ExtendedExecutionForegroundResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionForegroundResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionForegroundResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionForegroundResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(::windows_core::IUnknown);
impl ExtendedExecutionForegroundRevokedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionForegroundRevokedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ExtendedExecutionForegroundRevokedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExtendedExecutionForegroundRevokedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for ExtendedExecutionForegroundRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionForegroundRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionForegroundRevokedEventArgs {}
impl ::core::fmt::Debug for ExtendedExecutionForegroundRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionForegroundRevokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs;{b07cd940-9557-aea4-2c99-bdd56d9be461})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionForegroundRevokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionForegroundRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
}
impl ::core::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundRevokedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionForegroundRevokedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExtendedExecutionForegroundRevokedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundRevokedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundRevokedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionForegroundRevokedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundSession(::windows_core::IUnknown);
impl ExtendedExecutionForegroundSession {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ExtendedExecutionForegroundSession, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Revoked<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Revoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRevoked<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRevoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestExtensionAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestExtensionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionForegroundReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ExtendedExecutionForegroundReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExtendedExecutionForegroundReason>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution_Foreground\"`*"]
    pub fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReason)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ExtendedExecutionForegroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionForegroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionForegroundSession {}
impl ::core::fmt::Debug for ExtendedExecutionForegroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExtendedExecutionForegroundSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession;{fbf440e1-9d10-4201-b01e-c83275296f2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionForegroundSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionForegroundSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
}
impl ::core::convert::From<ExtendedExecutionForegroundSession> for ::windows_core::IUnknown {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundSession> for ::windows_core::IUnknown {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionForegroundSession> for ::windows_core::IInspectable {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundSession> for ::windows_core::IInspectable {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ExtendedExecutionForegroundSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ExtendedExecutionForegroundSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundSession {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundSession {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb07cd940_9557_aea4_2c99_bdd56d9be461);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionForegroundRevokedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionForegroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbf440e1_9d10_4201_b01e_c83275296f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Revoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Revoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RequestExtensionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestExtensionAsync: usize,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionForegroundReason) -> ::windows_core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ExtendedExecutionForegroundReason) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
