#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceDiscoveryMechanism(pub i32);
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
impl ::core::marker::Copy for DeviceDiscoveryMechanism {}
impl ::core::clone::Clone for DeviceDiscoveryMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceDiscoveryMechanism {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceDiscoveryMechanism {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceDiscoveryMechanism {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDiscoveryMechanism").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IWSDAddress(::windows_core::IUnknown);
impl IWSDAddress {
    pub unsafe fn Serialize<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszbuffer: &mut [u16], fsafe: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszbuffer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDAddress> for ::windows_core::IUnknown {
    fn from(value: IWSDAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAddress> for ::windows_core::IUnknown {
    fn from(value: &IWSDAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAddress {}
impl ::core::fmt::Debug for IWSDAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDAddress {
    type Vtable = IWSDAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9574c6c_12a6_4f74_93a1_3318ff605759);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDAsyncCallback(::windows_core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncResult>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pasyncresult: Param0, pasyncstate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncOperationComplete)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDAsyncCallback> for ::windows_core::IUnknown {
    fn from(value: IWSDAsyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAsyncCallback> for ::windows_core::IUnknown {
    fn from(value: &IWSDAsyncCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDAsyncCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDAsyncCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDAsyncCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncCallback {}
impl ::core::fmt::Debug for IWSDAsyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDAsyncCallback {
    type Vtable = IWSDAsyncCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: ::windows_core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDAsyncResult(::windows_core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncCallback>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pcallback: Param0, pasyncstate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCallback)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi(), pasyncstate.into_param().abi()).ok()
    }
    pub unsafe fn SetWaitHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, hwaithandle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWaitHandle)(::windows_core::Interface::as_raw(self), hwaithandle.into_param().abi()).ok()
    }
    pub unsafe fn HasCompleted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HasCompleted)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAsyncState(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetAsyncState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetEvent(&self) -> ::windows_core::Result<WSD_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<WSD_EVENT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSD_EVENT>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDAsyncResult> for ::windows_core::IUnknown {
    fn from(value: IWSDAsyncResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAsyncResult> for ::windows_core::IUnknown {
    fn from(value: &IWSDAsyncResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDAsyncResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDAsyncResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncResult {}
impl ::core::fmt::Debug for IWSDAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDAsyncResult {
    type Vtable = IWSDAsyncResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWaitHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwaithandle: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub HasCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpoint: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDAttachment(::windows_core::IUnknown);
impl IWSDAttachment {}
impl ::core::convert::From<IWSDAttachment> for ::windows_core::IUnknown {
    fn from(value: IWSDAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDAttachment> for ::windows_core::IUnknown {
    fn from(value: &IWSDAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAttachment {}
impl ::core::fmt::Debug for IWSDAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDAttachment {
    type Vtable = IWSDAttachment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d55a616_9df8_4b09_b156_9ba351a48b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IWSDDeviceHost(::windows_core::IUnknown);
impl IWSDDeviceHost {
    pub unsafe fn Init<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDXMLContext>>(&self, pszlocalid: Param0, pcontext: Param1, pphostaddresses: &[::core::option::Option<IWSDAddress>]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pphostaddresses)), pphostaddresses.len() as _).ok()
    }
    pub unsafe fn Start<'a, Param2: ::windows_core::IntoParam<'a, IWSDDeviceHostNotify>>(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(pscopelist), pnotificationsink.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterPortType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pporttype)).ok()
    }
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pthismodelmetadata), ::core::mem::transmute(pthisdevicemetadata), ::core::mem::transmute(phostmetadata), ::core::mem::transmute(pcustommetadata)).ok()
    }
    pub unsafe fn RegisterService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pszserviceid: Param0, pservice: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), pservice.into_param().abi()).ok()
    }
    pub unsafe fn RetireService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszserviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RetireService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi()).ok()
    }
    pub unsafe fn AddDynamicService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pszserviceid: Param0, pszendpointaddress: Param1, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDynamicService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), pszendpointaddress.into_param().abi(), ::core::mem::transmute(pporttype), ::core::mem::transmute(pportname), ::core::mem::transmute(pany), pservice.into_param().abi()).ok()
    }
    pub unsafe fn RemoveDynamicService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszserviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveDynamicService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi()).ok()
    }
    pub unsafe fn SetServiceDiscoverable<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszserviceid: Param0, fdiscoverable: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceDiscoverable)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), fdiscoverable.into_param().abi()).ok()
    }
    pub unsafe fn SignalEvent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszserviceid: Param0, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SignalEvent)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
}
impl ::core::convert::From<IWSDDeviceHost> for ::windows_core::IUnknown {
    fn from(value: IWSDDeviceHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceHost> for ::windows_core::IUnknown {
    fn from(value: &IWSDDeviceHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDDeviceHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDDeviceHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDDeviceHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHost {}
impl ::core::fmt::Debug for IWSDDeviceHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDDeviceHost {
    type Vtable = IWSDDeviceHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x917fe891_3d13_4138_9809_934c8abeb12c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, pphostaddresses: *const ::windows_core::RawPtr, dwhostaddresscount: u32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterPortType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub RegisterService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RetireService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AddDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pszendpointaddress: ::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveDynamicService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetServiceDiscoverable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, fdiscoverable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SignalEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDDeviceHostNotify(::windows_core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszserviceid: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetService)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IWSDDeviceHostNotify> for ::windows_core::IUnknown {
    fn from(value: IWSDDeviceHostNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceHostNotify> for ::windows_core::IUnknown {
    fn from(value: &IWSDDeviceHostNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDDeviceHostNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDDeviceHostNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHostNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHostNotify {}
impl ::core::fmt::Debug for IWSDDeviceHostNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHostNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDDeviceHostNotify {
    type Vtable = IWSDDeviceHostNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDDeviceProxy(::windows_core::IUnknown);
impl IWSDDeviceProxy {
    pub unsafe fn Init<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDAddress>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWSDXMLContext>, Param4: ::windows_core::IntoParam<'a, IWSDDeviceProxy>>(&self, pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3, psponsor: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), psponsor.into_param().abi()).ok()
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi()).ok()
    }
    pub unsafe fn GetHostMetadata(&self) -> ::windows_core::Result<*mut WSD_HOST_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_HOST_METADATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetHostMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_HOST_METADATA>(result__)
    }
    pub unsafe fn GetThisModelMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_MODEL_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_THIS_MODEL_METADATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetThisModelMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_THIS_MODEL_METADATA>(result__)
    }
    pub unsafe fn GetThisDeviceMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_DEVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_THIS_DEVICE_METADATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetThisDeviceMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_THIS_DEVICE_METADATA>(result__)
    }
    pub unsafe fn GetAllMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceProxyById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszserviceid: Param0) -> ::windows_core::Result<IWSDServiceProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceProxyById)(::windows_core::Interface::as_raw(self), pszserviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDServiceProxy>(result__)
    }
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows_core::Result<IWSDServiceProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceProxyByType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDServiceProxy>(result__)
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDDeviceProxy> for ::windows_core::IUnknown {
    fn from(value: IWSDDeviceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDDeviceProxy> for ::windows_core::IUnknown {
    fn from(value: &IWSDDeviceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDDeviceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDDeviceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDDeviceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceProxy {}
impl ::core::fmt::Debug for IWSDDeviceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDDeviceProxy {
    type Vtable = IWSDDeviceProxy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeee0c031_c578_4c0e_9a3b_973c35f409db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows_core::PCWSTR, pdeviceaddress: ::windows_core::RawPtr, pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, psponsor: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetHostMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows_core::HRESULT,
    pub GetThisModelMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows_core::HRESULT,
    pub GetThisDeviceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows_core::HRESULT,
    pub GetAllMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub GetServiceProxyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppserviceproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetServiceProxyByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDEndpointProxy(::windows_core::IUnknown);
impl IWSDEndpointProxy {
    pub unsafe fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOneWayRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendTwoWayRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), ::core::mem::transmute(presponsecontext)).ok()
    }
    pub unsafe fn SendTwoWayRequestAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, IWSDAsyncCallback>>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: Param2, pcallback: Param3) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SendTwoWayRequestAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pasyncstate.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn AbortAsyncOperation<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, pasyncresult: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortAsyncOperation)(::windows_core::Interface::as_raw(self), pasyncresult.into_param().abi()).ok()
    }
    pub unsafe fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessFault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfault)).ok()
    }
    pub unsafe fn GetErrorInfo(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetFaultInfo(&self) -> ::windows_core::Result<*mut WSD_SOAP_FAULT> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_SOAP_FAULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetFaultInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
}
impl ::core::convert::From<IWSDEndpointProxy> for ::windows_core::IUnknown {
    fn from(value: IWSDEndpointProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDEndpointProxy> for ::windows_core::IUnknown {
    fn from(value: &IWSDEndpointProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDEndpointProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDEndpointProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDEndpointProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDEndpointProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEndpointProxy {}
impl ::core::fmt::Debug for IWSDEndpointProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEndpointProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDEndpointProxy {
    type Vtable = IWSDEndpointProxy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1860d430_b24c_4975_9f90_dbb39baa24ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SendOneWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
    pub SendTwoWayRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::HRESULT,
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, presult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AbortAsyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetFaultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDEventingStatus(::windows_core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubscriptionaction: Param0) {
        (::windows_core::Interface::vtable(self).SubscriptionRenewed)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi())
    }
    pub unsafe fn SubscriptionRenewalFailed<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubscriptionaction: Param0, hr: ::windows_core::HRESULT) {
        (::windows_core::Interface::vtable(self).SubscriptionRenewalFailed)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi(), ::core::mem::transmute(hr))
    }
    pub unsafe fn SubscriptionEnded<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubscriptionaction: Param0) {
        (::windows_core::Interface::vtable(self).SubscriptionEnded)(::windows_core::Interface::as_raw(self), pszsubscriptionaction.into_param().abi())
    }
}
impl ::core::convert::From<IWSDEventingStatus> for ::windows_core::IUnknown {
    fn from(value: IWSDEventingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDEventingStatus> for ::windows_core::IUnknown {
    fn from(value: &IWSDEventingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDEventingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDEventingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDEventingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDEventingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEventingStatus {}
impl ::core::fmt::Debug for IWSDEventingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEventingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDEventingStatus {
    type Vtable = IWSDEventingStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR, hr: ::windows_core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR),
}
#[repr(transparent)]
pub struct IWSDHttpAddress(::windows_core::IUnknown);
impl IWSDHttpAddress {
    pub unsafe fn Serialize<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszbuffer: &mut [u16], fsafe: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszbuffer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wport)).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsafe: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
    pub unsafe fn GetSecure(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSecure)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetSecure<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsecure: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecure)(::windows_core::Interface::as_raw(self), fsecure.into_param().abi()).ok()
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDHttpAddress> for ::windows_core::IUnknown {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for ::windows_core::IUnknown {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for &'a IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: IWSDHttpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDHttpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDTransportAddress> for IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDTransportAddress> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDTransportAddress> for &'a IWSDHttpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDTransportAddress> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDHttpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAddress {}
impl ::core::fmt::Debug for IWSDHttpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDHttpAddress {
    type Vtable = IWSDHttpAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsecure: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDHttpAuthParameters(::windows_core::IUnknown);
impl IWSDHttpAuthParameters {
    pub unsafe fn GetClientAccessToken(&self) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).GetClientAccessToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    pub unsafe fn GetAuthType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWSDHttpAuthParameters> for ::windows_core::IUnknown {
    fn from(value: IWSDHttpAuthParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpAuthParameters> for ::windows_core::IUnknown {
    fn from(value: &IWSDHttpAuthParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDHttpAuthParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDHttpAuthParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAuthParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAuthParameters {}
impl ::core::fmt::Debug for IWSDHttpAuthParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAuthParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDHttpAuthParameters {
    type Vtable = IWSDHttpAuthParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b476df0_8dac_480d_b05c_99781a5884aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClientAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetAuthType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDHttpMessageParameters(::windows_core::IUnknown);
impl IWSDHttpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLocalAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRemoteAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLowerParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
    pub unsafe fn SetInboundHttpHeaders<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszheaders: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInboundHttpHeaders)(::windows_core::Interface::as_raw(self), pszheaders.into_param().abi()).ok()
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetInboundHttpHeaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetOutboundHttpHeaders<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszheaders: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutboundHttpHeaders)(::windows_core::Interface::as_raw(self), pszheaders.into_param().abi()).ok()
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutboundHttpHeaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetID<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetID)(::windows_core::Interface::as_raw(self), pszid.into_param().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pcontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContext)(::windows_core::Interface::as_raw(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDHttpMessageParameters> for ::windows_core::IUnknown {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for ::windows_core::IUnknown {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDHttpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDHttpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDHttpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMessageParameters> for IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMessageParameters> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMessageParameters> for &'a IWSDHttpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMessageParameters> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDHttpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDHttpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpMessageParameters {}
impl ::core::fmt::Debug for IWSDHttpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDHttpMessageParameters {
    type Vtable = IWSDHttpMessageParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x540bd122_5c83_4dec_b396_ea62a2697fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDInboundAttachment(::windows_core::IUnknown);
impl IWSDInboundAttachment {
    pub unsafe fn Read(&self, pbuffer: &mut [u8], pdwnumberofbytesread: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(pdwnumberofbytesread)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDInboundAttachment> for ::windows_core::IUnknown {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for ::windows_core::IUnknown {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDInboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: IWSDInboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDInboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDInboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAttachment> for IWSDInboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAttachment> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAttachment> for &'a IWSDInboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAttachment> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDInboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDInboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDInboundAttachment {}
impl ::core::fmt::Debug for IWSDInboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDInboundAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDInboundAttachment {
    type Vtable = IWSDInboundAttachment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDMessageParameters(::windows_core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetLowerParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
}
impl ::core::convert::From<IWSDMessageParameters> for ::windows_core::IUnknown {
    fn from(value: IWSDMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDMessageParameters> for ::windows_core::IUnknown {
    fn from(value: &IWSDMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMessageParameters {}
impl ::core::fmt::Debug for IWSDMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDMessageParameters {
    type Vtable = IWSDMessageParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxparams: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDMetadataExchange(::windows_core::IUnknown);
impl IWSDMetadataExchange {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
}
impl ::core::convert::From<IWSDMetadataExchange> for ::windows_core::IUnknown {
    fn from(value: IWSDMetadataExchange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDMetadataExchange> for ::windows_core::IUnknown {
    fn from(value: &IWSDMetadataExchange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDMetadataExchange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDMetadataExchange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDMetadataExchange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDMetadataExchange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMetadataExchange {}
impl ::core::fmt::Debug for IWSDMetadataExchange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMetadataExchange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDMetadataExchange {
    type Vtable = IWSDMetadataExchange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06996d57_1d67_4928_9307_3d7833fdb846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDOutboundAttachment(::windows_core::IUnknown);
impl IWSDOutboundAttachment {
    pub unsafe fn Write(&self, pbuffer: &[u8]) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWSDOutboundAttachment> for ::windows_core::IUnknown {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for ::windows_core::IUnknown {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDOutboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: IWSDOutboundAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDOutboundAttachment> for IWSDAttachment {
    fn from(value: &IWSDOutboundAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAttachment> for IWSDOutboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAttachment> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAttachment> for &'a IWSDOutboundAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAttachment> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDOutboundAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDOutboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDOutboundAttachment {}
impl ::core::fmt::Debug for IWSDOutboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDOutboundAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDOutboundAttachment {
    type Vtable = IWSDOutboundAttachment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDSSLClientCertificate(::windows_core::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn GetClientCertificate(&self) -> ::windows_core::Result<*mut ::win32_security::Cryptography::CERT_CONTEXT> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_security::Cryptography::CERT_CONTEXT>::zeroed();
        (::windows_core::Interface::vtable(self).GetClientCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_security::Cryptography::CERT_CONTEXT>(result__)
    }
    pub unsafe fn GetMappedAccessToken(&self) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).GetMappedAccessToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<IWSDSSLClientCertificate> for ::windows_core::IUnknown {
    fn from(value: IWSDSSLClientCertificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDSSLClientCertificate> for ::windows_core::IUnknown {
    fn from(value: &IWSDSSLClientCertificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDSSLClientCertificate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDSSLClientCertificate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDSSLClientCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSSLClientCertificate {}
impl ::core::fmt::Debug for IWSDSSLClientCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSSLClientCertificate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDSSLClientCertificate {
    type Vtable = IWSDSSLClientCertificate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde105e87_a0da_418e_98ad_27b9eed87bdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut ::win32_security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    GetClientCertificate: usize,
    pub GetMappedAccessToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phtoken: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDScopeMatchingRule(::windows_core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetScopeRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn MatchScopes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszscope1: Param0, pszscope2: Param1) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).MatchScopes)(::windows_core::Interface::as_raw(self), pszscope1.into_param().abi(), pszscope2.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWSDScopeMatchingRule> for ::windows_core::IUnknown {
    fn from(value: IWSDScopeMatchingRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDScopeMatchingRule> for ::windows_core::IUnknown {
    fn from(value: &IWSDScopeMatchingRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDScopeMatchingRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDScopeMatchingRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDScopeMatchingRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDScopeMatchingRule {}
impl ::core::fmt::Debug for IWSDScopeMatchingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDScopeMatchingRule").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDScopeMatchingRule {
    type Vtable = IWSDScopeMatchingRule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcafe424_fef5_481a_bd9f_33ce0574256f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetScopeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub MatchScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszscope1: ::windows_core::PCWSTR, pszscope2: ::windows_core::PCWSTR, pfmatch: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDServiceMessaging(::windows_core::IUnknown);
impl IWSDServiceMessaging {
    pub unsafe fn SendResponse<'a, Param2: ::windows_core::IntoParam<'a, IWSDMessageParameters>>(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendResponse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbody), ::core::mem::transmute(poperation), pmessageparameters.into_param().abi()).ok()
    }
    pub unsafe fn FaultRequest<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>>(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: Param1, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FaultRequest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequestheader), pmessageparameters.into_param().abi(), ::core::mem::transmute(pfault)).ok()
    }
}
impl ::core::convert::From<IWSDServiceMessaging> for ::windows_core::IUnknown {
    fn from(value: IWSDServiceMessaging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceMessaging> for ::windows_core::IUnknown {
    fn from(value: &IWSDServiceMessaging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDServiceMessaging {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDServiceMessaging {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDServiceMessaging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceMessaging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceMessaging {}
impl ::core::fmt::Debug for IWSDServiceMessaging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceMessaging").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDServiceMessaging {
    type Vtable = IWSDServiceMessaging_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SendResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FaultRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows_core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDServiceProxy(::windows_core::IUnknown);
impl IWSDServiceProxy {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_SERVICE_METADATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetServiceMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<*mut WSDXML_ELEMENT> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSDXML_ELEMENT>::zeroed();
        (::windows_core::Interface::vtable(self).SubscribeToOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperation), punknown.into_param().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnsubscribeToOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows_core::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventingStatusCallback)(::windows_core::Interface::as_raw(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
}
impl ::core::convert::From<IWSDServiceProxy> for ::windows_core::IUnknown {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for ::windows_core::IUnknown {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDServiceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDServiceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxy> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMetadataExchange> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMetadataExchange> for &'a IWSDServiceProxy {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMetadataExchange> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDServiceProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxy {}
impl ::core::fmt::Debug for IWSDServiceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDServiceProxy {
    type Vtable = IWSDServiceProxy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: ::windows_core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT,
    pub GetServiceMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows_core::HRESULT,
    pub SubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub UnsubscribeToOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT,
    pub SetEventingStatusCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDServiceProxyEventing(::windows_core::IUnknown);
impl IWSDServiceProxyEventing {
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginGetMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetMetadata<'a, Param0: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, presult: Param0) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_METADATA_SECTION_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndGetMetadata)(::windows_core::Interface::as_raw(self), presult.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_METADATA_SECTION_LIST>(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_SERVICE_METADATA>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetServiceMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SERVICE_METADATA>(result__)
    }
    pub unsafe fn SubscribeToOperation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, poperation: *const WSD_OPERATION, punknown: Param1, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<*mut WSDXML_ELEMENT> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSDXML_ELEMENT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SubscribeToOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperation), punknown.into_param().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_ELEMENT>(result__)
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnsubscribeToOperation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(poperation)).ok()
    }
    pub unsafe fn SetEventingStatusCallback<'a, Param0: ::windows_core::IntoParam<'a, IWSDEventingStatus>>(&self, pstatus: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEventingStatusCallback)(::windows_core::Interface::as_raw(self), pstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEndpointProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDEndpointProxy>(result__)
    }
    pub unsafe fn SubscribeToMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, poperations: &[WSD_OPERATION], punknown: Param2, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, punknown.into_param().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn BeginSubscribeToMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param5: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param6: ::windows_core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: &[WSD_OPERATION], punknown: Param2, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Param5, pasynccallback: Param6) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginSubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, punknown.into_param().abi(), ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndSubscribeToMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: &[WSD_OPERATION], presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany)).ok()
    }
    pub unsafe fn BeginUnsubscribeToMultipleOperations<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginUnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndUnsubscribeToMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: &[WSD_OPERATION], presult: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndUnsubscribeToMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into_param().abi()).ok()
    }
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[WSD_OPERATION], pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn BeginRenewMultipleOperations<'a, Param4: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param5: ::windows_core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: &[WSD_OPERATION], pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Param4, pasynccallback: Param5) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginRenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pexpires), ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndRenewMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: &[WSD_OPERATION], presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndRenewMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
    pub unsafe fn BeginGetStatusForMultipleOperations<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, IWSDAsyncCallback>>(&self, poperations: &[WSD_OPERATION], pany: *const WSDXML_ELEMENT, pasyncstate: Param3, pasynccallback: Param4) -> ::windows_core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginGetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, ::core::mem::transmute(pany), pasyncstate.into_param().abi(), pasynccallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAsyncResult>(result__)
    }
    pub unsafe fn EndGetStatusForMultipleOperations<'a, Param2: ::windows_core::IntoParam<'a, IWSDAsyncResult>>(&self, poperations: &[WSD_OPERATION], presult: Param2, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndGetStatusForMultipleOperations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(poperations)), poperations.len() as _, presult.into_param().abi(), ::core::mem::transmute(ppexpires), ::core::mem::transmute(ppany)).ok()
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for ::windows_core::IUnknown {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for ::windows_core::IUnknown {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDMetadataExchange {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMetadataExchange> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMetadataExchange> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMetadataExchange> for &'a IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMetadataExchange> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: IWSDServiceProxyEventing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDServiceProxyEventing> for IWSDServiceProxy {
    fn from(value: &IWSDServiceProxyEventing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDServiceProxy> for IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDServiceProxy> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDServiceProxy> for &'a IWSDServiceProxyEventing {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDServiceProxy> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDServiceProxyEventing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxyEventing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxyEventing {}
impl ::core::fmt::Debug for IWSDServiceProxyEventing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxyEventing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDServiceProxyEventing {
    type Vtable = IWSDServiceProxyEventing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows_core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndRenewMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows_core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows_core::RawPtr, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows_core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDSignatureProperty(::windows_core::IUnknown);
impl IWSDSignatureProperty {
    pub unsafe fn IsMessageSigned(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsMessageSigned)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn IsMessageSignatureTrusted(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsMessageSignatureTrusted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetKeyInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbkeyinfo), ::core::mem::transmute(pdwkeyinfosize)).ok()
    }
    pub unsafe fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsignature), ::core::mem::transmute(pdwsignaturesize)).ok()
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignedInfoHash)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbsignedinfohash), ::core::mem::transmute(pdwhashsize)).ok()
    }
}
impl ::core::convert::From<IWSDSignatureProperty> for ::windows_core::IUnknown {
    fn from(value: IWSDSignatureProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDSignatureProperty> for ::windows_core::IUnknown {
    fn from(value: &IWSDSignatureProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDSignatureProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDSignatureProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDSignatureProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDSignatureProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSignatureProperty {}
impl ::core::fmt::Debug for IWSDSignatureProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSignatureProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDSignatureProperty {
    type Vtable = IWSDSignatureProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsMessageSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsigned: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetKeyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDTransportAddress(::windows_core::IUnknown);
impl IWSDTransportAddress {
    pub unsafe fn Serialize<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszbuffer: &mut [u16], fsafe: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszbuffer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wport)).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransportAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsafe: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDTransportAddress> for ::windows_core::IUnknown {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for ::windows_core::IUnknown {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDTransportAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDTransportAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDTransportAddress> for IWSDAddress {
    fn from(value: IWSDTransportAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDTransportAddress> for IWSDAddress {
    fn from(value: &IWSDTransportAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for IWSDTransportAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for &'a IWSDTransportAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDTransportAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDTransportAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDTransportAddress {}
impl ::core::fmt::Debug for IWSDTransportAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDTransportAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDTransportAddress {
    type Vtable = IWSDTransportAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70d23498_4ee6_4340_a3df_d845d2235467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows_core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTransportAddressEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsafe: ::win32_foundation::BOOL, ppszaddress: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDUdpAddress(::windows_core::IUnknown);
impl IWSDUdpAddress {
    pub unsafe fn Serialize<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszbuffer: &mut [u16], fsafe: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Serialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuffer)), pszbuffer.len() as _, fsafe.into_param().abi()).ok()
    }
    pub unsafe fn Deserialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszbuffer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Deserialize)(::windows_core::Interface::as_raw(self), pszbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wport)).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTransportAddressEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsafe: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportAddressEx)(::windows_core::Interface::as_raw(self), fsafe.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn SetTransportAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszaddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransportAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const ::win32_networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSockaddr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psockaddr)).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn GetSockaddr(&self) -> ::windows_core::Result<::win32_networking::WinSock::SOCKADDR_STORAGE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_networking::WinSock::SOCKADDR_STORAGE>::zeroed();
        (::windows_core::Interface::vtable(self).GetSockaddr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_networking::WinSock::SOCKADDR_STORAGE>(result__)
    }
    pub unsafe fn SetExclusive<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fexclusive: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExclusive)(::windows_core::Interface::as_raw(self), fexclusive.into_param().abi()).ok()
    }
    pub unsafe fn GetExclusive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExclusive)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(messagetype)).ok()
    }
    pub unsafe fn GetMessageType(&self) -> ::windows_core::Result<WSDUdpMessageType> {
        let mut result__ = ::core::mem::MaybeUninit::<WSDUdpMessageType>::zeroed();
        (::windows_core::Interface::vtable(self).GetMessageType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSDUdpMessageType>(result__)
    }
    pub unsafe fn SetTTL(&self, dwttl: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTTL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwttl)).ok()
    }
    pub unsafe fn GetTTL(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTTL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAlias(&self, palias: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlias)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(palias)).ok()
    }
    pub unsafe fn GetAlias(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetAlias)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IWSDUdpAddress> for ::windows_core::IUnknown {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for ::windows_core::IUnknown {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDAddress> for &'a IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDAddress> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: IWSDUdpAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpAddress> for IWSDTransportAddress {
    fn from(value: &IWSDUdpAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDTransportAddress> for IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDTransportAddress> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDTransportAddress> for &'a IWSDUdpAddress {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDTransportAddress> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDUdpAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDUdpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpAddress {}
impl ::core::fmt::Debug for IWSDUdpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDUdpAddress {
    type Vtable = IWSDUdpAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74d6124a_a441_4f78_a1eb_97a8d1996893);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub SetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *const ::win32_networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    SetSockaddr: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub GetSockaddr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psockaddr: *mut ::win32_networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    GetSockaddr: usize,
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fexclusive: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows_core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows_core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows_core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows_core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, palias: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDUdpMessageParameters(::windows_core::IUnknown);
impl IWSDUdpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLocalAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetLocalAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRemoteAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDAddress>(result__)
    }
    pub unsafe fn SetRemoteAddress<'a, Param0: ::windows_core::IntoParam<'a, IWSDAddress>>(&self, paddress: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddress)(::windows_core::Interface::as_raw(self), paddress.into_param().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLowerParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDMessageParameters>(result__)
    }
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRetransmitParams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pparams)).ok()
    }
    pub unsafe fn GetRetransmitParams(&self) -> ::windows_core::Result<WSDUdpRetransmitParams> {
        let mut result__ = ::core::mem::MaybeUninit::<WSDUdpRetransmitParams>::zeroed();
        (::windows_core::Interface::vtable(self).GetRetransmitParams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSDUdpRetransmitParams>(result__)
    }
}
impl ::core::convert::From<IWSDUdpMessageParameters> for ::windows_core::IUnknown {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for ::windows_core::IUnknown {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: IWSDUdpMessageParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDUdpMessageParameters> for IWSDMessageParameters {
    fn from(value: &IWSDUdpMessageParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMessageParameters> for IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMessageParameters> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWSDMessageParameters> for &'a IWSDUdpMessageParameters {
    fn into_param(self) -> ::windows_core::Param<'a, IWSDMessageParameters> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDUdpMessageParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDUdpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpMessageParameters {}
impl ::core::fmt::Debug for IWSDUdpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpMessageParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDUdpMessageParameters {
    type Vtable = IWSDUdpMessageParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDXMLContext(::windows_core::IUnknown);
impl IWSDXMLContext {
    pub unsafe fn AddNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszuri: Param0, pszsuggestedprefix: Param1) -> ::windows_core::Result<*mut WSDXML_NAMESPACE> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSDXML_NAMESPACE>::zeroed();
        (::windows_core::Interface::vtable(self).AddNamespace)(::windows_core::Interface::as_raw(self), pszuri.into_param().abi(), pszsuggestedprefix.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAMESPACE>(result__)
    }
    pub unsafe fn AddNameToNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszuri: Param0, pszname: Param1) -> ::windows_core::Result<*mut WSDXML_NAME> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSDXML_NAME>::zeroed();
        (::windows_core::Interface::vtable(self).AddNameToNamespace)(::windows_core::Interface::as_raw(self), pszuri.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAME>(result__)
    }
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[*const WSDXML_NAMESPACE], blayernumber: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pnamespaces)), pnamespaces.len() as _, ::core::mem::transmute(blayernumber)).ok()
    }
    pub unsafe fn SetTypes(&self, ptypes: &[*const WSDXML_TYPE], blayernumber: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(ptypes)), ptypes.len() as _, ::core::mem::transmute(blayernumber)).ok()
    }
}
impl ::core::convert::From<IWSDXMLContext> for ::windows_core::IUnknown {
    fn from(value: IWSDXMLContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDXMLContext> for ::windows_core::IUnknown {
    fn from(value: &IWSDXMLContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDXMLContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDXMLContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDXMLContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDXMLContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDXMLContext {}
impl ::core::fmt::Debug for IWSDXMLContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDXMLContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDXMLContext {
    type Vtable = IWSDXMLContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszsuggestedprefix: ::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::HRESULT,
    pub AddNameToNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT,
    pub SetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::HRESULT,
    pub SetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDiscoveredService(::windows_core::IUnknown);
impl IWSDiscoveredService {
    pub unsafe fn GetEndpointReference(&self) -> ::windows_core::Result<*mut WSD_ENDPOINT_REFERENCE> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_ENDPOINT_REFERENCE>::zeroed();
        (::windows_core::Interface::vtable(self).GetEndpointReference)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_ENDPOINT_REFERENCE>(result__)
    }
    pub unsafe fn GetTypes(&self) -> ::windows_core::Result<*mut WSD_NAME_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_NAME_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_NAME_LIST>(result__)
    }
    pub unsafe fn GetScopes(&self) -> ::windows_core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_URI_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).GetScopes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    pub unsafe fn GetXAddrs(&self) -> ::windows_core::Result<*mut WSD_URI_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_URI_LIST>::zeroed();
        (::windows_core::Interface::vtable(self).GetXAddrs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_URI_LIST>(result__)
    }
    pub unsafe fn GetMetadataVersion(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadataVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExtendedDiscoXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppheaderany), ::core::mem::transmute(ppbodyany)).ok()
    }
    pub unsafe fn GetProbeResolveTag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetProbeResolveTag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteTransportAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalTransportAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalInterfaceGUID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveredService> for ::windows_core::IUnknown {
    fn from(value: IWSDiscoveredService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveredService> for ::windows_core::IUnknown {
    fn from(value: &IWSDiscoveredService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDiscoveredService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDiscoveredService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDiscoveredService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveredService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveredService {}
impl ::core::fmt::Debug for IWSDiscoveredService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveredService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDiscoveredService {
    type Vtable = IWSDiscoveredService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bad8a3b_b374_4420_9632_aac945b374aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetEndpointReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows_core::HRESULT,
    pub GetTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows_core::HRESULT,
    pub GetScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub GetXAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub GetMetadataVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows_core::HRESULT,
    pub GetExtendedDiscoXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub GetProbeResolveTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDiscoveryProvider(::windows_core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaddressfamily)).ok()
    }
    pub unsafe fn Attach<'a, Param0: ::windows_core::IntoParam<'a, IWSDiscoveryProviderNotify>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Attach)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Detach)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SearchById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszid: Param0, psztag: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchById)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchByAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszaddress: Param0, psztag: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchByAddress)(::windows_core::Interface::as_raw(self), pszaddress.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchByType<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: Param2, psztag: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchByType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), pszmatchby.into_param().abi(), psztag.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetXMLContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveryProvider> for ::windows_core::IUnknown {
    fn from(value: IWSDiscoveryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryProvider> for ::windows_core::IUnknown {
    fn from(value: &IWSDiscoveryProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDiscoveryProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDiscoveryProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDiscoveryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProvider {}
impl ::core::fmt::Debug for IWSDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDiscoveryProvider {
    type Vtable = IWSDiscoveryProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SearchById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDiscoveryProviderNotify(::windows_core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, IWSDiscoveredService>>(&self, pservice: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), pservice.into_param().abi()).ok()
    }
    pub unsafe fn SearchFailed<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hr: ::windows_core::HRESULT, psztag: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchFailed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), psztag.into_param().abi()).ok()
    }
    pub unsafe fn SearchComplete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, psztag: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SearchComplete)(::windows_core::Interface::as_raw(self), psztag.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDiscoveryProviderNotify> for ::windows_core::IUnknown {
    fn from(value: IWSDiscoveryProviderNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryProviderNotify> for ::windows_core::IUnknown {
    fn from(value: &IWSDiscoveryProviderNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDiscoveryProviderNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDiscoveryProviderNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProviderNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProviderNotify {}
impl ::core::fmt::Debug for IWSDiscoveryProviderNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProviderNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDiscoveryProviderNotify {
    type Vtable = IWSDiscoveryProviderNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pservice: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDiscoveryPublisher(::windows_core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressFamily)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaddressfamily)).ok()
    }
    pub unsafe fn RegisterNotificationSink<'a, Param0: ::windows_core::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterNotificationSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterNotificationSink<'a, Param0: ::windows_core::IntoParam<'a, IWSDiscoveryPublisherNotify>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegisterNotificationSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn Publish<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszid: Param0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param4, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Publish)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), ::core::mem::transmute(ullmetadataversion), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn UnPublish<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszid: Param0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param3, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnPublish)(::windows_core::Interface::as_raw(self), pszid.into_param().abi(), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::core::mem::transmute(pany)).ok()
    }
    pub unsafe fn MatchProbe<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1, pszid: Param2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param6, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MatchProbe)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pprobemessage), pmessageparameters.into_param().abi(), pszid.into_param().abi(), ::core::mem::transmute(ullmetadataversion), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn MatchResolve<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1, pszid: Param2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param6, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MatchResolve)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(presolvemessage), pmessageparameters.into_param().abi(), pszid.into_param().abi(), ::core::mem::transmute(ullmetadataversion), ::core::mem::transmute(ullinstanceid), ::core::mem::transmute(ullmessagenumber), pszsessionid.into_param().abi(), ::core::mem::transmute(ptypeslist), ::core::mem::transmute(pscopeslist), ::core::mem::transmute(pxaddrslist)).ok()
    }
    pub unsafe fn PublishEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszid: Param0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param4, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PublishEx)(
            ::windows_core::Interface::as_raw(self),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn MatchProbeEx<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1, pszid: Param2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param6, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MatchProbeEx)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(pprobemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn MatchResolveEx<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1, pszid: Param2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: Param6, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MatchResolveEx)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(presolvemessage),
            pmessageparameters.into_param().abi(),
            pszid.into_param().abi(),
            ::core::mem::transmute(ullmetadataversion),
            ::core::mem::transmute(ullinstanceid),
            ::core::mem::transmute(ullmessagenumber),
            pszsessionid.into_param().abi(),
            ::core::mem::transmute(ptypeslist),
            ::core::mem::transmute(pscopeslist),
            ::core::mem::transmute(pxaddrslist),
            ::core::mem::transmute(pheaderany),
            ::core::mem::transmute(preferenceparameterany),
            ::core::mem::transmute(ppolicyany),
            ::core::mem::transmute(pendpointreferenceany),
            ::core::mem::transmute(pany),
        )
        .ok()
    }
    pub unsafe fn RegisterScopeMatchingRule<'a, Param0: ::windows_core::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterScopeMatchingRule)(::windows_core::Interface::as_raw(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterScopeMatchingRule<'a, Param0: ::windows_core::IntoParam<'a, IWSDScopeMatchingRule>>(&self, pscopematchingrule: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegisterScopeMatchingRule)(::windows_core::Interface::as_raw(self), pscopematchingrule.into_param().abi()).ok()
    }
    pub unsafe fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetXMLContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
    }
}
impl ::core::convert::From<IWSDiscoveryPublisher> for ::windows_core::IUnknown {
    fn from(value: IWSDiscoveryPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisher> for ::windows_core::IUnknown {
    fn from(value: &IWSDiscoveryPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDiscoveryPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDiscoveryPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisher {}
impl ::core::fmt::Debug for IWSDiscoveryPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDiscoveryPublisher {
    type Vtable = IWSDiscoveryPublisher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae01e1a8_3ff9_4148_8116_057cc616fe13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Publish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub UnPublish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchProbe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub MatchResolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT,
    pub PublishEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchProbeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub MatchResolveEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWSDiscoveryPublisherNotify(::windows_core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    pub unsafe fn ProbeHandler<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProbeHandler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
    pub unsafe fn ResolveHandler<'a, Param1: ::windows_core::IntoParam<'a, IWSDMessageParameters>>(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResolveHandler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psoap), pmessageparameters.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWSDiscoveryPublisherNotify> for ::windows_core::IUnknown {
    fn from(value: IWSDiscoveryPublisherNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWSDiscoveryPublisherNotify> for ::windows_core::IUnknown {
    fn from(value: &IWSDiscoveryPublisherNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSDiscoveryPublisherNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWSDiscoveryPublisherNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisherNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisherNotify {}
impl ::core::fmt::Debug for IWSDiscoveryPublisherNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisherNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWSDiscoveryPublisherNotify {
    type Vtable = IWSDiscoveryPublisherNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe67651b0_337a_4b3c_9758_733388568251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ProbeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResolveHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub type PWSD_SOAP_MESSAGE_HANDLER = ::core::option::Option<unsafe extern "system" fn(thisunknown: ::core::option::Option<::windows_core::IUnknown>, event: *mut WSD_EVENT) -> ::windows_core::HRESULT>;
#[repr(C)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_GetStatus {}
impl ::core::clone::Clone for REQUESTBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for REQUESTBODY_GetStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_GetStatus>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_GetStatus {}
impl ::core::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Renew {}
impl ::core::clone::Clone for REQUESTBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for REQUESTBODY_Renew {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Renew>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Renew {}
impl ::core::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Subscribe {}
impl ::core::clone::Clone for REQUESTBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for REQUESTBODY_Subscribe {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Subscribe>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Subscribe {}
impl ::core::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Unsubscribe {}
impl ::core::clone::Clone for REQUESTBODY_Unsubscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
unsafe impl ::windows_core::Abi for REQUESTBODY_Unsubscribe {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUESTBODY_Unsubscribe>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Unsubscribe {}
impl ::core::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl ::core::marker::Copy for RESPONSEBODY_GetMetadata {}
impl ::core::clone::Clone for RESPONSEBODY_GetMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
unsafe impl ::windows_core::Abi for RESPONSEBODY_GetMetadata {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_GetMetadata>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetMetadata {}
impl ::core::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_GetStatus {}
impl ::core::clone::Clone for RESPONSEBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
unsafe impl ::windows_core::Abi for RESPONSEBODY_GetStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_GetStatus>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetStatus {}
impl ::core::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_Renew {}
impl ::core::clone::Clone for RESPONSEBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
unsafe impl ::windows_core::Abi for RESPONSEBODY_Renew {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_Renew>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_Renew {}
impl ::core::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_Subscribe {}
impl ::core::clone::Clone for RESPONSEBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
unsafe impl ::windows_core::Abi for RESPONSEBODY_Subscribe {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_Subscribe>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_Subscribe {}
impl ::core::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: ::windows_core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_SubscriptionEnd {}
impl ::core::clone::Clone for RESPONSEBODY_SubscriptionEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for RESPONSEBODY_SubscriptionEnd {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESPONSEBODY_SubscriptionEnd>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
impl ::core::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(WSDAllocateLinkedMemory(::core::mem::transmute(pparent), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
        }
        WSDAttachLinkedMemory(::core::mem::transmute(pparent), ::core::mem::transmute(pchild))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceHost<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1) -> ::windows_core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost(pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, ppdevicehost: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceHost(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceHost2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows_core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHost2(pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceHost2(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszlocalid: Param0, pcontext: Param1, pphostaddresses: &[::core::option::Option<IWSDAddress>]) -> ::windows_core::Result<IWSDDeviceHost> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceHostAdvanced(pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, pphostaddresses: *const ::windows_core::RawPtr, dwhostaddresscount: u32, ppdevicehost: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceHostAdvanced(pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pphostaddresses)), pphostaddresses.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceHost>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceProxy<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2) -> ::windows_core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy(pszdeviceid: ::windows_core::PCWSTR, pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, ppdeviceproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceProxy(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pszlocalid: Param1, pcontext: Param2, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows_core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxy2(pszdeviceid: ::windows_core::PCWSTR, pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceProxy2(pszdeviceid.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWSDAddress>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszdeviceid: Param0, pdeviceaddress: Param1, pszlocalid: Param2, pcontext: Param3) -> ::windows_core::Result<IWSDDeviceProxy> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDeviceProxyAdvanced(pszdeviceid: ::windows_core::PCWSTR, pdeviceaddress: ::windows_core::RawPtr, pszlocalid: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, ppdeviceproxy: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDeviceProxyAdvanced(pszdeviceid.into_param().abi(), pdeviceaddress.into_param().abi(), pszlocalid.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDDeviceProxy>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<'a, Param0: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows_core::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider(pcontext: ::windows_core::RawPtr, ppprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDiscoveryProvider(pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<'a, Param0: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows_core::Result<IWSDiscoveryProvider> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryProvider2(pcontext: ::windows_core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDiscoveryProvider2(pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryProvider>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<'a, Param0: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0) -> ::windows_core::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher(pcontext: ::windows_core::RawPtr, pppublisher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDiscoveryPublisher(pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<'a, Param0: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pcontext: Param0, pconfigparams: &[WSD_CONFIG_PARAM]) -> ::windows_core::Result<IWSDiscoveryPublisher> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateDiscoveryPublisher2(pcontext: ::windows_core::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateDiscoveryPublisher2(pcontext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pconfigparams)), pconfigparams.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDiscoveryPublisher>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> ::windows_core::Result<IWSDHttpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpAddress(ppaddress: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateHttpAddress(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDHttpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> ::windows_core::Result<IWSDHttpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateHttpMessageParameters(pptxparams: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateHttpMessageParameters(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDHttpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> ::windows_core::Result<IWSDOutboundAttachment> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateOutboundAttachment(ppattachment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateOutboundAttachment(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDOutboundAttachment>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> ::windows_core::Result<IWSDUdpAddress> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpAddress(ppaddress: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateUdpAddress(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDUdpAddress>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> ::windows_core::Result<IWSDUdpMessageParameters> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDCreateUdpMessageParameters(pptxparams: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDCreateUdpMessageParameters(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDUdpMessageParameters>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
        }
        WSDDetachLinkedMemory(::core::mem::transmute(pvoid))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSDEventType(pub i32);
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
impl ::core::marker::Copy for WSDEventType {}
impl ::core::clone::Clone for WSDEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSDEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDEventType").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
        }
        WSDFreeLinkedMemory(::core::mem::transmute(pvoid))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDGenerateFault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, IWSDXMLContext>>(pszcode: Param0, pszsubcode: Param1, pszreason: Param2, pszdetail: Param3, pcontext: Param4) -> ::windows_core::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFault(pszcode: ::windows_core::PCWSTR, pszsubcode: ::windows_core::PCWSTR, pszreason: ::windows_core::PCWSTR, pszdetail: ::windows_core::PCWSTR, pcontext: ::windows_core::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_SOAP_FAULT>::zeroed();
        WSDGenerateFault(pszcode.into_param().abi(), pszsubcode.into_param().abi(), pszreason.into_param().abi(), pszdetail.into_param().abi(), pcontext.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDGenerateFaultEx<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: Param3) -> ::windows_core::Result<*mut WSD_SOAP_FAULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: ::windows_core::PCWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSD_SOAP_FAULT>::zeroed();
        WSDGenerateFaultEx(::core::mem::transmute(pcode), ::core::mem::transmute(psubcode), ::core::mem::transmute(preasons), pszdetail.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSD_SOAP_FAULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_core::HRESULT;
        }
        WSDGetConfigurationOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pvoid), ::core::mem::transmute(cboutbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_core::HRESULT;
        }
        WSDSetConfigurationOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pvoid), ::core::mem::transmute(cbinbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSDUdpMessageType(pub i32);
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
impl ::core::marker::Copy for WSDUdpMessageType {}
impl ::core::clone::Clone for WSDUdpMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDUdpMessageType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSDUdpMessageType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDUdpMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDUdpMessageType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl ::core::marker::Copy for WSDUdpRetransmitParams {}
impl ::core::clone::Clone for WSDUdpRetransmitParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDUdpRetransmitParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDUdpRetransmitParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDUdpRetransmitParams {}
impl ::core::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut ::windows_core::PWSTR, cchdestout: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriDecode(source: ::windows_core::PCWSTR, cchsource: u32, destout: *mut ::windows_core::PWSTR, cchdestout: *mut u32) -> ::windows_core::HRESULT;
        }
        WSDUriDecode(::core::mem::transmute(::windows_core::as_ptr_or_null(source)), source.len() as _, ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut ::windows_core::PWSTR, cchdestout: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDUriEncode(source: ::windows_core::PCWSTR, cchsource: u32, destout: *mut ::windows_core::PWSTR, cchdestout: *mut u32) -> ::windows_core::HRESULT;
        }
        WSDUriEncode(::core::mem::transmute(::windows_core::as_ptr_or_null(source)), source.len() as _, ::core::mem::transmute(destout), ::core::mem::transmute(cchdestout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT;
        }
        WSDXMLAddChild(::core::mem::transmute(pparent), ::core::mem::transmute(pchild)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT;
        }
        WSDXMLAddSibling(::core::mem::transmute(pfirst), ::core::mem::transmute(psecond)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLBuildAnyForSingleElement<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pelementname: *mut WSDXML_NAME, psztext: Param1, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: ::windows_core::PCWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT;
        }
        WSDXMLBuildAnyForSingleElement(::core::mem::transmute(pelementname), psztext.into_param().abi(), ::core::mem::transmute(ppany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT;
        }
        WSDXMLCleanupElement(::core::mem::transmute(pany)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLCreateContext() -> ::windows_core::Result<IWSDXMLContext> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLCreateContext(ppcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WSDXMLCreateContext(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWSDXMLContext>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(psznamespace: Param0, pszname: Param1) -> ::windows_core::Result<*mut WSDXML_NAME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut WSDXML_NAME>::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut WSDXML_NAME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WSDXMLGetValueFromAny<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(psznamespace: Param0, pszname: Param1, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WSDXMLGetValueFromAny(psznamespace: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        WSDXMLGetValueFromAny(psznamespace.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(pany), ::core::mem::transmute(ppszvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_ATTRIBUTE {}
impl ::core::clone::Clone for WSDXML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_ATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ATTRIBUTE {}
impl ::core::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl ::core::marker::Copy for WSDXML_ELEMENT {}
impl ::core::clone::Clone for WSDXML_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT {}
impl ::core::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSDXML_ELEMENT_LIST {}
impl ::core::clone::Clone for WSDXML_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_ELEMENT_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_ELEMENT_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT_LIST {}
impl ::core::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_NAME {}
impl ::core::clone::Clone for WSDXML_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NAME {}
impl ::core::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NAMESPACE {
    pub Uri: ::windows_core::PCWSTR,
    pub PreferredPrefix: ::windows_core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl ::core::marker::Copy for WSDXML_NAMESPACE {}
impl ::core::clone::Clone for WSDXML_NAMESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_NAMESPACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NAMESPACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NAMESPACE {}
impl ::core::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl ::core::marker::Copy for WSDXML_NODE {}
impl ::core::clone::Clone for WSDXML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_NODE {}
impl ::core::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSDXML_OP(pub i32);
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
impl ::core::marker::Copy for WSDXML_OP {}
impl ::core::clone::Clone for WSDXML_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSDXML_OP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSDXML_OP {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSDXML_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDXML_OP").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_PREFIX_MAPPING {}
impl ::core::clone::Clone for WSDXML_PREFIX_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_PREFIX_MAPPING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_PREFIX_MAPPING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_PREFIX_MAPPING {}
impl ::core::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_TEXT {}
impl ::core::clone::Clone for WSDXML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_TEXT {}
impl ::core::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSDXML_TYPE {
    pub Uri: ::windows_core::PCWSTR,
    pub Table: *const u8,
}
impl ::core::marker::Copy for WSDXML_TYPE {}
impl ::core::clone::Clone for WSDXML_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
unsafe impl ::windows_core::Abi for WSDXML_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSDXML_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSDXML_TYPE {}
impl ::core::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: ::windows_core::PCWSTR,
    pub MessageNumber: u64,
}
impl ::core::marker::Copy for WSD_APP_SEQUENCE {}
impl ::core::clone::Clone for WSD_APP_SEQUENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_APP_SEQUENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_APP_SEQUENCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_APP_SEQUENCE {}
impl ::core::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_BYE {}
impl ::core::clone::Clone for WSD_BYE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_BYE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_BYE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_BYE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_BYE {}
impl ::core::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut ::core::option::Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_ADDRESSES {}
impl ::core::clone::Clone for WSD_CONFIG_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_CONFIG_ADDRESSES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_CONFIG_ADDRESSES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_ADDRESSES {}
impl ::core::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_PARAM {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_CONFIG_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_CONFIG_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_PARAM {}
impl ::core::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
impl ::core::marker::Copy for WSD_CONFIG_PARAM_TYPE {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSD_CONFIG_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSD_CONFIG_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_CONFIG_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WSD_DATETIME {
    pub isPositive: ::win32_foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: ::win32_foundation::BOOL,
    pub TZIsPositive: ::win32_foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
impl ::core::marker::Copy for WSD_DATETIME {}
impl ::core::clone::Clone for WSD_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DATETIME").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).field("TZIsLocal", &self.TZIsLocal).field("TZIsPositive", &self.TZIsPositive).field("TZHour", &self.TZHour).field("TZMinute", &self.TZMinute).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_DATETIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_DATETIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_DATETIME {}
impl ::core::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: &str = "http://*:5357/";
pub const WSD_DEFAULT_HOSTING_ADDRESS: &str = "http://*:5357/";
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: &str = "https://*:5358/";
#[repr(C)]
pub struct WSD_DURATION {
    pub isPositive: ::win32_foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
impl ::core::marker::Copy for WSD_DURATION {}
impl ::core::clone::Clone for WSD_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_DURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_DURATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_DURATION {}
impl ::core::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: ::windows_core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_ENDPOINT_REFERENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_ENDPOINT_REFERENCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_ENDPOINT_REFERENCE_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_ENDPOINT_REFERENCE_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENT {
    pub Hr: ::windows_core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: ::windows_core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: ::core::option::Option<IWSDMessageParameters>,
}
impl ::core::clone::Clone for WSD_EVENT {
    fn clone(&self) -> Self {
        Self {
            Hr: self.Hr,
            EventType: self.EventType,
            DispatchTag: self.DispatchTag,
            HandlerContext: self.HandlerContext.clone(),
            Soap: self.Soap,
            Operation: self.Operation,
            MessageParameters: self.MessageParameters.clone(),
        }
    }
}
impl ::core::fmt::Debug for WSD_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENT").field("Hr", &self.Hr).field("EventType", &self.EventType).field("DispatchTag", &self.DispatchTag).field("HandlerContext", &self.HandlerContext).field("Soap", &self.Soap).field("Operation", &self.Operation).field("MessageParameters", &self.MessageParameters).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WSD_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Hr == other.Hr && self.EventType == other.EventType && self.DispatchTag == other.DispatchTag && self.HandlerContext == other.HandlerContext && self.Soap == other.Soap && self.Operation == other.Operation && self.MessageParameters == other.MessageParameters
    }
}
impl ::core::cmp::Eq for WSD_EVENT {}
impl ::core::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: ::windows_core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENTING_DELIVERY_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_DELIVERY_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENTING_DELIVERY_MODE_PUSH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_DELIVERY_MODE_PUSH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
impl ::core::marker::Copy for WSD_EVENTING_EXPIRES {}
impl ::core::clone::Clone for WSD_EVENTING_EXPIRES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENTING_EXPIRES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_EXPIRES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_EXPIRES {}
impl ::core::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: ::windows_core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENTING_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER {}
impl ::core::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER_ACTION {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_EVENTING_FILTER_ACTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_EVENTING_FILTER_ACTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
impl ::core::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: ::core::option::Option<::windows_core::IUnknown>,
}
impl ::core::clone::Clone for WSD_HANDLER_CONTEXT {
    fn clone(&self) -> Self {
        Self { Handler: self.Handler, PVoid: self.PVoid, Unknown: self.Unknown.clone() }
    }
}
impl ::core::fmt::Debug for WSD_HANDLER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HANDLER_CONTEXT").field("Handler", &self.Handler.map(|f| f as usize)).field("PVoid", &self.PVoid).field("Unknown", &self.Unknown).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_HANDLER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WSD_HANDLER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Handler.map(|f| f as usize) == other.Handler.map(|f| f as usize) && self.PVoid == other.PVoid && self.Unknown == other.Unknown
    }
}
impl ::core::cmp::Eq for WSD_HANDLER_CONTEXT {}
impl ::core::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_HEADER_RELATESTO {}
impl ::core::clone::Clone for WSD_HEADER_RELATESTO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_HEADER_RELATESTO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HEADER_RELATESTO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HEADER_RELATESTO {}
impl ::core::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_HELLO {}
impl ::core::clone::Clone for WSD_HELLO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HELLO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_HELLO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HELLO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HELLO {}
impl ::core::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl ::core::marker::Copy for WSD_HOST_METADATA {}
impl ::core::clone::Clone for WSD_HOST_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_HOST_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_HOST_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_HOST_METADATA {}
impl ::core::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_LOCALIZED_STRING {
    pub lang: ::windows_core::PCWSTR,
    pub String: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_LOCALIZED_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_LOCALIZED_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING {}
impl ::core::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING_LIST {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_LOCALIZED_STRING_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_LOCALIZED_STRING_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
impl ::core::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_METADATA_SECTION {
    pub Dialect: ::windows_core::PCWSTR,
    pub Identifier: ::windows_core::PCWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION {}
impl ::core::clone::Clone for WSD_METADATA_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_METADATA_SECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_METADATA_SECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION {}
impl ::core::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION_LIST {}
impl ::core::clone::Clone for WSD_METADATA_SECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_METADATA_SECTION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_METADATA_SECTION_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION_LIST {}
impl ::core::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl ::core::marker::Copy for WSD_NAME_LIST {}
impl ::core::clone::Clone for WSD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_NAME_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_NAME_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_NAME_LIST {}
impl ::core::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl ::core::marker::Copy for WSD_OPERATION {}
impl ::core::clone::Clone for WSD_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_OPERATION").field("RequestType", &self.RequestType).field("ResponseType", &self.ResponseType).field("RequestStubFunction", &self.RequestStubFunction.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_OPERATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_OPERATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_OPERATION {}
impl ::core::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl ::core::marker::Copy for WSD_PORT_TYPE {}
impl ::core::clone::Clone for WSD_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_PORT_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PORT_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PORT_TYPE {}
impl ::core::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE {}
impl ::core::clone::Clone for WSD_PROBE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_PROBE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE {}
impl ::core::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH {}
impl ::core::clone::Clone for WSD_PROBE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_PROBE_MATCH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH {}
impl ::core::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCHES {}
impl ::core::clone::Clone for WSD_PROBE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_PROBE_MATCHES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCHES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCHES {}
impl ::core::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH_LIST {}
impl ::core::clone::Clone for WSD_PROBE_MATCH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_PROBE_MATCH_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_PROBE_MATCH_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH_LIST {}
impl ::core::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
impl ::core::marker::Copy for WSD_PROTOCOL_TYPE {}
impl ::core::clone::Clone for WSD_PROTOCOL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSD_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSD_PROTOCOL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSD_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PARAMETERS {}
impl ::core::clone::Clone for WSD_REFERENCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_REFERENCE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_REFERENCE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
impl ::core::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PROPERTIES {}
impl ::core::clone::Clone for WSD_REFERENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_REFERENCE_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_REFERENCE_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
impl ::core::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: ::windows_core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RELATIONSHIP_METADATA {}
impl ::core::clone::Clone for WSD_RELATIONSHIP_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_RELATIONSHIP_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RELATIONSHIP_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
impl ::core::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE {}
impl ::core::clone::Clone for WSD_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_RESOLVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE {}
impl ::core::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCH {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_RESOLVE_MATCH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE_MATCH>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCH {}
impl ::core::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCHES {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_RESOLVE_MATCHES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_RESOLVE_MATCHES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCHES {}
impl ::core::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SCOPES {
    pub MatchBy: ::windows_core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_SCOPES {}
impl ::core::clone::Clone for WSD_SCOPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SCOPES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SCOPES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SCOPES {}
impl ::core::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut ::win32_security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: ::win32_security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: ::win32_security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: ::windows_core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).field("pszCNGHashAlgId", &self.pszCNGHashAlgId).field("pbCertHash", &self.pbCertHash).field("dwCertHashSize", &self.dwCertHashSize).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows_core::Abi for WSD_SECURITY_CERT_VALIDATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_CERT_VALIDATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut ::win32_security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: ::win32_security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: ::win32_security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows_core::Abi for WSD_SECURITY_CERT_VALIDATION_V1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_CERT_VALIDATION_V1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut ::win32_security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: ::win32_security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows_core::Abi for WSD_SECURITY_SIGNATURE_VALIDATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SECURITY_SIGNATURE_VALIDATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SERVICE_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SERVICE_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA {}
impl ::core::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA_LIST {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SERVICE_METADATA_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SERVICE_METADATA_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
impl ::core::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: ::windows_core::PCWSTR,
    pub Role: ::windows_core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT {}
impl ::core::clone::Clone for WSD_SOAP_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_FAULT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT {}
impl ::core::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_CODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_FAULT_CODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_CODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_CODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_REASON {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_FAULT_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_REASON {}
impl ::core::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_SUBCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_FAULT_SUBCODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_FAULT_SUBCODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_HEADER {
    pub To: ::windows_core::PCWSTR,
    pub Action: ::windows_core::PCWSTR,
    pub MessageID: ::windows_core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_HEADER {}
impl ::core::clone::Clone for WSD_SOAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_HEADER").field("To", &self.To).field("Action", &self.Action).field("MessageID", &self.MessageID).field("RelatesTo", &self.RelatesTo).field("ReplyTo", &self.ReplyTo).field("From", &self.From).field("FaultTo", &self.FaultTo).field("AppSequence", &self.AppSequence).field("AnyHeaders", &self.AnyHeaders).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_HEADER {}
impl ::core::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl ::core::marker::Copy for WSD_SOAP_MESSAGE {}
impl ::core::clone::Clone for WSD_SOAP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SOAP_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_SOAP_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_SOAP_MESSAGE {}
impl ::core::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WSD_STUB_FUNCTION = ::core::option::Option<unsafe extern "system" fn(server: ::core::option::Option<::windows_core::IUnknown>, session: ::core::option::Option<IWSDServiceMessaging>, event: *mut WSD_EVENT) -> ::windows_core::HRESULT>;
#[repr(C)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows_core::HRESULT,
    pub eventHandle: ::win32_foundation::HANDLE,
    pub messageParameters: ::core::option::Option<IWSDMessageParameters>,
    pub results: *mut ::core::ffi::c_void,
}
impl ::core::clone::Clone for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        Self { hr: self.hr, eventHandle: self.eventHandle, messageParameters: self.messageParameters.clone(), results: self.results }
    }
}
impl ::core::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
impl ::core::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
impl ::core::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: ::windows_core::PCWSTR,
    pub SerialNumber: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_DEVICE_METADATA {}
impl ::core::clone::Clone for WSD_THIS_DEVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_THIS_DEVICE_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_THIS_DEVICE_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
impl ::core::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: ::windows_core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: ::windows_core::PCWSTR,
    pub ModelUrl: ::windows_core::PCWSTR,
    pub PresentationUrl: ::windows_core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_MODEL_METADATA {}
impl ::core::clone::Clone for WSD_THIS_MODEL_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_MODEL_METADATA").field("Manufacturer", &self.Manufacturer).field("ManufacturerUrl", &self.ManufacturerUrl).field("ModelName", &self.ModelName).field("ModelNumber", &self.ModelNumber).field("ModelUrl", &self.ModelUrl).field("PresentationUrl", &self.PresentationUrl).field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_THIS_MODEL_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_THIS_MODEL_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_THIS_MODEL_METADATA {}
impl ::core::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_UNKNOWN_LOOKUP {}
impl ::core::clone::Clone for WSD_UNKNOWN_LOOKUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_UNKNOWN_LOOKUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_UNKNOWN_LOOKUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
impl ::core::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WSD_URI_LIST {}
impl ::core::clone::Clone for WSD_URI_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows_core::Abi for WSD_URI_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WSD_URI_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WSD_URI_LIST {}
impl ::core::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
