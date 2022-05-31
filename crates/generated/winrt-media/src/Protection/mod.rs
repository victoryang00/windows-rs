#[cfg(feature = "PlayReady")]
pub mod PlayReady;
#[repr(transparent)]
pub struct ComponentLoadFailedEventArgs(::windows_core::IUnknown);
impl ComponentLoadFailedEventArgs {
    pub fn Information(&self) -> ::windows_core::Result<RevocationAndRenewalInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Information)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RevocationAndRenewalInformation>(result__)
        }
    }
    pub fn Completion(&self) -> ::windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaProtectionServiceCompletion>(result__)
        }
    }
}
impl ::core::clone::Clone for ComponentLoadFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ComponentLoadFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ComponentLoadFailedEventArgs {}
impl ::core::fmt::Debug for ComponentLoadFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentLoadFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ComponentLoadFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ComponentLoadFailedEventArgs;{95972e93-7746-417e-8495-f031bbc5862c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IComponentLoadFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ComponentLoadFailedEventArgs";
}
impl ::core::convert::From<ComponentLoadFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ComponentLoadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ComponentLoadFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ComponentLoadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ComponentLoadFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ComponentLoadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ComponentLoadFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ComponentLoadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ComponentLoadFailedEventArgs {}
unsafe impl ::core::marker::Sync for ComponentLoadFailedEventArgs {}
#[repr(transparent)]
pub struct ComponentLoadFailedEventHandler(pub ::windows_core::IUnknown);
impl ComponentLoadFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ComponentLoadFailedEventHandlerBox::<F> { vtable: &ComponentLoadFailedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, MediaProtectionManager>, Param1: ::windows_core::IntoParam<'a, ComponentLoadFailedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ComponentLoadFailedEventHandlerBox<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ComponentLoadFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ComponentLoadFailedEventHandlerBox<F> {
    const VTABLE: ComponentLoadFailedEventHandler_Vtbl = ComponentLoadFailedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ComponentLoadFailedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ComponentLoadFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ComponentLoadFailedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ComponentLoadFailedEventHandler {}
impl ::core::fmt::Debug for ComponentLoadFailedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentLoadFailedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ComponentLoadFailedEventHandler {
    type Vtable = ComponentLoadFailedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95da643c_6db9_424b_86ca_091af432081c);
}
unsafe impl ::windows_core::RuntimeType for ComponentLoadFailedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{95da643c-6db9-424b-86ca-091af432081c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ComponentLoadFailedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub struct ComponentRenewal;
impl ComponentRenewal {
    pub fn RenewSystemComponentsAsync<'a, Param0: ::windows_core::IntoParam<'a, RevocationAndRenewalInformation>>(information: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<RenewalStatus, u32>> {
        Self::IComponentRenewalStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenewSystemComponentsAsync)(::windows_core::Interface::as_raw(this), information.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>(result__)
        })
    }
    pub fn IComponentRenewalStatics<R, F: FnOnce(&IComponentRenewalStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ComponentRenewal, IComponentRenewalStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ComponentRenewal {
    const NAME: &'static str = "Windows.Media.Protection.ComponentRenewal";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for GraphicsTrustStatus {}
impl ::core::clone::Clone for GraphicsTrustStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GraphicsTrustStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GraphicsTrustStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GraphicsTrustStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsTrustStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GraphicsTrustStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.GraphicsTrustStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl ::core::marker::Copy for HdcpProtection {}
impl ::core::clone::Clone for HdcpProtection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpProtection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdcpProtection {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdcpProtection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpProtection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdcpProtection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpProtection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HdcpSession(::windows_core::IUnknown);
impl HdcpSession {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HdcpSession, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEffectiveProtectionAtLeast)(::windows_core::Interface::as_raw(this), protection, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetEffectiveProtection(&self) -> ::windows_core::Result<::winrt_foundation::IReference<HdcpProtection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetEffectiveProtection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<HdcpProtection>>(result__)
        }
    }
    pub fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<HdcpSetProtectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetDesiredMinProtectionAsync)(::windows_core::Interface::as_raw(this), protection, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<HdcpSetProtectionResult>>(result__)
        }
    }
    pub fn ProtectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<HdcpSession, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveProtectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProtectionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for HdcpSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdcpSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdcpSession {}
impl ::core::fmt::Debug for HdcpSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdcpSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.HdcpSession;{718845e9-64d7-426d-809b-1be461941a2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HdcpSession {
    type Vtable = IHdcpSession_Vtbl;
    const IID: ::windows_core::GUID = <IHdcpSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.HdcpSession";
}
impl ::core::convert::From<HdcpSession> for ::windows_core::IUnknown {
    fn from(value: HdcpSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdcpSession> for ::windows_core::IUnknown {
    fn from(value: &HdcpSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HdcpSession> for ::windows_core::IInspectable {
    fn from(value: HdcpSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdcpSession> for ::windows_core::IInspectable {
    fn from(value: &HdcpSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HdcpSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HdcpSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HdcpSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HdcpSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HdcpSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HdcpSession {}
unsafe impl ::core::marker::Sync for HdcpSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for HdcpSetProtectionResult {}
impl ::core::clone::Clone for HdcpSetProtectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpSetProtectionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdcpSetProtectionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdcpSetProtectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpSetProtectionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdcpSetProtectionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpSetProtectionResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IComponentLoadFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95972e93_7746_417e_8495_f031bbc5862c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentLoadFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IComponentRenewalStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComponentRenewalStatics {
    type Vtable = IComponentRenewalStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ffbcd67_b795_48c5_8b7b_a7c4efe202e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentRenewalStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RenewSystemComponentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, information: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdcpSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdcpSession {
    type Vtable = IHdcpSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x718845e9_64d7_426d_809b_1be461941a2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdcpSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEffectiveProtectionAtLeast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetEffectiveProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredMinProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45694947_c741_434b_a79e_474c12d93d2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionPMPServer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c111226_7b26_4d31_95bb_9c1b08ef7fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionPMPServerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionPMPServerFactory {
    type Vtable = IMediaProtectionPMPServerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x602c8e5e_f7d2_487e_af91_dbc4252b2182);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreatePMPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreatePMPServer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionServiceCompletion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b5cca18_cfd5_44ee_a2ed_df76010c14b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceCompletion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, success: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMediaProtectionServiceRequest(::windows_core::IUnknown);
impl IMediaProtectionServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::convert::From<IMediaProtectionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: IMediaProtectionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaProtectionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &IMediaProtectionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaProtectionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: IMediaProtectionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaProtectionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &IMediaProtectionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaProtectionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaProtectionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaProtectionServiceRequest {}
impl ::core::fmt::Debug for IMediaProtectionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaProtectionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IMediaProtectionServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b1de0ea6-2094-478d-87a4-8b95200f85c6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IMediaProtectionServiceRequest {
    type Vtable = IMediaProtectionServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1de0ea6_2094_478d_87a4_8b95200f85c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProtectionSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7ac5d7e_7480_4d29_a464_7bcd913dd8e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, keysystem: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ProtectionCapabilityResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevocationAndRenewalInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3a1937b_2501_439e_a6e7_6fc95e175fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevocationAndRenewalItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3099c20c_3cf0_49ea_902d_caf32d2dde2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reasons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RevocationAndRenewalReasons) -> ::windows_core::HRESULT,
    pub HeaderHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PublicKeyHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RenewalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34283baf_abb4_4fc1_bd89_93f106573a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceRequestedEventArgs2 {
    type Vtable = IServiceRequestedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x553c69d6_fafe_4128_8dfa_130e398a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-media")]
    pub MediaPlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-media"))]
    MediaPlaybackItem: usize,
}
#[repr(transparent)]
pub struct MediaProtectionManager(::windows_core::IUnknown);
impl MediaProtectionManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaProtectionManager, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ServiceRequested<'a, Param0: ::windows_core::IntoParam<'a, ServiceRequestedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveServiceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServiceRequested)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn RebootNeeded<'a, Param0: ::windows_core::IntoParam<'a, RebootNeededEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RebootNeeded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRebootNeeded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRebootNeeded)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn ComponentLoadFailed<'a, Param0: ::windows_core::IntoParam<'a, ComponentLoadFailedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ComponentLoadFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveComponentLoadFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveComponentLoadFailed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionManager {}
impl ::core::fmt::Debug for MediaProtectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaProtectionManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionManager;{45694947-c741-434b-a79e-474c12d93d2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
    const IID: ::windows_core::GUID = <IMediaProtectionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionManager";
}
impl ::core::convert::From<MediaProtectionManager> for ::windows_core::IUnknown {
    fn from(value: MediaProtectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionManager> for ::windows_core::IUnknown {
    fn from(value: &MediaProtectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaProtectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaProtectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProtectionManager> for ::windows_core::IInspectable {
    fn from(value: MediaProtectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionManager> for ::windows_core::IInspectable {
    fn from(value: &MediaProtectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaProtectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaProtectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaProtectionManager {}
unsafe impl ::core::marker::Sync for MediaProtectionManager {}
#[repr(transparent)]
pub struct MediaProtectionPMPServer(::windows_core::IUnknown);
impl MediaProtectionPMPServer {
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreatePMPServer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(pproperties: Param0) -> ::windows_core::Result<MediaProtectionPMPServer> {
        Self::IMediaProtectionPMPServerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePMPServer)(::windows_core::Interface::as_raw(this), pproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<MediaProtectionPMPServer>(result__)
        })
    }
    pub fn IMediaProtectionPMPServerFactory<R, F: FnOnce(&IMediaProtectionPMPServerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MediaProtectionPMPServer, IMediaProtectionPMPServerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaProtectionPMPServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionPMPServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionPMPServer {}
impl ::core::fmt::Debug for MediaProtectionPMPServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionPMPServer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaProtectionPMPServer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionPMPServer;{0c111226-7b26-4d31-95bb-9c1b08ef7fc0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
    const IID: ::windows_core::GUID = <IMediaProtectionPMPServer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionPMPServer";
}
impl ::core::convert::From<MediaProtectionPMPServer> for ::windows_core::IUnknown {
    fn from(value: MediaProtectionPMPServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionPMPServer> for ::windows_core::IUnknown {
    fn from(value: &MediaProtectionPMPServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaProtectionPMPServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaProtectionPMPServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProtectionPMPServer> for ::windows_core::IInspectable {
    fn from(value: MediaProtectionPMPServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionPMPServer> for ::windows_core::IInspectable {
    fn from(value: &MediaProtectionPMPServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaProtectionPMPServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaProtectionPMPServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaProtectionPMPServer {}
unsafe impl ::core::marker::Sync for MediaProtectionPMPServer {}
#[repr(transparent)]
pub struct MediaProtectionServiceCompletion(::windows_core::IUnknown);
impl MediaProtectionServiceCompletion {
    pub fn Complete(&self, success: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this), success).ok() }
    }
}
impl ::core::clone::Clone for MediaProtectionServiceCompletion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionServiceCompletion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionServiceCompletion {}
impl ::core::fmt::Debug for MediaProtectionServiceCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionServiceCompletion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaProtectionServiceCompletion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionServiceCompletion;{8b5cca18-cfd5-44ee-a2ed-df76010c14b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
    const IID: ::windows_core::GUID = <IMediaProtectionServiceCompletion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionServiceCompletion";
}
impl ::core::convert::From<MediaProtectionServiceCompletion> for ::windows_core::IUnknown {
    fn from(value: MediaProtectionServiceCompletion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionServiceCompletion> for ::windows_core::IUnknown {
    fn from(value: &MediaProtectionServiceCompletion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProtectionServiceCompletion> for ::windows_core::IInspectable {
    fn from(value: MediaProtectionServiceCompletion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProtectionServiceCompletion> for ::windows_core::IInspectable {
    fn from(value: &MediaProtectionServiceCompletion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaProtectionServiceCompletion {}
unsafe impl ::core::marker::Sync for MediaProtectionServiceCompletion {}
#[repr(transparent)]
pub struct ProtectionCapabilities(::windows_core::IUnknown);
impl ProtectionCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProtectionCapabilities, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsTypeSupported<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, r#type: Param0, keysystem: Param1) -> ::windows_core::Result<ProtectionCapabilityResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProtectionCapabilityResult>::zeroed();
            (::windows_core::Interface::vtable(this).IsTypeSupported)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), keysystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProtectionCapabilityResult>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectionCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionCapabilities {}
impl ::core::fmt::Debug for ProtectionCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ProtectionCapabilities;{c7ac5d7e-7480-4d29-a464-7bcd913dd8e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IProtectionCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.ProtectionCapabilities";
}
impl ::core::convert::From<ProtectionCapabilities> for ::windows_core::IUnknown {
    fn from(value: ProtectionCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionCapabilities> for ::windows_core::IUnknown {
    fn from(value: &ProtectionCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtectionCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtectionCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtectionCapabilities> for ::windows_core::IInspectable {
    fn from(value: ProtectionCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtectionCapabilities> for ::windows_core::IInspectable {
    fn from(value: &ProtectionCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtectionCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtectionCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtectionCapabilities {}
unsafe impl ::core::marker::Sync for ProtectionCapabilities {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionCapabilityResult {}
impl ::core::clone::Clone for ProtectionCapabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionCapabilityResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProtectionCapabilityResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionCapabilityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionCapabilityResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtectionCapabilityResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.ProtectionCapabilityResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RebootNeededEventHandler(pub ::windows_core::IUnknown);
impl RebootNeededEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RebootNeededEventHandlerBox::<F> { vtable: &RebootNeededEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, MediaProtectionManager>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RebootNeededEventHandlerBox<F: FnMut(&::core::option::Option<MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RebootNeededEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RebootNeededEventHandlerBox<F> {
    const VTABLE: RebootNeededEventHandler_Vtbl = RebootNeededEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<RebootNeededEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for RebootNeededEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RebootNeededEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RebootNeededEventHandler {}
impl ::core::fmt::Debug for RebootNeededEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RebootNeededEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RebootNeededEventHandler {
    type Vtable = RebootNeededEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e12a45_973b_4a3a_b260_91898a49a82c);
}
unsafe impl ::windows_core::RuntimeType for RebootNeededEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{64e12a45-973b-4a3a-b260-91898a49a82c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RebootNeededEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl ::core::marker::Copy for RenewalStatus {}
impl ::core::clone::Clone for RenewalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RenewalStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RenewalStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RenewalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenewalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RenewalStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RenewalStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RevocationAndRenewalInformation(::windows_core::IUnknown);
impl RevocationAndRenewalInformation {
    #[cfg(feature = "winrt-foundation")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<RevocationAndRenewalItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<RevocationAndRenewalItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for RevocationAndRenewalInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevocationAndRenewalInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevocationAndRenewalInformation {}
impl ::core::fmt::Debug for RevocationAndRenewalInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevocationAndRenewalInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalInformation;{f3a1937b-2501-439e-a6e7-6fc95e175fcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
    const IID: ::windows_core::GUID = <IRevocationAndRenewalInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalInformation";
}
impl ::core::convert::From<RevocationAndRenewalInformation> for ::windows_core::IUnknown {
    fn from(value: RevocationAndRenewalInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevocationAndRenewalInformation> for ::windows_core::IUnknown {
    fn from(value: &RevocationAndRenewalInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RevocationAndRenewalInformation> for ::windows_core::IInspectable {
    fn from(value: RevocationAndRenewalInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevocationAndRenewalInformation> for ::windows_core::IInspectable {
    fn from(value: &RevocationAndRenewalInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RevocationAndRenewalInformation {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalInformation {}
#[repr(transparent)]
pub struct RevocationAndRenewalItem(::windows_core::IUnknown);
impl RevocationAndRenewalItem {
    pub fn Reasons(&self) -> ::windows_core::Result<RevocationAndRenewalReasons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RevocationAndRenewalReasons>::zeroed();
            (::windows_core::Interface::vtable(this).Reasons)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RevocationAndRenewalReasons>(result__)
        }
    }
    pub fn HeaderHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderHash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PublicKeyHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PublicKeyHash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RenewalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RenewalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for RevocationAndRenewalItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevocationAndRenewalItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevocationAndRenewalItem {}
impl ::core::fmt::Debug for RevocationAndRenewalItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RevocationAndRenewalItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalItem;{3099c20c-3cf0-49ea-902d-caf32d2dde2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
    const IID: ::windows_core::GUID = <IRevocationAndRenewalItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalItem";
}
impl ::core::convert::From<RevocationAndRenewalItem> for ::windows_core::IUnknown {
    fn from(value: RevocationAndRenewalItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevocationAndRenewalItem> for ::windows_core::IUnknown {
    fn from(value: &RevocationAndRenewalItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RevocationAndRenewalItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RevocationAndRenewalItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RevocationAndRenewalItem> for ::windows_core::IInspectable {
    fn from(value: RevocationAndRenewalItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RevocationAndRenewalItem> for ::windows_core::IInspectable {
    fn from(value: &RevocationAndRenewalItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RevocationAndRenewalItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RevocationAndRenewalItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RevocationAndRenewalItem {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl ::core::marker::Copy for RevocationAndRenewalReasons {}
impl ::core::clone::Clone for RevocationAndRenewalReasons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RevocationAndRenewalReasons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RevocationAndRenewalReasons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RevocationAndRenewalReasons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalReasons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RevocationAndRenewalReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RevocationAndRenewalReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RevocationAndRenewalReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for RevocationAndRenewalReasons {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RevocationAndRenewalReasons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ServiceRequestedEventArgs(::windows_core::IUnknown);
impl ServiceRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<IMediaProtectionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IMediaProtectionServiceRequest>(result__)
        }
    }
    pub fn Completion(&self) -> ::windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Completion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaProtectionServiceCompletion>(result__)
        }
    }
    #[cfg(feature = "winrt-media")]
    pub fn MediaPlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IServiceRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
}
impl ::core::clone::Clone for ServiceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceRequestedEventArgs {}
impl ::core::fmt::Debug for ServiceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ServiceRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ServiceRequestedEventArgs;{34283baf-abb4-4fc1-bd89-93f106573a49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IServiceRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ServiceRequestedEventArgs";
}
impl ::core::convert::From<ServiceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ServiceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ServiceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ServiceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ServiceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ServiceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ServiceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ServiceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ServiceRequestedEventArgs {}
#[repr(transparent)]
pub struct ServiceRequestedEventHandler(pub ::windows_core::IUnknown);
impl ServiceRequestedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ServiceRequestedEventHandlerBox::<F> { vtable: &ServiceRequestedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, MediaProtectionManager>, Param1: ::windows_core::IntoParam<'a, ServiceRequestedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ServiceRequestedEventHandlerBox<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ServiceRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaProtectionManager>, &::core::option::Option<ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ServiceRequestedEventHandlerBox<F> {
    const VTABLE: ServiceRequestedEventHandler_Vtbl = ServiceRequestedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ServiceRequestedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ServiceRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceRequestedEventHandler {}
impl ::core::fmt::Debug for ServiceRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ServiceRequestedEventHandler {
    type Vtable = ServiceRequestedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2d690ba_cac9_48e1_95c0_d38495a84055);
}
unsafe impl ::windows_core::RuntimeType for ServiceRequestedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d2d690ba-cac9-48e1-95c0-d38495a84055}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ServiceRequestedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
