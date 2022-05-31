#[cfg(feature = "DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "Telemetry")]
pub mod Telemetry;
#[cfg(feature = "TraceReporting")]
pub mod TraceReporting;
#[repr(transparent)]
pub struct DiagnosticActionResult(::windows_core::IUnknown);
impl DiagnosticActionResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Results(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Results)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for DiagnosticActionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiagnosticActionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticActionResult {}
impl ::core::fmt::Debug for DiagnosticActionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticActionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiagnosticActionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticActionResult;{c265a296-e73b-4097-b28f-3442f03dd831})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_Vtbl;
    const IID: ::windows_core::GUID = <IDiagnosticActionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticActionResult";
}
impl ::core::convert::From<DiagnosticActionResult> for ::windows_core::IUnknown {
    fn from(value: DiagnosticActionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticActionResult> for ::windows_core::IUnknown {
    fn from(value: &DiagnosticActionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiagnosticActionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiagnosticActionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiagnosticActionResult> for ::windows_core::IInspectable {
    fn from(value: DiagnosticActionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticActionResult> for ::windows_core::IInspectable {
    fn from(value: &DiagnosticActionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiagnosticActionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiagnosticActionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DiagnosticActionResult {}
unsafe impl ::core::marker::Sync for DiagnosticActionResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DiagnosticActionState(pub i32);
impl DiagnosticActionState {
    pub const Initializing: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const VerifyingTrust: Self = Self(2i32);
    pub const Detecting: Self = Self(3i32);
    pub const Resolving: Self = Self(4i32);
    pub const VerifyingResolution: Self = Self(5i32);
    pub const Executing: Self = Self(6i32);
}
impl ::core::marker::Copy for DiagnosticActionState {}
impl ::core::clone::Clone for DiagnosticActionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DiagnosticActionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DiagnosticActionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DiagnosticActionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticActionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiagnosticActionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DiagnosticActionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DiagnosticInvoker(::windows_core::IUnknown);
impl DiagnosticInvoker {
    #[cfg(feature = "Data_Json")]
    pub fn RunDiagnosticActionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Json::JsonObject>>(&self, context: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RunDiagnosticActionAsync)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    pub fn RunDiagnosticActionFromStringAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, context: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = &::windows_core::Interface::cast::<IDiagnosticInvoker2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RunDiagnosticActionFromStringAsync)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::User>>(user: Param0) -> ::windows_core::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<DiagnosticInvoker>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IDiagnosticInvokerStatics<R, F: FnOnce(&IDiagnosticInvokerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiagnosticInvoker, IDiagnosticInvokerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiagnosticInvoker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiagnosticInvoker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticInvoker {}
impl ::core::fmt::Debug for DiagnosticInvoker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticInvoker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiagnosticInvoker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DiagnosticInvoker;{187b270a-02e3-4f86-84fc-fdd892b5940f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_Vtbl;
    const IID: ::windows_core::GUID = <IDiagnosticInvoker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticInvoker";
}
impl ::core::convert::From<DiagnosticInvoker> for ::windows_core::IUnknown {
    fn from(value: DiagnosticInvoker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticInvoker> for ::windows_core::IUnknown {
    fn from(value: &DiagnosticInvoker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiagnosticInvoker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiagnosticInvoker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiagnosticInvoker> for ::windows_core::IInspectable {
    fn from(value: DiagnosticInvoker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiagnosticInvoker> for ::windows_core::IInspectable {
    fn from(value: &DiagnosticInvoker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiagnosticInvoker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiagnosticInvoker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DiagnosticInvoker {}
unsafe impl ::core::marker::Sync for DiagnosticInvoker {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticActionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc265a296_e73b_4097_b28f_3442f03dd831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticActionResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Results: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticInvoker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x187b270a_02e3_4f86_84fc_fdd892b5940f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Json")]
    pub RunDiagnosticActionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Json"))]
    RunDiagnosticActionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticInvoker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticInvoker2 {
    type Vtable = IDiagnosticInvoker2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bf945c_155a_4b52_a8ec_070c44f95000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvoker2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RunDiagnosticActionFromStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticInvokerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiagnosticInvokerStatics {
    type Vtable = IDiagnosticInvokerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cfad8de_f15c_4554_a813_c113c3881b09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticInvokerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessCpuUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessCpuUsage {
    type Vtable = IProcessCpuUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bbb2472_c8bf_423a_a810_b559ae4354e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessCpuUsageReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a6d9cac_3987_4e2f_a119_6b5fa214f1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessCpuUsageReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KernelTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UserTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiagnosticInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe830b04b_300e_4ee6_a0ab_5b5f5231b434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub DiskUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiagnosticInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiagnosticInfo2 {
    type Vtable = IProcessDiagnosticInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9558cb1a_3d0b_49ec_ab70_4f7a112805de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppDiagnosticInfos: usize,
    pub IsPackaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiagnosticInfoStatics {
    type Vtable = IProcessDiagnosticInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f41b260_b49f_428c_aa0e_84744f49ca95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForProcesses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForProcesses: usize,
    pub GetForCurrentProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiagnosticInfoStatics2 {
    type Vtable = IProcessDiagnosticInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a869897_9899_4a44_a29b_091663be09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiagnosticInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetForProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiskUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiskUsage {
    type Vtable = IProcessDiskUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ad78bfd_7e51_4e53_bfaa_5a6ee1aabbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessDiskUsageReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x401627fd_535d_4c1f_81b8_da54e1be635e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDiskUsageReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReadOperationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub WriteOperationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub OtherOperationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub BytesReadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub BytesWrittenCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub OtherBytesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessMemoryUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf50b229b_827c_42b7_b07c_0e32627e6b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessMemoryUsageReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2c77cba_1951_4685_8532_7e749ecf8eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryUsageReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NonPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PageFaultCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PageFileSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakNonPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakPageFileSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakVirtualMemorySizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakWorkingSetSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PrivatePageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub VirtualMemorySizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub WorkingSetSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemCpuUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemCpuUsage {
    type Vtable = ISystemCpuUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6037b3ac_02d6_4234_8362_7fe3adc81f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemCpuUsageReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c26d0b2_9483_4f62_ab57_82b29d9719b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCpuUsageReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KernelTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UserTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemDiagnosticInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa290fe05_dff3_407f_9a1b_0b2b317ca800);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemDiagnosticInfoStatics {
    type Vtable = ISystemDiagnosticInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd404ac21_fc7d_45f0_9a3f_39203aed9f7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemDiagnosticInfoStatics2 {
    type Vtable = ISystemDiagnosticInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79ded189_6af9_4da9_a422_15f73255b3eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDiagnosticInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsArchitectureSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PreferredArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ProcessorArchitecture) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMemoryUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17ffc595_1702_49cf_aa27_2f0a32591404);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMemoryUsageReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38663c87_2a9f_403a_bd19_2cf3e8169500);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMemoryUsageReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TotalPhysicalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AvailableSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub CommittedSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ProcessCpuUsage(::windows_core::IUnknown);
impl ProcessCpuUsage {
    pub fn GetReport(&self) -> ::windows_core::Result<ProcessCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessCpuUsageReport>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessCpuUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessCpuUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessCpuUsage {}
impl ::core::fmt::Debug for ProcessCpuUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessCpuUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessCpuUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsage;{0bbb2472-c8bf-423a-a810-b559ae4354e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessCpuUsage {
    type Vtable = IProcessCpuUsage_Vtbl;
    const IID: ::windows_core::GUID = <IProcessCpuUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsage";
}
impl ::core::convert::From<ProcessCpuUsage> for ::windows_core::IUnknown {
    fn from(value: ProcessCpuUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessCpuUsage> for ::windows_core::IUnknown {
    fn from(value: &ProcessCpuUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessCpuUsage> for ::windows_core::IInspectable {
    fn from(value: ProcessCpuUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessCpuUsage> for ::windows_core::IInspectable {
    fn from(value: &ProcessCpuUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessCpuUsage {}
unsafe impl ::core::marker::Sync for ProcessCpuUsage {}
#[repr(transparent)]
pub struct ProcessCpuUsageReport(::windows_core::IUnknown);
impl ProcessCpuUsageReport {
    pub fn KernelTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).KernelTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn UserTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UserTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessCpuUsageReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessCpuUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessCpuUsageReport {}
impl ::core::fmt::Debug for ProcessCpuUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessCpuUsageReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessCpuUsageReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessCpuUsageReport;{8a6d9cac-3987-4e2f-a119-6b5fa214f1b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_Vtbl;
    const IID: ::windows_core::GUID = <IProcessCpuUsageReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsageReport";
}
impl ::core::convert::From<ProcessCpuUsageReport> for ::windows_core::IUnknown {
    fn from(value: ProcessCpuUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessCpuUsageReport> for ::windows_core::IUnknown {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessCpuUsageReport> for ::windows_core::IInspectable {
    fn from(value: ProcessCpuUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessCpuUsageReport> for ::windows_core::IInspectable {
    fn from(value: &ProcessCpuUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessCpuUsageReport {}
unsafe impl ::core::marker::Sync for ProcessCpuUsageReport {}
#[repr(transparent)]
pub struct ProcessDiagnosticInfo(::windows_core::IUnknown);
impl ProcessDiagnosticInfo {
    pub fn ProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExecutableFileName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<ProcessDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessDiagnosticInfo>(result__)
        }
    }
    pub fn ProcessStartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn DiskUsage(&self) -> ::windows_core::Result<ProcessDiskUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DiskUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessDiskUsage>(result__)
        }
    }
    pub fn MemoryUsage(&self) -> ::windows_core::Result<ProcessMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MemoryUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessMemoryUsage>(result__)
        }
    }
    pub fn CpuUsage(&self) -> ::windows_core::Result<ProcessCpuUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CpuUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessCpuUsage>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppDiagnosticInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::AppDiagnosticInfo>> {
        let this = &::windows_core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppDiagnosticInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::AppDiagnosticInfo>>(result__)
        }
    }
    pub fn IsPackaged(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPackaged)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForProcesses() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ProcessDiagnosticInfo>> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForProcesses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ProcessDiagnosticInfo>>(result__)
        })
    }
    pub fn GetForCurrentProcess() -> ::windows_core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentProcess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    pub fn TryGetForProcessId(processid: u32) -> ::windows_core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetForProcessId)(::windows_core::Interface::as_raw(this), processid, result__.as_mut_ptr()).from_abi::<ProcessDiagnosticInfo>(result__)
        })
    }
    pub fn IProcessDiagnosticInfoStatics<R, F: FnOnce(&IProcessDiagnosticInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProcessDiagnosticInfoStatics2<R, F: FnOnce(&IProcessDiagnosticInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProcessDiagnosticInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiagnosticInfo {}
impl ::core::fmt::Debug for ProcessDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiagnosticInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessDiagnosticInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiagnosticInfo;{e830b04b-300e-4ee6-a0ab-5b5f5231b434})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = <IProcessDiagnosticInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiagnosticInfo";
}
impl ::core::convert::From<ProcessDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: ProcessDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: &ProcessDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessDiagnosticInfo {}
unsafe impl ::core::marker::Sync for ProcessDiagnosticInfo {}
#[repr(transparent)]
pub struct ProcessDiskUsage(::windows_core::IUnknown);
impl ProcessDiskUsage {
    pub fn GetReport(&self) -> ::windows_core::Result<ProcessDiskUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessDiskUsageReport>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessDiskUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessDiskUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiskUsage {}
impl ::core::fmt::Debug for ProcessDiskUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiskUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessDiskUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsage;{5ad78bfd-7e51-4e53-bfaa-5a6ee1aabbf8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessDiskUsage {
    type Vtable = IProcessDiskUsage_Vtbl;
    const IID: ::windows_core::GUID = <IProcessDiskUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsage";
}
impl ::core::convert::From<ProcessDiskUsage> for ::windows_core::IUnknown {
    fn from(value: ProcessDiskUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiskUsage> for ::windows_core::IUnknown {
    fn from(value: &ProcessDiskUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessDiskUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessDiskUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessDiskUsage> for ::windows_core::IInspectable {
    fn from(value: ProcessDiskUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiskUsage> for ::windows_core::IInspectable {
    fn from(value: &ProcessDiskUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessDiskUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessDiskUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessDiskUsage {}
unsafe impl ::core::marker::Sync for ProcessDiskUsage {}
#[repr(transparent)]
pub struct ProcessDiskUsageReport(::windows_core::IUnknown);
impl ProcessDiskUsageReport {
    pub fn ReadOperationCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).ReadOperationCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn WriteOperationCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).WriteOperationCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn OtherOperationCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).OtherOperationCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn BytesReadCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReadCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn BytesWrittenCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesWrittenCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn OtherBytesCount(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).OtherBytesCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessDiskUsageReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessDiskUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiskUsageReport {}
impl ::core::fmt::Debug for ProcessDiskUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiskUsageReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessDiskUsageReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessDiskUsageReport;{401627fd-535d-4c1f-81b8-da54e1be635e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_Vtbl;
    const IID: ::windows_core::GUID = <IProcessDiskUsageReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsageReport";
}
impl ::core::convert::From<ProcessDiskUsageReport> for ::windows_core::IUnknown {
    fn from(value: ProcessDiskUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiskUsageReport> for ::windows_core::IUnknown {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessDiskUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessDiskUsageReport> for ::windows_core::IInspectable {
    fn from(value: ProcessDiskUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessDiskUsageReport> for ::windows_core::IInspectable {
    fn from(value: &ProcessDiskUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessDiskUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessDiskUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessDiskUsageReport {}
unsafe impl ::core::marker::Sync for ProcessDiskUsageReport {}
#[repr(transparent)]
pub struct ProcessMemoryUsage(::windows_core::IUnknown);
impl ProcessMemoryUsage {
    pub fn GetReport(&self) -> ::windows_core::Result<ProcessMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessMemoryUsageReport>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessMemoryUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryUsage {}
impl ::core::fmt::Debug for ProcessMemoryUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessMemoryUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsage;{f50b229b-827c-42b7-b07c-0e32627e6b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_Vtbl;
    const IID: ::windows_core::GUID = <IProcessMemoryUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsage";
}
impl ::core::convert::From<ProcessMemoryUsage> for ::windows_core::IUnknown {
    fn from(value: ProcessMemoryUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryUsage> for ::windows_core::IUnknown {
    fn from(value: &ProcessMemoryUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessMemoryUsage> for ::windows_core::IInspectable {
    fn from(value: ProcessMemoryUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryUsage> for ::windows_core::IInspectable {
    fn from(value: &ProcessMemoryUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryUsage {}
unsafe impl ::core::marker::Sync for ProcessMemoryUsage {}
#[repr(transparent)]
pub struct ProcessMemoryUsageReport(::windows_core::IUnknown);
impl ProcessMemoryUsageReport {
    pub fn NonPagedPoolSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).NonPagedPoolSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PageFaultCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PageFaultCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PageFileSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PageFileSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PagedPoolSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PagedPoolSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakNonPagedPoolSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakNonPagedPoolSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPageFileSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakPageFileSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPagedPoolSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakPagedPoolSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakVirtualMemorySizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakVirtualMemorySizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakWorkingSetSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakWorkingSetSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PrivatePageCount(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PrivatePageCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VirtualMemorySizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).VirtualMemorySizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn WorkingSetSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).WorkingSetSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessMemoryUsageReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryUsageReport {}
impl ::core::fmt::Debug for ProcessMemoryUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryUsageReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessMemoryUsageReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.ProcessMemoryUsageReport;{c2c77cba-1951-4685-8532-7e749ecf8eeb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_Vtbl;
    const IID: ::windows_core::GUID = <IProcessMemoryUsageReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsageReport";
}
impl ::core::convert::From<ProcessMemoryUsageReport> for ::windows_core::IUnknown {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryUsageReport> for ::windows_core::IUnknown {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessMemoryUsageReport> for ::windows_core::IInspectable {
    fn from(value: ProcessMemoryUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryUsageReport> for ::windows_core::IInspectable {
    fn from(value: &ProcessMemoryUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryUsageReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryUsageReport {}
#[repr(transparent)]
pub struct SystemCpuUsage(::windows_core::IUnknown);
impl SystemCpuUsage {
    pub fn GetReport(&self) -> ::windows_core::Result<SystemCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemCpuUsageReport>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemCpuUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemCpuUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCpuUsage {}
impl ::core::fmt::Debug for SystemCpuUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemCpuUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemCpuUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsage;{6037b3ac-02d6-4234-8362-7fe3adc81f5f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemCpuUsage {
    type Vtable = ISystemCpuUsage_Vtbl;
    const IID: ::windows_core::GUID = <ISystemCpuUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsage";
}
impl ::core::convert::From<SystemCpuUsage> for ::windows_core::IUnknown {
    fn from(value: SystemCpuUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCpuUsage> for ::windows_core::IUnknown {
    fn from(value: &SystemCpuUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemCpuUsage> for ::windows_core::IInspectable {
    fn from(value: SystemCpuUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCpuUsage> for ::windows_core::IInspectable {
    fn from(value: &SystemCpuUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemCpuUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemCpuUsage {}
unsafe impl ::core::marker::Sync for SystemCpuUsage {}
#[repr(transparent)]
pub struct SystemCpuUsageReport(::windows_core::IUnknown);
impl SystemCpuUsageReport {
    pub fn KernelTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).KernelTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn UserTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UserTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn IdleTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).IdleTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemCpuUsageReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemCpuUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCpuUsageReport {}
impl ::core::fmt::Debug for SystemCpuUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemCpuUsageReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemCpuUsageReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemCpuUsageReport;{2c26d0b2-9483-4f62-ab57-82b29d9719b8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_Vtbl;
    const IID: ::windows_core::GUID = <ISystemCpuUsageReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsageReport";
}
impl ::core::convert::From<SystemCpuUsageReport> for ::windows_core::IUnknown {
    fn from(value: SystemCpuUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCpuUsageReport> for ::windows_core::IUnknown {
    fn from(value: &SystemCpuUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemCpuUsageReport> for ::windows_core::IInspectable {
    fn from(value: SystemCpuUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemCpuUsageReport> for ::windows_core::IInspectable {
    fn from(value: &SystemCpuUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemCpuUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemCpuUsageReport {}
unsafe impl ::core::marker::Sync for SystemCpuUsageReport {}
#[repr(transparent)]
pub struct SystemDiagnosticInfo(::windows_core::IUnknown);
impl SystemDiagnosticInfo {
    pub fn MemoryUsage(&self) -> ::windows_core::Result<SystemMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MemoryUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMemoryUsage>(result__)
        }
    }
    pub fn CpuUsage(&self) -> ::windows_core::Result<SystemCpuUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CpuUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemCpuUsage>(result__)
        }
    }
    pub fn GetForCurrentSystem() -> ::windows_core::Result<SystemDiagnosticInfo> {
        Self::ISystemDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemDiagnosticInfo>(result__)
        })
    }
    pub fn IsArchitectureSupported(r#type: super::ProcessorArchitecture) -> ::windows_core::Result<bool> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsArchitectureSupported)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn PreferredArchitecture() -> ::windows_core::Result<super::ProcessorArchitecture> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ProcessorArchitecture>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredArchitecture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ProcessorArchitecture>(result__)
        })
    }
    pub fn ISystemDiagnosticInfoStatics<R, F: FnOnce(&ISystemDiagnosticInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemDiagnosticInfoStatics2<R, F: FnOnce(&ISystemDiagnosticInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemDiagnosticInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemDiagnosticInfo {}
impl ::core::fmt::Debug for SystemDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemDiagnosticInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemDiagnosticInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemDiagnosticInfo;{a290fe05-dff3-407f-9a1b-0b2b317ca800})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISystemDiagnosticInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemDiagnosticInfo";
}
impl ::core::convert::From<SystemDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: SystemDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: SystemDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: &SystemDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemDiagnosticInfo {}
unsafe impl ::core::marker::Sync for SystemDiagnosticInfo {}
#[repr(transparent)]
pub struct SystemMemoryUsage(::windows_core::IUnknown);
impl SystemMemoryUsage {
    pub fn GetReport(&self) -> ::windows_core::Result<SystemMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemMemoryUsageReport>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMemoryUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMemoryUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMemoryUsage {}
impl ::core::fmt::Debug for SystemMemoryUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMemoryUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMemoryUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsage;{17ffc595-1702-49cf-aa27-2f0a32591404})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMemoryUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsage";
}
impl ::core::convert::From<SystemMemoryUsage> for ::windows_core::IUnknown {
    fn from(value: SystemMemoryUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMemoryUsage> for ::windows_core::IUnknown {
    fn from(value: &SystemMemoryUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMemoryUsage> for ::windows_core::IInspectable {
    fn from(value: SystemMemoryUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMemoryUsage> for ::windows_core::IInspectable {
    fn from(value: &SystemMemoryUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMemoryUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMemoryUsage {}
unsafe impl ::core::marker::Sync for SystemMemoryUsage {}
#[repr(transparent)]
pub struct SystemMemoryUsageReport(::windows_core::IUnknown);
impl SystemMemoryUsageReport {
    pub fn TotalPhysicalSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalPhysicalSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn AvailableSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn CommittedSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).CommittedSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMemoryUsageReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMemoryUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMemoryUsageReport {}
impl ::core::fmt::Debug for SystemMemoryUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMemoryUsageReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemMemoryUsageReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.SystemMemoryUsageReport;{38663c87-2a9f-403a-bd19-2cf3e8169500})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_Vtbl;
    const IID: ::windows_core::GUID = <ISystemMemoryUsageReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsageReport";
}
impl ::core::convert::From<SystemMemoryUsageReport> for ::windows_core::IUnknown {
    fn from(value: SystemMemoryUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMemoryUsageReport> for ::windows_core::IUnknown {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMemoryUsageReport> for ::windows_core::IInspectable {
    fn from(value: SystemMemoryUsageReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMemoryUsageReport> for ::windows_core::IInspectable {
    fn from(value: &SystemMemoryUsageReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemMemoryUsageReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMemoryUsageReport {}
unsafe impl ::core::marker::Sync for SystemMemoryUsageReport {}
