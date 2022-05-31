#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDClient(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDClient {
    type Vtable = INDClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub RegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RegistrationCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveRegistrationCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub ProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ProximityDetectionCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveProximityDetectionCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub LicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    LicenseFetchCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveLicenseFetchCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub ReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ReRegistrationNeeded: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveReRegistrationNeeded: usize,
    #[cfg(feature = "winrt-")]
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ClosedCaptionDataReceived: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveClosedCaptionDataReceived: usize,
    #[cfg(feature = "winrt-")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturl: ::windows_core::RawPtr, startasyncoptions: u32, registrationcustomdata: ::windows_core::RawPtr, licensefetchdescriptor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    StartAsync: usize,
    #[cfg(feature = "winrt-")]
    pub LicenseFetchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    LicenseFetchAsync: usize,
    #[cfg(feature = "winrt-")]
    pub ReRegistrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registrationcustomdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ReRegistrationAsync: usize,
    #[cfg(feature = "winrt-")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Close: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDClientFactory(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDClientFactory {
    type Vtable = INDClientFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadengine: ::windows_core::RawPtr, streamparser: ::windows_core::RawPtr, pmessenger: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateInstance: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDClosedCaptionDataReceivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDClosedCaptionDataReceivedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn ClosedCaptionDataFormat(&self) -> ::windows_core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NDClosedCaptionFormat>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionDataFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDClosedCaptionFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PresentationTimestamp(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).PresentationTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ClosedCaptionData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDClosedCaptionDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: INDClosedCaptionDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDClosedCaptionDataReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &INDClosedCaptionDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDClosedCaptionDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: INDClosedCaptionDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDClosedCaptionDataReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &INDClosedCaptionDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDClosedCaptionDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDClosedCaptionDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDClosedCaptionDataReceivedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDClosedCaptionDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDClosedCaptionDataReceivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4738d29f-c345-4649-8468-b8c5fc357190}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDClosedCaptionDataReceivedEventArgs {
    type Vtable = INDClosedCaptionDataReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4738d29f_c345_4649_8468_b8c5fc357190);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ClosedCaptionDataFormat: usize,
    #[cfg(feature = "winrt-")]
    pub PresentationTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PresentationTimestamp: usize,
    #[cfg(feature = "winrt-")]
    pub ClosedCaptionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ClosedCaptionData: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDCustomData(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDCustomData {
    #[cfg(feature = "winrt-")]
    pub fn CustomDataTypeID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomDataTypeID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CustomData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDCustomData> for ::windows_core::IUnknown {
    fn from(value: INDCustomData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDCustomData> for ::windows_core::IUnknown {
    fn from(value: &INDCustomData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDCustomData> for ::windows_core::IInspectable {
    fn from(value: INDCustomData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDCustomData> for ::windows_core::IInspectable {
    fn from(value: &INDCustomData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDCustomData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDCustomData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDCustomData {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDCustomData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDCustomData").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDCustomData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDCustomData {
    type Vtable = INDCustomData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CustomDataTypeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CustomDataTypeID: usize,
    #[cfg(feature = "winrt-")]
    pub CustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDCustomDataFactory(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDCustomDataFactory {
    type Vtable = INDCustomDataFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd65405ab_3424_4833_8c9a_af5fdeb22872);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateInstance: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDDownloadEngine(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDDownloadEngine {
    #[cfg(feature = "winrt-")]
    pub fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0, sessionidbytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Open)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr())).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Seek<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, startposition: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), startposition.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn BufferFullMinThresholdInSamples(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BufferFullMinThresholdInSamples)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn BufferFullMaxThresholdInSamples(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BufferFullMaxThresholdInSamples)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Notifier(&self) -> ::windows_core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Notifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDDownloadEngineNotifier>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDDownloadEngine> for ::windows_core::IUnknown {
    fn from(value: INDDownloadEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDDownloadEngine> for ::windows_core::IUnknown {
    fn from(value: &INDDownloadEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDDownloadEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDDownloadEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDDownloadEngine> for ::windows_core::IInspectable {
    fn from(value: INDDownloadEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDDownloadEngine> for ::windows_core::IInspectable {
    fn from(value: &INDDownloadEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDDownloadEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDDownloadEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDDownloadEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDDownloadEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDDownloadEngine {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDDownloadEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDDownloadEngine").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2d223d65-c4b6-4438-8d46-b96e6d0fb21f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDDownloadEngine {
    type Vtable = INDDownloadEngine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Open: usize,
    #[cfg(feature = "winrt-")]
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Pause: usize,
    #[cfg(feature = "winrt-")]
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Resume: usize,
    #[cfg(feature = "winrt-")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Close: usize,
    #[cfg(feature = "winrt-")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startposition: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Seek: usize,
    #[cfg(feature = "winrt-")]
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanSeek: usize,
    #[cfg(feature = "winrt-")]
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BufferFullMinThresholdInSamples: usize,
    #[cfg(feature = "winrt-")]
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BufferFullMaxThresholdInSamples: usize,
    #[cfg(feature = "winrt-")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Notifier: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDDownloadEngineNotifier(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDDownloadEngineNotifier {
    #[cfg(feature = "winrt-")]
    pub fn OnStreamOpened(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnStreamOpened)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(::windows_core::Interface::as_raw(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnContentIDReceived<'a, Param0: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnDataReceived)(::windows_core::Interface::as_raw(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnEndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnEndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnNetworkError(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnNetworkError)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDDownloadEngineNotifier> for ::windows_core::IUnknown {
    fn from(value: INDDownloadEngineNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDDownloadEngineNotifier> for ::windows_core::IUnknown {
    fn from(value: &INDDownloadEngineNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDDownloadEngineNotifier> for ::windows_core::IInspectable {
    fn from(value: INDDownloadEngineNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDDownloadEngineNotifier> for ::windows_core::IInspectable {
    fn from(value: &INDDownloadEngineNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDDownloadEngineNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDDownloadEngineNotifier {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDDownloadEngineNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDDownloadEngineNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d720b4d4-f4b8-4530-a809-9193a571e7fc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub OnStreamOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnStreamOpened: usize,
    #[cfg(feature = "winrt-")]
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnPlayReadyObjectReceived: usize,
    #[cfg(feature = "winrt-")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnContentIDReceived: usize,
    #[cfg(feature = "winrt-")]
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnDataReceived: usize,
    #[cfg(feature = "winrt-")]
    pub OnEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnEndOfStream: usize,
    #[cfg(feature = "winrt-")]
    pub OnNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnNetworkError: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDLicenseFetchCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDLicenseFetchCompletedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDCustomData>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: INDLicenseFetchCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &INDLicenseFetchCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: INDLicenseFetchCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &INDLicenseFetchCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDLicenseFetchCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDLicenseFetchCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDLicenseFetchCompletedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDLicenseFetchCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1ee30a1a-11b2-4558-8865-e3a516922517}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDLicenseFetchCompletedEventArgs {
    type Vtable = INDLicenseFetchCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ee30a1a_11b2_4558_8865_e3a516922517);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDLicenseFetchDescriptor(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDLicenseFetchDescriptor {
    #[cfg(feature = "winrt-")]
    pub fn ContentIDType(&self) -> ::windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NDContentIDType>::zeroed();
            (::windows_core::Interface::vtable(this).ContentIDType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ContentID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchDescriptor> for ::windows_core::IUnknown {
    fn from(value: INDLicenseFetchDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchDescriptor> for ::windows_core::IUnknown {
    fn from(value: &INDLicenseFetchDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchDescriptor> for ::windows_core::IInspectable {
    fn from(value: INDLicenseFetchDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchDescriptor> for ::windows_core::IInspectable {
    fn from(value: &INDLicenseFetchDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDLicenseFetchDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDLicenseFetchDescriptor {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDLicenseFetchDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5498d33a-e686-4935-a567-7ca77ad20fa4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5498d33a_e686_4935_a567_7ca77ad20fa4);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ContentIDType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ContentIDType: usize,
    #[cfg(feature = "winrt-")]
    pub ContentID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ContentID: usize,
    #[cfg(feature = "winrt-")]
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    LicenseFetchChallengeCustomData: usize,
    #[cfg(feature = "winrt-")]
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetLicenseFetchChallengeCustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDLicenseFetchDescriptorFactory(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDLicenseFetchDescriptorFactory {
    type Vtable = INDLicenseFetchDescriptorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0031202_cfac_4f00_ae6a_97af80b848f2);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateInstance: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDLicenseFetchResult(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDLicenseFetchResult {
    #[cfg(feature = "winrt-")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDCustomData>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchResult> for ::windows_core::IUnknown {
    fn from(value: INDLicenseFetchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchResult> for ::windows_core::IUnknown {
    fn from(value: &INDLicenseFetchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDLicenseFetchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDLicenseFetchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDLicenseFetchResult> for ::windows_core::IInspectable {
    fn from(value: INDLicenseFetchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDLicenseFetchResult> for ::windows_core::IInspectable {
    fn from(value: &INDLicenseFetchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDLicenseFetchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDLicenseFetchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDLicenseFetchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDLicenseFetchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDLicenseFetchResult {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDLicenseFetchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{21d39698-aa62-45ff-a5ff-8037e5433825}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDLicenseFetchResult {
    type Vtable = INDLicenseFetchResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21d39698_aa62_45ff_a5ff_8037e5433825);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDMessenger(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDMessenger {
    #[cfg(feature = "winrt-")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), responsedatabytes.len() as u32, ::core::mem::transmute(responsedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDMessenger> for ::windows_core::IUnknown {
    fn from(value: INDMessenger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDMessenger> for ::windows_core::IUnknown {
    fn from(value: &INDMessenger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDMessenger> for ::windows_core::IInspectable {
    fn from(value: INDMessenger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDMessenger> for ::windows_core::IInspectable {
    fn from(value: &INDMessenger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDMessenger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDMessenger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDMessenger {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDMessenger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDMessenger").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDMessenger {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d42df95d-a75b-47bf-8249-bc83820da38a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDMessenger {
    type Vtable = INDMessenger_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd42df95d_a75b_47bf_8249_bc83820da38a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SendRegistrationRequestAsync: usize,
    #[cfg(feature = "winrt-")]
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SendProximityDetectionStartAsync: usize,
    #[cfg(feature = "winrt-")]
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SendProximityDetectionResponseAsync: usize,
    #[cfg(feature = "winrt-")]
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SendLicenseFetchRequestAsync: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDProximityDetectionCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDProximityDetectionCompletedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn ProximityDetectionRetryCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ProximityDetectionRetryCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDProximityDetectionCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: INDProximityDetectionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDProximityDetectionCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &INDProximityDetectionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDProximityDetectionCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: INDProximityDetectionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDProximityDetectionCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &INDProximityDetectionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDProximityDetectionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDProximityDetectionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDProximityDetectionCompletedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDProximityDetectionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDProximityDetectionCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2a706328-da25-4f8c-9eb7-5d0fc3658bca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDProximityDetectionCompletedEventArgs {
    type Vtable = INDProximityDetectionCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ProximityDetectionRetryCount: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDRegistrationCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDRegistrationCompletedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TransmitterProperties(&self) -> ::windows_core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransmitterProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDTransmitterProperties>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TransmitterCertificateAccepted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TransmitterCertificateAccepted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransmitterCertificateAccepted)(::windows_core::Interface::as_raw(this), accept).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDRegistrationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: INDRegistrationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDRegistrationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &INDRegistrationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDRegistrationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: INDRegistrationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDRegistrationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &INDRegistrationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDRegistrationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDRegistrationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDRegistrationCompletedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDRegistrationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDRegistrationCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9e39b64d-ab5b-4905-acdc-787a77c6374d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDRegistrationCompletedEventArgs {
    type Vtable = INDRegistrationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ResponseCustomData: usize,
    #[cfg(feature = "winrt-")]
    pub TransmitterProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TransmitterProperties: usize,
    #[cfg(feature = "winrt-")]
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TransmitterCertificateAccepted: usize,
    #[cfg(feature = "winrt-")]
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTransmitterCertificateAccepted: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDSendResult(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDSendResult {
    #[cfg(feature = "winrt-")]
    pub fn Response(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDSendResult> for ::windows_core::IUnknown {
    fn from(value: INDSendResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDSendResult> for ::windows_core::IUnknown {
    fn from(value: &INDSendResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDSendResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDSendResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDSendResult> for ::windows_core::IInspectable {
    fn from(value: INDSendResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDSendResult> for ::windows_core::IInspectable {
    fn from(value: &INDSendResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDSendResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDSendResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDSendResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDSendResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDSendResult {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDSendResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDSendResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDSendResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e3685517-a584-479d-90b7-d689c7bf7c80}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDSendResult {
    type Vtable = INDSendResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3685517_a584_479d_90b7_d689c7bf7c80);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Response: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDStartResult(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDStartResult {
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn MediaStreamSource(&self) -> ::windows_core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::MediaStreamSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStartResult> for ::windows_core::IUnknown {
    fn from(value: INDStartResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStartResult> for ::windows_core::IUnknown {
    fn from(value: &INDStartResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDStartResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDStartResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStartResult> for ::windows_core::IInspectable {
    fn from(value: INDStartResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStartResult> for ::windows_core::IInspectable {
    fn from(value: &INDStartResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDStartResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDStartResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDStartResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDStartResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDStartResult {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDStartResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStartResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDStartResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{79f6e96e-f50f-4015-8ba4-c2bc344ebd4e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDStartResult {
    type Vtable = INDStartResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    MediaStreamSource: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDStorageFileHelper(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDStorageFileHelper {
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage", feature = "winrt-"))]
    pub fn GetFileURLs<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileURLs)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStorageFileHelper> for ::windows_core::IUnknown {
    fn from(value: INDStorageFileHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStorageFileHelper> for ::windows_core::IUnknown {
    fn from(value: &INDStorageFileHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStorageFileHelper> for ::windows_core::IInspectable {
    fn from(value: INDStorageFileHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStorageFileHelper> for ::windows_core::IInspectable {
    fn from(value: &INDStorageFileHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDStorageFileHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDStorageFileHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDStorageFileHelper {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDStorageFileHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStorageFileHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage", feature = "winrt-"))]
    pub GetFileURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage", feature = "winrt-")))]
    GetFileURLs: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDStreamParser(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDStreamParser {
    #[cfg(feature = "winrt-")]
    pub fn ParseData(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseData)(::windows_core::Interface::as_raw(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn GetStreamInformation<'a, Param0: ::windows_core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>>(&self, descriptor: Param0, streamtype: &mut NDMediaStreamType) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetStreamInformation)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), streamtype, result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn BeginOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BeginOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn EndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Notifier(&self) -> ::windows_core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Notifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDStreamParserNotifier>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStreamParser> for ::windows_core::IUnknown {
    fn from(value: INDStreamParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStreamParser> for ::windows_core::IUnknown {
    fn from(value: &INDStreamParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDStreamParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDStreamParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStreamParser> for ::windows_core::IInspectable {
    fn from(value: INDStreamParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStreamParser> for ::windows_core::IInspectable {
    fn from(value: &INDStreamParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDStreamParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDStreamParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDStreamParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDStreamParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDStreamParser {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDStreamParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStreamParser").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDStreamParser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e0baa198-9796-41c9-8695-59437e67e66a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDStreamParser {
    type Vtable = INDStreamParser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0baa198_9796_41c9_8695_59437e67e66a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ParseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ParseData: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub GetStreamInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows_core::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    GetStreamInformation: usize,
    #[cfg(feature = "winrt-")]
    pub BeginOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BeginOfStream: usize,
    #[cfg(feature = "winrt-")]
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EndOfStream: usize,
    #[cfg(feature = "winrt-")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Notifier: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDStreamParserNotifier(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDStreamParserNotifier {
    #[cfg(feature = "winrt-")]
    pub fn OnContentIDReceived<'a, Param0: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media", feature = "winrt-"))]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(::windows_core::Interface::as_raw(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn OnSampleParsed<'a, Param2: ::windows_core::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnSampleParsed)(::windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::core::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::core::mem::transmute(probytes.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStreamParserNotifier> for ::windows_core::IUnknown {
    fn from(value: INDStreamParserNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStreamParserNotifier> for ::windows_core::IUnknown {
    fn from(value: &INDStreamParserNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDStreamParserNotifier> for ::windows_core::IInspectable {
    fn from(value: INDStreamParserNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDStreamParserNotifier> for ::windows_core::IInspectable {
    fn from(value: &INDStreamParserNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDStreamParserNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDStreamParserNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDStreamParserNotifier {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDStreamParserNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStreamParserNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c167acd0-2ce6-426c-ace5-5e9275fea715}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc167acd0_2ce6_426c_ace5_5e9275fea715);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OnContentIDReceived: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media", feature = "winrt-"))]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostreamdescriptors: ::windows_core::RawPtr, videostreamdescriptors: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-media", feature = "winrt-")))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub OnSampleParsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows_core::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    OnSampleParsed: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows_core::RawPtr, keyid: ::windows_core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    OnBeginSetupDecryptor: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDTCPMessengerFactory(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDTCPMessengerFactory {
    type Vtable = INDTCPMessengerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, remotehostport: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateInstance: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct INDTransmitterProperties(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl INDTransmitterProperties {
    #[cfg(feature = "winrt-")]
    pub fn CertificateType(&self) -> ::windows_core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NDCertificateType>::zeroed();
            (::windows_core::Interface::vtable(this).CertificateType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDCertificateType>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PlatformIdentifier(&self) -> ::windows_core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NDCertificatePlatformID>::zeroed();
            (::windows_core::Interface::vtable(this).PlatformIdentifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDCertificatePlatformID>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SupportedFeatures(&self) -> ::windows_core::Result<::windows_core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<NDCertificateFeature>>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFeatures)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<NDCertificateFeature>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SecurityLevel(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SecurityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SecurityVersion(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SecurityVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ClientID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).ClientID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelDigest(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelDigest)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelManufacturerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelManufacturerName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDTransmitterProperties> for ::windows_core::IUnknown {
    fn from(value: INDTransmitterProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDTransmitterProperties> for ::windows_core::IUnknown {
    fn from(value: &INDTransmitterProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INDTransmitterProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INDTransmitterProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<INDTransmitterProperties> for ::windows_core::IInspectable {
    fn from(value: INDTransmitterProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&INDTransmitterProperties> for ::windows_core::IInspectable {
    fn from(value: &INDTransmitterProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INDTransmitterProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INDTransmitterProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for INDTransmitterProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for INDTransmitterProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for INDTransmitterProperties {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for INDTransmitterProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDTransmitterProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e536af23-ac4f-4adc-8c66-4ff7c2702dd6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for INDTransmitterProperties {
    type Vtable = INDTransmitterProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CertificateType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CertificateType: usize,
    #[cfg(feature = "winrt-")]
    pub PlatformIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PlatformIdentifier: usize,
    #[cfg(feature = "winrt-")]
    pub SupportedFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SupportedFeatures: usize,
    #[cfg(feature = "winrt-")]
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SecurityLevel: usize,
    #[cfg(feature = "winrt-")]
    pub SecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SecurityVersion: usize,
    #[cfg(feature = "winrt-")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ExpirationDate: usize,
    #[cfg(feature = "winrt-")]
    pub ClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ClientID: usize,
    #[cfg(feature = "winrt-")]
    pub ModelDigest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ModelDigest: usize,
    #[cfg(feature = "winrt-")]
    pub ModelManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ModelManufacturerName: usize,
    #[cfg(feature = "winrt-")]
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ModelName: usize,
    #[cfg(feature = "winrt-")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ModelNumber: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows_core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyDecryptorSetup) -> ::windows_core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeader2 {
    type Vtable = IPlayReadyContentHeader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x359c79f4_2180_498c_965b_e754d875eab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeaderFactory {
    type Vtable = IPlayReadyContentHeaderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: ::windows_core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows_core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceFromComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentkeyid: ::windows_core::GUID, contentkeyidstring: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows_core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows_core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeaderFactory2 {
    type Vtable = IPlayReadyContentHeaderFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: *const ::windows_core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows_core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows_core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentResolver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentResolver {
    type Vtable = IPlayReadyContentResolver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayReadyDomain(::windows_core::IUnknown);
impl IPlayReadyDomain {
    pub fn AccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DomainJoinUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::convert::From<IPlayReadyDomain> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyDomain> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyDomain> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyDomain> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPlayReadyDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyDomain {}
impl ::core::fmt::Debug for IPlayReadyDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyDomain").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DomainJoinUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainIterableFactory {
    type Vtable = IPlayReadyDomainIterableFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domainaccountid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainJoinServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x171b4a5a_405f_4739_b040_67b9f0c38758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainLeaveServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x062d58be_97ad_4917_aa03_46d4c252d464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyITADataGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24446b8e_10b9_4530_b25b_901a8029a9b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GenerateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcpsystemid: ::windows_core::GUID, countofstreams: u32, configuration: ::windows_core::RawPtr, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GenerateData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyIndividualizationServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct IPlayReadyLicense(::windows_core::IUnknown);
impl IPlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).FullyEvaluated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UsableForPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountID)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ChainDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GetKIDAtChainDepth)(::windows_core::Interface::as_raw(this), chaindepth, result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::convert::From<IPlayReadyLicense> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicense> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyLicense> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicense> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPlayReadyLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicense {}
impl ::core::fmt::Debug for IPlayReadyLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FullyEvaluated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub UsableForPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpireAfterFirstPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DomainAccountID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetKIDAtChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicense2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicense2 {
    type Vtable = IPlayReadyLicense2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f4e7a7_d8e3_48a0_bcda_ff9f40530436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SecureStopId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(::windows_core::IUnknown);
impl IPlayReadyLicenseAcquisitionServiceRequest {
    pub fn ContentHeader(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentHeader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn SetContentHeader<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentHeader)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::convert::From<IPlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseAcquisitionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::fmt::Debug for IPlayReadyLicenseAcquisitionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseAcquisitionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7fa5eb5_fe0c_b225_bc60_5a9edd32ceb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: ::windows_core::RawPtr, fullyevaluated: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseIterableFactory {
    type Vtable = IPlayReadyLicenseIterableFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4179f08_0837_4978_8e68_be4293c8d7a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: ::windows_core::RawPtr, fullyevaluated: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseManagement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseManagement {
    type Vtable = IPlayReadyLicenseManagement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeleteLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayReadyLicenseSession(::windows_core::IUnknown);
impl IPlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyLicenseSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyLicenseSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyLicenseSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyLicenseSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPlayReadyLicenseSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseSession {}
impl ::core::fmt::Debug for IPlayReadyLicenseSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a1723a39-87fa-4fdd-abbb-a9720e845259}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1723a39_87fa_4fdd_abbb_a9720e845259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateLAServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mpm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayReadyLicenseSession2(::windows_core::IUnknown);
impl IPlayReadyLicenseSession2 {
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, result__.as_mut_ptr()).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession2> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyLicenseSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession2> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyLicenseSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession2> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyLicenseSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession2> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyLicenseSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadyLicenseSession2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadyLicenseSession2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession> for &IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession> {
        ::core::convert::TryInto::<IPlayReadyLicenseSession>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPlayReadyLicenseSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseSession2 {}
impl ::core::fmt::Debug for IPlayReadyLicenseSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseSession2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4909be3a-3aed-4656-8ad7-ee0fd7799510}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSession2 {
    type Vtable = IPlayReadyLicenseSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: ::windows_core::RawPtr, fullyevaluated: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSessionFactory {
    type Vtable = IPlayReadyLicenseSessionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62492699_6527_429e_98be_48d798ac2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyMeteringReportServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetMeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyRevocationServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543d66ac_faf0_4560_84a5_0e4acec939e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySecureStopIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySecureStopIterableFactory {
    type Vtable = IPlayReadySecureStopIterableFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateInstance: usize,
}
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequest(::windows_core::IUnknown);
impl IPlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SessionID)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn UpdateTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).PublisherCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::convert::From<IPlayReadySecureStopServiceRequest> for ::windows_core::IUnknown {
    fn from(value: IPlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadySecureStopServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadySecureStopServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadySecureStopServiceRequest> for ::windows_core::IInspectable {
    fn from(value: IPlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadySecureStopServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadySecureStopServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadySecureStopServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadySecureStopServiceRequest {}
impl ::core::fmt::Debug for IPlayReadySecureStopServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadySecureStopServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b5501ee5-01bf-4401-9677-05630a6a4cc8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PublisherCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySecureStopServiceRequestFactory {
    type Vtable = IPlayReadySecureStopServiceRequestFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e448ac9_e67e_494e_9f49_6285438c76cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPlayReadyServiceRequest(::windows_core::IUnknown);
impl IPlayReadyServiceRequest {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::convert::From<IPlayReadyServiceRequest> for ::windows_core::IUnknown {
    fn from(value: IPlayReadyServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &IPlayReadyServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPlayReadyServiceRequest> for ::windows_core::IInspectable {
    fn from(value: IPlayReadyServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlayReadyServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &IPlayReadyServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: IPlayReadyServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPlayReadyServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPlayReadyServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyServiceRequest {}
impl ::core::fmt::Debug for IPlayReadyServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8bad2836-a703-45a6-a180-76f3565aa725}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPlayReadyServiceRequest {
    type Vtable = IPlayReadyServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bad2836_a703_45a6_a180_76f3565aa725);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BeginServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NextServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GenerateManualEnablingChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessManualEnablingResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySoapMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetMessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MessageHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MessageHeaders: usize,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics {
    type Vtable = IPlayReadyStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e69c00d_247c_469a_8f31_5c1a1571d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DomainJoinServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainLeaveServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IndividualizationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub LicenseAcquirerServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub MeteringReportServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RevocationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub MediaProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PlayReadySecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics2 {
    type Vtable = IPlayReadyStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f8d6a92_5f9a_423e_9466_b33969af7a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics3 {
    type Vtable = IPlayReadyStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SecureStopServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CheckSupportedHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics4 {
    type Vtable = IPlayReadyStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50a91300_d824_4231_9d5e_78ef8844c7d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics5 {
    type Vtable = IPlayReadyStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDCertificateFeature(pub i32);
#[cfg(feature = "winrt-")]
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDCertificateFeature {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDCertificateFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDCertificateFeature {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDCertificateFeature {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDCertificateFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateFeature").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDCertificatePlatformID(pub i32);
#[cfg(feature = "winrt-")]
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDCertificatePlatformID {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDCertificatePlatformID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDCertificatePlatformID {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDCertificatePlatformID {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDCertificatePlatformID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificatePlatformID").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDCertificateType(pub i32);
#[cfg(feature = "winrt-")]
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDCertificateType {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDCertificateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDCertificateType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDCertificateType {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDCertificateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateType").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDCertificateType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDClient(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDClient {
    #[cfg(feature = "winrt-")]
    pub fn RegistrationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveRegistrationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRegistrationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ProximityDetectionCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ProximityDetectionCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveProximityDetectionCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProximityDetectionCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn LicenseFetchCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveLicenseFetchCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLicenseFetchCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ReRegistrationNeeded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<NDClient, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReRegistrationNeeded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveReRegistrationNeeded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReRegistrationNeeded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ClosedCaptionDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionDataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveClosedCaptionDataReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosedCaptionDataReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn StartAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, INDCustomData>, Param3: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, contenturl: Param0, startasyncoptions: u32, registrationcustomdata: Param2, licensefetchdescriptor: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDStartResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), contenturl.into_param().abi(), startasyncoptions, registrationcustomdata.into_param().abi(), licensefetchdescriptor.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDStartResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LicenseFetchAsync<'a, Param0: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDLicenseFetchResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchAsync)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDLicenseFetchResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ReRegistrationAsync<'a, Param0: ::windows_core::IntoParam<'a, INDCustomData>>(&self, registrationcustomdata: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReRegistrationAsync)(::windows_core::Interface::as_raw(this), registrationcustomdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, INDDownloadEngine>, Param1: ::windows_core::IntoParam<'a, INDStreamParser>, Param2: ::windows_core::IntoParam<'a, INDMessenger>>(downloadengine: Param0, streamparser: Param1, pmessenger: Param2) -> ::windows_core::Result<NDClient> {
        Self::INDClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), downloadengine.into_param().abi(), streamparser.into_param().abi(), pmessenger.into_param().abi(), result__.as_mut_ptr()).from_abi::<NDClient>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDClient, INDClientFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDClient {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDClient").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDClient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDClient;{3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDClient {
    type Vtable = INDClient_Vtbl;
    const IID: ::windows_core::GUID = <INDClient as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDClient> for ::windows_core::IUnknown {
    fn from(value: NDClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDClient> for ::windows_core::IUnknown {
    fn from(value: &NDClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDClient> for ::windows_core::IInspectable {
    fn from(value: NDClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDClient> for ::windows_core::IInspectable {
    fn from(value: &NDClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDClosedCaptionFormat(pub i32);
#[cfg(feature = "winrt-")]
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDClosedCaptionFormat {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDClosedCaptionFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDClosedCaptionFormat {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDClosedCaptionFormat {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDClosedCaptionFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDClosedCaptionFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDContentIDType(pub i32);
#[cfg(feature = "winrt-")]
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDContentIDType {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDContentIDType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDContentIDType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDContentIDType {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDContentIDType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDContentIDType").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDContentIDType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDCustomData(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDCustomData {
    #[cfg(feature = "winrt-")]
    pub fn CustomDataTypeID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomDataTypeID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CustomData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateInstance(customdatatypeidbytes: &[u8], customdatabytes: &[u8]) -> ::windows_core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), customdatatypeidbytes.len() as u32, ::core::mem::transmute(customdatatypeidbytes.as_ptr()), customdatabytes.len() as u32, ::core::mem::transmute(customdatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<NDCustomData>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDCustomData, INDCustomDataFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDCustomData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDCustomData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDCustomData {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDCustomData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCustomData").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDCustomData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDCustomData;{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDCustomData {
    type Vtable = INDCustomData_Vtbl;
    const IID: ::windows_core::GUID = <INDCustomData as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDCustomData> for ::windows_core::IUnknown {
    fn from(value: NDCustomData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDCustomData> for ::windows_core::IUnknown {
    fn from(value: &NDCustomData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDCustomData> for ::windows_core::IInspectable {
    fn from(value: NDCustomData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDCustomData> for ::windows_core::IInspectable {
    fn from(value: &NDCustomData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDCustomData> for INDCustomData {
    type Error = ::windows_core::Error;
    fn try_from(value: NDCustomData) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDCustomData> for INDCustomData {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDCustomData) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDCustomData> for NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, INDCustomData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDCustomData> for &NDCustomData {
    fn into_param(self) -> ::windows_core::Param<'a, INDCustomData> {
        ::core::convert::TryInto::<INDCustomData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDDownloadEngineNotifier {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDDownloadEngineNotifier, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnStreamOpened(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnStreamOpened)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(::windows_core::Interface::as_raw(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnContentIDReceived<'a, Param0: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnDataReceived)(::windows_core::Interface::as_raw(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnEndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnEndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnNetworkError(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnNetworkError)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDDownloadEngineNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDDownloadEngineNotifier {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDDownloadEngineNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDDownloadEngineNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier;{d720b4d4-f4b8-4530-a809-9193a571e7fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
    const IID: ::windows_core::GUID = <INDDownloadEngineNotifier as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDDownloadEngineNotifier> for ::windows_core::IUnknown {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDDownloadEngineNotifier> for ::windows_core::IUnknown {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDDownloadEngineNotifier> for ::windows_core::IInspectable {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDDownloadEngineNotifier> for ::windows_core::IInspectable {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    type Error = ::windows_core::Error;
    fn try_from(value: NDDownloadEngineNotifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDDownloadEngineNotifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDDownloadEngineNotifier> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, INDDownloadEngineNotifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDDownloadEngineNotifier> for &NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, INDDownloadEngineNotifier> {
        ::core::convert::TryInto::<INDDownloadEngineNotifier>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDLicenseFetchDescriptor {
    #[cfg(feature = "winrt-")]
    pub fn ContentIDType(&self) -> ::windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NDContentIDType>::zeroed();
            (::windows_core::Interface::vtable(this).ContentIDType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ContentID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateInstance<'a, Param2: ::windows_core::IntoParam<'a, INDCustomData>>(contentidtype: NDContentIDType, contentidbytes: &[u8], licensefetchchallengecustomdata: Param2) -> ::windows_core::Result<NDLicenseFetchDescriptor> {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), contentidtype, contentidbytes.len() as u32, ::core::mem::transmute(contentidbytes.as_ptr()), licensefetchchallengecustomdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<NDLicenseFetchDescriptor>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDLicenseFetchDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDLicenseFetchDescriptor {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDLicenseFetchDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDLicenseFetchDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor;{5498d33a-e686-4935-a567-7ca77ad20fa4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <INDLicenseFetchDescriptor as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDLicenseFetchDescriptor> for ::windows_core::IUnknown {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDLicenseFetchDescriptor> for ::windows_core::IUnknown {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDLicenseFetchDescriptor> for ::windows_core::IInspectable {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDLicenseFetchDescriptor> for ::windows_core::IInspectable {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: NDLicenseFetchDescriptor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDLicenseFetchDescriptor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, INDLicenseFetchDescriptor> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor> for &NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows_core::Param<'a, INDLicenseFetchDescriptor> {
        ::core::convert::TryInto::<INDLicenseFetchDescriptor>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDMediaStreamType(pub i32);
#[cfg(feature = "winrt-")]
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDMediaStreamType {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDMediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDMediaStreamType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDMediaStreamType {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDMediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDMediaStreamType").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDProximityDetectionType(pub i32);
#[cfg(feature = "winrt-")]
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDProximityDetectionType {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDProximityDetectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDProximityDetectionType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDProximityDetectionType {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDProximityDetectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDProximityDetectionType").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NDStartAsyncOptions(pub i32);
#[cfg(feature = "winrt-")]
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for NDStartAsyncOptions {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDStartAsyncOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for NDStartAsyncOptions {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for NDStartAsyncOptions {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDStartAsyncOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStartAsyncOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDStorageFileHelper(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDStorageFileHelper {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDStorageFileHelper, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage", feature = "winrt-"))]
    pub fn GetFileURLs<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileURLs)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDStorageFileHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDStorageFileHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDStorageFileHelper {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDStorageFileHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStorageFileHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStorageFileHelper;{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
    const IID: ::windows_core::GUID = <INDStorageFileHelper as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDStorageFileHelper> for ::windows_core::IUnknown {
    fn from(value: NDStorageFileHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDStorageFileHelper> for ::windows_core::IUnknown {
    fn from(value: &NDStorageFileHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDStorageFileHelper> for ::windows_core::IInspectable {
    fn from(value: NDStorageFileHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDStorageFileHelper> for ::windows_core::IInspectable {
    fn from(value: &NDStorageFileHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDStorageFileHelper> for INDStorageFileHelper {
    type Error = ::windows_core::Error;
    fn try_from(value: NDStorageFileHelper) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDStorageFileHelper> for INDStorageFileHelper {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDStorageFileHelper) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDStorageFileHelper> for NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, INDStorageFileHelper> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDStorageFileHelper> for &NDStorageFileHelper {
    fn into_param(self) -> ::windows_core::Param<'a, INDStorageFileHelper> {
        ::core::convert::TryInto::<INDStorageFileHelper>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDStreamParserNotifier(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDStreamParserNotifier {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDStreamParserNotifier, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn OnContentIDReceived<'a, Param0: ::windows_core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-media", feature = "winrt-"))]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(::windows_core::Interface::as_raw(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn OnSampleParsed<'a, Param2: ::windows_core::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnSampleParsed)(::windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::core::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows_core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::core::mem::transmute(probytes.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDStreamParserNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDStreamParserNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDStreamParserNotifier {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDStreamParserNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStreamParserNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStreamParserNotifier;{c167acd0-2ce6-426c-ace5-5e9275fea715})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
    const IID: ::windows_core::GUID = <INDStreamParserNotifier as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDStreamParserNotifier> for ::windows_core::IUnknown {
    fn from(value: NDStreamParserNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDStreamParserNotifier> for ::windows_core::IUnknown {
    fn from(value: &NDStreamParserNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDStreamParserNotifier> for ::windows_core::IInspectable {
    fn from(value: NDStreamParserNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDStreamParserNotifier> for ::windows_core::IInspectable {
    fn from(value: &NDStreamParserNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDStreamParserNotifier> for INDStreamParserNotifier {
    type Error = ::windows_core::Error;
    fn try_from(value: NDStreamParserNotifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDStreamParserNotifier> for INDStreamParserNotifier {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDStreamParserNotifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDStreamParserNotifier> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, INDStreamParserNotifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDStreamParserNotifier> for &NDStreamParserNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, INDStreamParserNotifier> {
        ::core::convert::TryInto::<INDStreamParserNotifier>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct NDTCPMessenger(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl NDTCPMessenger {
    #[cfg(feature = "winrt-")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), responsedatabytes.len() as u32, ::core::mem::transmute(responsedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(remotehostname: Param0, remotehostport: u32) -> ::windows_core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), remotehostname.into_param().abi(), remotehostport, result__.as_mut_ptr()).from_abi::<NDTCPMessenger>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for NDTCPMessenger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for NDTCPMessenger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for NDTCPMessenger {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for NDTCPMessenger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDTCPMessenger").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDTCPMessenger;{d42df95d-a75b-47bf-8249-bc83820da38a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for NDTCPMessenger {
    type Vtable = INDMessenger_Vtbl;
    const IID: ::windows_core::GUID = <INDMessenger as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDTCPMessenger> for ::windows_core::IUnknown {
    fn from(value: NDTCPMessenger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDTCPMessenger> for ::windows_core::IUnknown {
    fn from(value: &NDTCPMessenger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<NDTCPMessenger> for ::windows_core::IInspectable {
    fn from(value: NDTCPMessenger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&NDTCPMessenger> for ::windows_core::IInspectable {
    fn from(value: &NDTCPMessenger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<NDTCPMessenger> for INDMessenger {
    type Error = ::windows_core::Error;
    fn try_from(value: NDTCPMessenger) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&NDTCPMessenger> for INDMessenger {
    type Error = ::windows_core::Error;
    fn try_from(value: &NDTCPMessenger) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDMessenger> for NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, INDMessenger> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, INDMessenger> for &NDTCPMessenger {
    fn into_param(self) -> ::windows_core::Param<'a, INDMessenger> {
        ::core::convert::TryInto::<INDMessenger>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyContentHeader(::windows_core::IUnknown);
impl PlayReadyContentHeader {
    pub fn KeyId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).KeyId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn KeyIdString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).KeyIdString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LicenseAcquisitionUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquisitionUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquisitionUserInterfaceUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn EncryptionType(&self) -> ::windows_core::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlayReadyEncryptionAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).EncryptionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadyEncryptionAlgorithm>(result__)
        }
    }
    pub fn CustomAttributes(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DecryptorSetup(&self) -> ::windows_core::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlayReadyDecryptorSetup>::zeroed();
            (::windows_core::Interface::vtable(this).DecryptorSetup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadyDecryptorSetup>(result__)
        }
    }
    pub fn GetSerializedHeader(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GetSerializedHeader)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn HeaderWithEmbeddedUpdates(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderWithEmbeddedUpdates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn KeyIds(&self) -> ::windows_core::Result<::windows_core::Array<::windows_core::GUID>> {
        let this = &::windows_core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<::windows_core::GUID>>::zeroed();
            (::windows_core::Interface::vtable(this).KeyIds)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<::windows_core::GUID>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn KeyIdStrings(&self) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).KeyIdStrings)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CreateInstanceFromWindowsMediaDrmHeader<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(headerbytes: &[u8], licenseacquisitionurl: Param1, licenseacquisitionuserinterfaceurl: Param2, customattributes: Param3, domainserviceid: Param4) -> ::windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromWindowsMediaDrmHeader)(::windows_core::Interface::as_raw(this), headerbytes.len() as u32, ::core::mem::transmute(headerbytes.as_ptr()), licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn CreateInstanceFromComponents<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param5: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param6: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(contentkeyid: Param0, contentkeyidstring: Param1, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: Param3, licenseacquisitionuserinterfaceurl: Param4, customattributes: Param5, domainserviceid: Param6) -> ::windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromComponents)(::windows_core::Interface::as_raw(this), contentkeyid.into_param().abi(), contentkeyidstring.into_param().abi(), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[u8]) -> ::windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromPlayReadyHeader)(::windows_core::Interface::as_raw(this), headerbytes.len() as u32, ::core::mem::transmute(headerbytes.as_ptr()), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn CreateInstanceFromComponents2<'a, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param6: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param7: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(dwflags: u32, contentkeyids: &[::windows_core::GUID], contentkeyidstrings: &[::windows_core::HSTRING], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: Param4, licenseacquisitionuserinterfaceurl: Param5, customattributes: Param6, domainserviceid: Param7) -> ::windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromComponents2)(::windows_core::Interface::as_raw(this), dwflags, contentkeyids.len() as u32, ::core::mem::transmute(contentkeyids.as_ptr()), contentkeyidstrings.len() as u32, ::core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlayReadyContentHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyContentHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyContentHeader {}
impl ::core::fmt::Debug for PlayReadyContentHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyContentHeader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyContentHeader;{9a438a6a-7f4c-452e-88bd-0148c6387a2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyContentHeader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
impl ::core::convert::From<PlayReadyContentHeader> for ::windows_core::IUnknown {
    fn from(value: PlayReadyContentHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyContentHeader> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyContentHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyContentHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyContentHeader> for ::windows_core::IInspectable {
    fn from(value: PlayReadyContentHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyContentHeader> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyContentHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyContentHeader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct PlayReadyContentResolver;
impl PlayReadyContentResolver {
    pub fn ServiceRequest<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceRequest)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        })
    }
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyDecryptorSetup {}
impl ::core::clone::Clone for PlayReadyDecryptorSetup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyDecryptorSetup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlayReadyDecryptorSetup {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyDecryptorSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDecryptorSetup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PlayReadyDomain(::windows_core::IUnknown);
impl PlayReadyDomain {
    pub fn AccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DomainJoinUrl(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomain {}
impl ::core::fmt::Debug for PlayReadyDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomain").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyDomain {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomain;{adcc93ac-97e6-43ef-95e4-d7868f3b16a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyDomain as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
impl ::core::convert::From<PlayReadyDomain> for ::windows_core::IUnknown {
    fn from(value: PlayReadyDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomain> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyDomain> for ::windows_core::IInspectable {
    fn from(value: PlayReadyDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomain> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyDomain> for IPlayReadyDomain {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomain) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomain> for IPlayReadyDomain {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomain) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyDomain> for PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyDomain> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyDomain> for &PlayReadyDomain {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyDomain> {
        ::core::convert::TryInto::<IPlayReadyDomain>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadyDomainIterable(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadyDomainIterable {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IPlayReadyDomain>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(domainaccountid: Param0) -> ::windows_core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), domainaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlayReadyDomainIterable>(result__)
        })
    }
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadyDomainIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadyDomainIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadyDomainIterable {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadyDomainIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadyDomainIterable {
    type Vtable = ::winrt_foundation::Collections::IIterable_Vtbl<IPlayReadyDomain>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterable<IPlayReadyDomain> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyDomainIterable> for ::windows_core::IUnknown {
    fn from(value: PlayReadyDomainIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyDomainIterable> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyDomainIterable> for ::windows_core::IInspectable {
    fn from(value: PlayReadyDomainIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyDomainIterable> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadyDomainIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadyDomain> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainIterable) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadyDomain> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainIterable) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyDomain>> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyDomain>> for &PlayReadyDomainIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IPlayReadyDomain>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadyDomainIterator(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadyDomainIterator {
    #[cfg(feature = "winrt-foundation")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyDomain>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyDomain>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadyDomainIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadyDomainIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadyDomainIterator {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadyDomainIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadyDomainIterator {
    type Vtable = ::winrt_foundation::Collections::IIterator_Vtbl<IPlayReadyDomain>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterator<IPlayReadyDomain> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyDomainIterator> for ::windows_core::IUnknown {
    fn from(value: PlayReadyDomainIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyDomainIterator> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyDomainIterator> for ::windows_core::IInspectable {
    fn from(value: PlayReadyDomainIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyDomainIterator> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadyDomainIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadyDomain> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainIterator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadyDomain> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainIterator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyDomain>> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyDomain>> for &PlayReadyDomainIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterator<IPlayReadyDomain>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainJoinServiceRequest(::windows_core::IUnknown);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyDomainJoinServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainAccountId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainAccountId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainFriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DomainFriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDomainFriendlyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainFriendlyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomainJoinServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomainJoinServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomainJoinServiceRequest {}
impl ::core::fmt::Debug for PlayReadyDomainJoinServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainJoinServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest;{171b4a5a-405f-4739-b040-67b9f0c38758})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyDomainJoinServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
impl ::core::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyDomainLeaveServiceRequest(::windows_core::IUnknown);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyDomainLeaveServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainAccountId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainAccountId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomainLeaveServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomainLeaveServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomainLeaveServiceRequest {}
impl ::core::fmt::Debug for PlayReadyDomainLeaveServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainLeaveServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest;{062d58be-97ad-4917-aa03-46d4c252d464})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyDomainLeaveServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
impl ::core::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for PlayReadyEncryptionAlgorithm {}
impl ::core::clone::Clone for PlayReadyEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyEncryptionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlayReadyEncryptionAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyEncryptionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyEncryptionAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl ::core::marker::Copy for PlayReadyHardwareDRMFeatures {}
impl ::core::clone::Clone for PlayReadyHardwareDRMFeatures {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyHardwareDRMFeatures {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlayReadyHardwareDRMFeatures {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyHardwareDRMFeatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyHardwareDRMFeatures").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyITADataFormat {}
impl ::core::clone::Clone for PlayReadyITADataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyITADataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlayReadyITADataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyITADataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyITADataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PlayReadyITADataGenerator(::windows_core::IUnknown);
impl PlayReadyITADataGenerator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyITADataGenerator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GenerateData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, guidcpsystemid: Param0, countofstreams: u32, configuration: Param2, format: PlayReadyITADataFormat) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateData)(::windows_core::Interface::as_raw(this), guidcpsystemid.into_param().abi(), countofstreams, configuration.into_param().abi(), format, ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for PlayReadyITADataGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyITADataGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyITADataGenerator {}
impl ::core::fmt::Debug for PlayReadyITADataGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyITADataGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator;{24446b8e-10b9-4530-b25b-901a8029a9b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyITADataGenerator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
impl ::core::convert::From<PlayReadyITADataGenerator> for ::windows_core::IUnknown {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyITADataGenerator> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyITADataGenerator> for ::windows_core::IInspectable {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyITADataGenerator> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PlayReadyIndividualizationServiceRequest(::windows_core::IUnknown);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyIndividualizationServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyIndividualizationServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyIndividualizationServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyIndividualizationServiceRequest {}
impl ::core::fmt::Debug for PlayReadyIndividualizationServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyIndividualizationServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest;{21f5a86b-008c-4611-ab2f-aaa6c69f0e24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyIndividualizationServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
impl ::core::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyLicense(::windows_core::IUnknown);
impl PlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).FullyEvaluated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UsableForPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountID)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ChainDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GetKIDAtChainDepth)(::windows_core::Interface::as_raw(this), chaindepth, result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SecureStopId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SecureStopId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SecurityLevel(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SecurityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn InMemoryOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InMemoryOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExpiresInRealTime(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ExpiresInRealTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicense {}
impl ::core::fmt::Debug for PlayReadyLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicense;{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyLicense as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
impl ::core::convert::From<PlayReadyLicense> for ::windows_core::IUnknown {
    fn from(value: PlayReadyLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicense> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyLicense> for ::windows_core::IInspectable {
    fn from(value: PlayReadyLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicense> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyLicense> for IPlayReadyLicense {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicense) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicense> for IPlayReadyLicense {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicense) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicense> for PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicense> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicense> for &PlayReadyLicense {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicense> {
        ::core::convert::TryInto::<IPlayReadyLicense>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(::windows_core::IUnknown);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ContentHeader(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentHeader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn SetContentHeader<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentHeader)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, result__.as_mut_ptr()).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicenseAcquisitionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::fmt::Debug for PlayReadyLicenseAcquisitionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseAcquisitionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest;{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyLicenseAcquisitionServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
impl ::core::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyLicenseAcquisitionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadyLicenseIterable(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadyLicenseIterable {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyLicenseIterable, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IPlayReadyLicense>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable> {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, result__.as_mut_ptr()).from_abi::<PlayReadyLicenseIterable>(result__)
        })
    }
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadyLicenseIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadyLicenseIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadyLicenseIterable {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadyLicenseIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadyLicenseIterable {
    type Vtable = ::winrt_foundation::Collections::IIterable_Vtbl<IPlayReadyLicense>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterable<IPlayReadyLicense> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyLicenseIterable> for ::windows_core::IUnknown {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyLicenseIterable> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyLicenseIterable> for ::windows_core::IInspectable {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyLicenseIterable> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadyLicenseIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadyLicense> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseIterable) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadyLicense> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseIterable) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyLicense>> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyLicense>> for &PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IPlayReadyLicense>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadyLicenseIterator(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadyLicenseIterator {
    #[cfg(feature = "winrt-foundation")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyLicense>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyLicense>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadyLicenseIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadyLicenseIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadyLicenseIterator {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadyLicenseIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadyLicenseIterator {
    type Vtable = ::winrt_foundation::Collections::IIterator_Vtbl<IPlayReadyLicense>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterator<IPlayReadyLicense> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyLicenseIterator> for ::windows_core::IUnknown {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyLicenseIterator> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadyLicenseIterator> for ::windows_core::IInspectable {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadyLicenseIterator> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadyLicenseIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadyLicense> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseIterator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadyLicense> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseIterator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyLicense>> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyLicense>> for &PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterator<IPlayReadyLicense>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
pub struct PlayReadyLicenseManagement;
impl PlayReadyLicenseManagement {
    pub fn DeleteLicenses<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteLicenses)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[repr(transparent)]
pub struct PlayReadyLicenseSession(::windows_core::IUnknown);
impl PlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows_core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows_core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable> {
        let this = &::windows_core::Interface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, result__.as_mut_ptr()).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(configuration: Param0) -> ::windows_core::Result<PlayReadyLicenseSession> {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), configuration.into_param().abi(), result__.as_mut_ptr()).from_abi::<PlayReadyLicenseSession>(result__)
        })
    }
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlayReadyLicenseSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicenseSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicenseSession {}
impl ::core::fmt::Debug for PlayReadyLicenseSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseSession;{a1723a39-87fa-4fdd-abbb-a9720e845259})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyLicenseSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
impl ::core::convert::From<PlayReadyLicenseSession> for ::windows_core::IUnknown {
    fn from(value: PlayReadyLicenseSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseSession> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyLicenseSession> for ::windows_core::IInspectable {
    fn from(value: PlayReadyLicenseSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseSession> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession> {
        ::core::convert::TryInto::<IPlayReadyLicenseSession>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession2> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyLicenseSession2> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyLicenseSession2> {
        ::core::convert::TryInto::<IPlayReadyLicenseSession2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyMeteringReportServiceRequest(::windows_core::IUnknown);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyMeteringReportServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn MeteringCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).MeteringCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMeteringCertificate)(::windows_core::Interface::as_raw(this), meteringcertbytes.len() as u32, ::core::mem::transmute(meteringcertbytes.as_ptr())).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyMeteringReportServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyMeteringReportServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyMeteringReportServiceRequest {}
impl ::core::fmt::Debug for PlayReadyMeteringReportServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyMeteringReportServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest;{c12b231c-0ecd-4f11-a185-1e24a4a67fb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyMeteringReportServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
impl ::core::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadyRevocationServiceRequest(::windows_core::IUnknown);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyRevocationServiceRequest, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyRevocationServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyRevocationServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyRevocationServiceRequest {}
impl ::core::fmt::Debug for PlayReadyRevocationServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyRevocationServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest;{543d66ac-faf0-4560-84a5-0e4acec939e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadyRevocationServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
impl ::core::convert::From<PlayReadyRevocationServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyRevocationServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadyRevocationServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyRevocationServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadySecureStopIterable(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadySecureStopIterable {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), result__.as_mut_ptr()).from_abi::<PlayReadySecureStopIterable>(result__)
        })
    }
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadySecureStopIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadySecureStopIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadySecureStopIterable {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadySecureStopIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadySecureStopIterable {
    type Vtable = ::winrt_foundation::Collections::IIterable_Vtbl<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadySecureStopIterable> for ::windows_core::IUnknown {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadySecureStopIterable> for ::windows_core::IUnknown {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadySecureStopIterable> for ::windows_core::IInspectable {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadySecureStopIterable> for ::windows_core::IInspectable {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadySecureStopIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadySecureStopIterable) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterable> for ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadySecureStopIterable) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PlayReadySecureStopIterator(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PlayReadySecureStopIterator {
    #[cfg(feature = "winrt-foundation")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadySecureStopServiceRequest>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadySecureStopServiceRequest>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PlayReadySecureStopIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PlayReadySecureStopIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PlayReadySecureStopIterator {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PlayReadySecureStopIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PlayReadySecureStopIterator {
    type Vtable = ::winrt_foundation::Collections::IIterator_Vtbl<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadySecureStopIterator> for ::windows_core::IUnknown {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadySecureStopIterator> for ::windows_core::IUnknown {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PlayReadySecureStopIterator> for ::windows_core::IInspectable {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PlayReadySecureStopIterator> for ::windows_core::IInspectable {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PlayReadySecureStopIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadySecureStopIterator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterator> for ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadySecureStopIterator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadySecureStopServiceRequest(::windows_core::IUnknown);
impl PlayReadySecureStopServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SessionID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SessionID)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn UpdateTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).PublisherCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), result__.as_mut_ptr()).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    pub fn CreateInstanceFromSessionID<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(sessionid: Param0, publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromSessionID)(::windows_core::Interface::as_raw(this), sessionid.into_param().abi(), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), result__.as_mut_ptr()).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadySecureStopServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadySecureStopServiceRequest {}
impl ::core::fmt::Debug for PlayReadySecureStopServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest;{b5501ee5-01bf-4401-9677-05630a6a4cc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadySecureStopServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
impl ::core::convert::From<PlayReadySecureStopServiceRequest> for ::windows_core::IUnknown {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadySecureStopServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadySecureStopServiceRequest> for ::windows_core::IInspectable {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadySecureStopServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadySecureStopServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadySecureStopServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::core::convert::TryInto::<IPlayReadySecureStopServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows_core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct PlayReadySoapMessage(::windows_core::IUnknown);
impl PlayReadySoapMessage {
    pub fn GetMessageBody(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageBody)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MessageHeaders(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageHeaders)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadySoapMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadySoapMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadySoapMessage {}
impl ::core::fmt::Debug for PlayReadySoapMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySoapMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySoapMessage;{b659fcb5-ce41-41ba-8a0d-61df5fffa139})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
    const IID: ::windows_core::GUID = <IPlayReadySoapMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
impl ::core::convert::From<PlayReadySoapMessage> for ::windows_core::IUnknown {
    fn from(value: PlayReadySoapMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadySoapMessage> for ::windows_core::IUnknown {
    fn from(value: &PlayReadySoapMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PlayReadySoapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayReadySoapMessage> for ::windows_core::IInspectable {
    fn from(value: PlayReadySoapMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadySoapMessage> for ::windows_core::IInspectable {
    fn from(value: &PlayReadySoapMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PlayReadySoapMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct PlayReadyStatics;
impl PlayReadyStatics {
    pub fn DomainJoinServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn DomainLeaveServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).DomainLeaveServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn IndividualizationServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).IndividualizationServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn LicenseAcquirerServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquirerServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn MeteringReportServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).MeteringReportServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn RevocationServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).RevocationServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn MediaProtectionSystemId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).MediaProtectionSystemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn PlayReadySecurityVersion() -> ::windows_core::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PlayReadySecurityVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn PlayReadyCertificateSecurityLevel() -> ::windows_core::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PlayReadyCertificateSecurityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn SecureStopServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SecureStopServiceRequestType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows_core::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CheckSupportedHardware)(::windows_core::Interface::as_raw(this), hwdrmfeature, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn InputTrustAuthorityToCreate() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InputTrustAuthorityToCreate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ProtectionSystemId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn HardwareDRMDisabledAtTime() -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareDRMDisabledAtTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        })
    }
    pub fn HardwareDRMDisabledUntilTime() -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareDRMDisabledUntilTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        })
    }
    pub fn ResetHardwareDRMDisabled() -> ::windows_core::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).ResetHardwareDRMDisabled)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
