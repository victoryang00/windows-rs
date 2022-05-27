#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryClientStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlatformTelemetryClientStatics {
    type Vtable = IPlatformTelemetryClientStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bf3f25d_d5c3_4eea_8dbe_9c8dbb0d9d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RegisterWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, settings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d8518ab_2292_49bd_a15a_3d71d2145112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x819a8582_ca19_415e_bb79_9c224bfa3a73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetStorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub UploadQuotaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetUploadQuotaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
pub struct PlatformTelemetryClient;
impl PlatformTelemetryClient {
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn Register<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn RegisterWithSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PlatformTelemetryRegistrationSettings>>(id: Param0, settings: Param1) -> ::windows_core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterWithSettings)(::windows_core::Interface::as_raw(this), id.into_param().abi(), settings.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlatformTelemetryClientStatics<R, F: FnOnce(&IPlatformTelemetryClientStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlatformTelemetryClient, IPlatformTelemetryClientStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PlatformTelemetryClient {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationResult(::windows_core::IUnknown);
impl PlatformTelemetryRegistrationResult {
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn Status(&self) -> ::windows_core::Result<PlatformTelemetryRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlatformTelemetryRegistrationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlatformTelemetryRegistrationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PlatformTelemetryRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformTelemetryRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformTelemetryRegistrationResult {}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlatformTelemetryRegistrationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult;{4d8518ab-2292-49bd-a15a-3d71d2145112})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_Vtbl;
    const IID: ::windows_core::GUID = <IPlatformTelemetryRegistrationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows_core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows_core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows_core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows_core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationResult {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationResult {}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationSettings(::windows_core::IUnknown);
impl PlatformTelemetryRegistrationSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlatformTelemetryRegistrationSettings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn StorageSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).StorageSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn SetStorageSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStorageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn UploadQuotaSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UploadQuotaSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn SetUploadQuotaSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUploadQuotaSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PlatformTelemetryRegistrationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformTelemetryRegistrationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformTelemetryRegistrationSettings {}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlatformTelemetryRegistrationSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings;{819a8582-ca19-415e-bb79-9c224bfa3a73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_Vtbl;
    const IID: ::windows_core::GUID = <IPlatformTelemetryRegistrationSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows_core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows_core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows_core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows_core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationSettings {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationSettings {}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformTelemetryRegistrationStatus {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformTelemetryRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlatformTelemetryRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlatformTelemetryRegistrationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
