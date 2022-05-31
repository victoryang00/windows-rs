
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Deployment")]
pub mod Deployment;
#[cfg(feature = "Policies")]
pub mod Policies;
#[cfg(feature = "Update")]
pub mod Update;
#[cfg(feature = "Workplace")]
pub mod Workplace;
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmAlert(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMdmAlert {
    type Vtable = IMdmAlert_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0fbc327_28c1_4b52_a548_c5807caf70b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmAlert_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertDataType) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MdmAlertDataType) -> ::windows_core::HRESULT,
    pub Mark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertMark) -> ::windows_core::HRESULT,
    pub SetMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MdmAlertMark) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMdmSession {
    type Vtable = IMdmSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe89314c_8f64_4797_a9d7_9d88f86ae166);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Alerts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Alerts: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmSessionState) -> ::windows_core::HRESULT,
    pub AttachAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StartWithAlertsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StartWithAlertsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmSessionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMdmSessionManagerStatics {
    type Vtable = IMdmSessionManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf4ad959_f745_4b79_9b5c_de0bf8efe44b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmSessionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SessionIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SessionIds: usize,
    pub TryCreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteSessionById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetSessionById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MdmAlert(::windows_core::IUnknown);
impl MdmAlert {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MdmAlert, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Format(&self) -> ::windows_core::Result<MdmAlertDataType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MdmAlertDataType>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MdmAlertDataType>(result__)
        }
    }
    pub fn SetFormat(&self, value: MdmAlertDataType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Mark(&self) -> ::windows_core::Result<MdmAlertMark> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MdmAlertMark>::zeroed();
            (::windows_core::Interface::vtable(this).Mark)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MdmAlertMark>(result__)
        }
    }
    pub fn SetMark(&self, value: MdmAlertMark) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMark)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Target(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Target)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTarget<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MdmAlert {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MdmAlert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmAlert {}
impl ::core::fmt::Debug for MdmAlert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlert").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MdmAlert {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.MdmAlert;{b0fbc327-28c1-4b52-a548-c5807caf70b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MdmAlert {
    type Vtable = IMdmAlert_Vtbl;
    const IID: ::windows_core::GUID = <IMdmAlert as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MdmAlert {
    const NAME: &'static str = "Windows.Management.MdmAlert";
}
impl ::core::convert::From<MdmAlert> for ::windows_core::IUnknown {
    fn from(value: MdmAlert) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmAlert> for ::windows_core::IUnknown {
    fn from(value: &MdmAlert) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MdmAlert {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MdmAlert {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MdmAlert> for ::windows_core::IInspectable {
    fn from(value: MdmAlert) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmAlert> for ::windows_core::IInspectable {
    fn from(value: &MdmAlert) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MdmAlert {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MdmAlert {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmAlertDataType(pub i32);
impl MdmAlertDataType {
    pub const String: Self = Self(0i32);
    pub const Base64: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
}
impl ::core::marker::Copy for MdmAlertDataType {}
impl ::core::clone::Clone for MdmAlertDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmAlertDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MdmAlertDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmAlertDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertDataType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MdmAlertDataType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmAlertDataType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmAlertMark(pub i32);
impl MdmAlertMark {
    pub const None: Self = Self(0i32);
    pub const Fatal: Self = Self(1i32);
    pub const Critical: Self = Self(2i32);
    pub const Warning: Self = Self(3i32);
    pub const Informational: Self = Self(4i32);
}
impl ::core::marker::Copy for MdmAlertMark {}
impl ::core::clone::Clone for MdmAlertMark {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmAlertMark {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MdmAlertMark {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmAlertMark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertMark").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MdmAlertMark {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmAlertMark;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MdmSession(::windows_core::IUnknown);
impl MdmSession {
    #[cfg(feature = "winrt-foundation")]
    pub fn Alerts(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<MdmAlert>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Alerts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<MdmAlert>>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<MdmSessionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MdmSessionState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MdmSessionState>(result__)
        }
    }
    pub fn AttachAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn Delete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Delete)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StartWithAlertsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<MdmAlert>>>(&self, alerts: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartWithAlertsAsync)(::windows_core::Interface::as_raw(this), alerts.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for MdmSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MdmSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmSession {}
impl ::core::fmt::Debug for MdmSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MdmSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.MdmSession;{fe89314c-8f64-4797-a9d7-9d88f86ae166})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MdmSession {
    type Vtable = IMdmSession_Vtbl;
    const IID: ::windows_core::GUID = <IMdmSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MdmSession {
    const NAME: &'static str = "Windows.Management.MdmSession";
}
impl ::core::convert::From<MdmSession> for ::windows_core::IUnknown {
    fn from(value: MdmSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmSession> for ::windows_core::IUnknown {
    fn from(value: &MdmSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MdmSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MdmSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MdmSession> for ::windows_core::IInspectable {
    fn from(value: MdmSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmSession> for ::windows_core::IInspectable {
    fn from(value: &MdmSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MdmSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MdmSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct MdmSessionManager;
impl MdmSessionManager {
    #[cfg(feature = "winrt-foundation")]
    pub fn SessionIds() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SessionIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn TryCreateSession() -> ::windows_core::Result<MdmSession> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MdmSession>(result__)
        })
    }
    pub fn DeleteSessionById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sessionid: Param0) -> ::windows_core::Result<()> {
        Self::IMdmSessionManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).DeleteSessionById)(::windows_core::Interface::as_raw(this), sessionid.into_param().abi()).ok() })
    }
    pub fn GetSessionById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sessionid: Param0) -> ::windows_core::Result<MdmSession> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSessionById)(::windows_core::Interface::as_raw(this), sessionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<MdmSession>(result__)
        })
    }
    pub fn IMdmSessionManagerStatics<R, F: FnOnce(&IMdmSessionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MdmSessionManager, IMdmSessionManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MdmSessionManager {
    const NAME: &'static str = "Windows.Management.MdmSessionManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmSessionState(pub i32);
impl MdmSessionState {
    pub const NotStarted: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Communicating: Self = Self(3i32);
    pub const AlertStatusAvailable: Self = Self(4i32);
    pub const Retrying: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for MdmSessionState {}
impl ::core::clone::Clone for MdmSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmSessionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MdmSessionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSessionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MdmSessionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmSessionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
