#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DWorkflow {
    type Vtable = IPrint3DWorkflow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc56f74bd_3669_4a66_ab42_c8151930cd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetPrintModelPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsPrintReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPrintReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrintRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflow2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DWorkflow2 {
    type Vtable = IPrint3DWorkflow2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2a6c54f_8ac1_4918_9741_e34f3004239e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflow2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrinterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrinterChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrinterChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19f8c858_5ac8_4b55_8a5f_e61567dafb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Print3DWorkflowStatus) -> ::windows_core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Print3DWorkflowDetail) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45226402_95fc_4847_93b3_134dbf5c60f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintExtensionContextStatic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintExtensionContextStatic {
    type Vtable = IPrintExtensionContextStatic_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe70d9fc1_ff79_4aa4_8c9b_0c93aedfde8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintExtensionContextStatic_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintNotificationEventDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe00e4c8a_4828_4da1_8bb8_8672df8515e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNotificationEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EventData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEventData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskConfiguration {
    type Vtable = IPrintTaskConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3c22451_3aa4_4885_9240_311f5f8fbe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrinterExtensionContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSaveRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSaveRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeaf2fcb_621e_4b62_ac77_b281cce08d60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerextensioncontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe959d568_f729_44a4_871d_bd0628696a33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe06c2879_0d61_4938_91d0_96a45bee8479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflow(::windows_core::IUnknown);
impl Print3DWorkflow {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn DeviceID(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceID)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn GetPrintModelPackage(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetPrintModelPackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn IsPrintReady(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrintReady)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn SetIsPrintReady(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPrintReady)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrintRequested)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrintRequested)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>>>(&self, eventhandler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrinterChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPrint3DWorkflow2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrinterChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for Print3DWorkflow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflow {}
impl ::core::fmt::Debug for Print3DWorkflow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflow;{c56f74bd-3669-4a66-ab42-c8151930cd34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Print3DWorkflow {
    type Vtable = IPrint3DWorkflow_Vtbl;
    const IID: ::windows_core::GUID = <IPrint3DWorkflow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Print3DWorkflow {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflow";
}
impl ::core::convert::From<Print3DWorkflow> for ::windows_core::IUnknown {
    fn from(value: Print3DWorkflow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflow> for ::windows_core::IUnknown {
    fn from(value: &Print3DWorkflow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Print3DWorkflow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Print3DWorkflow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflow> for ::windows_core::IInspectable {
    fn from(value: Print3DWorkflow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflow> for ::windows_core::IInspectable {
    fn from(value: &Print3DWorkflow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Print3DWorkflow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Print3DWorkflow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflow {}
unsafe impl ::core::marker::Sync for Print3DWorkflow {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Print3DWorkflowDetail(pub i32);
impl Print3DWorkflowDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl ::core::marker::Copy for Print3DWorkflowDetail {}
impl ::core::clone::Clone for Print3DWorkflowDetail {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Print3DWorkflowDetail {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Print3DWorkflowDetail {
    type Abi = Self;
}
impl ::core::fmt::Debug for Print3DWorkflowDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflowDetail {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowDetail;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowPrintRequestedEventArgs(::windows_core::IUnknown);
impl Print3DWorkflowPrintRequestedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn Status(&self) -> ::windows_core::Result<Print3DWorkflowStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Print3DWorkflowStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Print3DWorkflowStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendedStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn SetSourceChanged(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceChanged)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for Print3DWorkflowPrintRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowPrintRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowPrintRequestedEventArgs {}
impl ::core::fmt::Debug for Print3DWorkflowPrintRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowPrintRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflowPrintRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs;{19f8c858-5ac8-4b55-8a5f-e61567dafb4d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Print3DWorkflowPrintRequestedEventArgs {
    type Vtable = IPrint3DWorkflowPrintRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrint3DWorkflowPrintRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Print3DWorkflowPrintRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowPrintRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: Print3DWorkflowPrintRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrintRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &Print3DWorkflowPrintRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowPrintRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: Print3DWorkflowPrintRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrintRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &Print3DWorkflowPrintRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Print3DWorkflowPrintRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowPrintRequestedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrintRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowPrinterChangedEventArgs(::windows_core::IUnknown);
impl Print3DWorkflowPrinterChangedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn NewDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NewDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for Print3DWorkflowPrinterChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowPrinterChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowPrinterChangedEventArgs {}
impl ::core::fmt::Debug for Print3DWorkflowPrinterChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowPrinterChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflowPrinterChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs;{45226402-95fc-4847-93b3-134dbf5c60f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Print3DWorkflowPrinterChangedEventArgs {
    type Vtable = IPrint3DWorkflowPrinterChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrint3DWorkflowPrinterChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Print3DWorkflowPrinterChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowPrinterChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: Print3DWorkflowPrinterChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrinterChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &Print3DWorkflowPrinterChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowPrinterChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: Print3DWorkflowPrinterChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowPrinterChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &Print3DWorkflowPrinterChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Print3DWorkflowPrinterChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowPrinterChangedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowPrinterChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Print3DWorkflowStatus(pub i32);
impl Print3DWorkflowStatus {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl ::core::marker::Copy for Print3DWorkflowStatus {}
impl ::core::clone::Clone for Print3DWorkflowStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Print3DWorkflowStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Print3DWorkflowStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for Print3DWorkflowStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflowStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.Extensions.Print3DWorkflowStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
pub struct PrintExtensionContext;
impl PrintExtensionContext {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn FromDeviceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IPrintExtensionContextStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).FromDeviceId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintExtensionContextStatic<R, F: FnOnce(&IPrintExtensionContextStatic) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PrintExtensionContext, IPrintExtensionContextStatic> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PrintExtensionContext {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintExtensionContext";
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintNotificationEventDetails(::windows_core::IUnknown);
impl PrintNotificationEventDetails {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn PrinterName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn EventData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EventData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn SetEventData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEventData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PrintNotificationEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintNotificationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintNotificationEventDetails {}
impl ::core::fmt::Debug for PrintNotificationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintNotificationEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintNotificationEventDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintNotificationEventDetails;{e00e4c8a-4828-4da1-8bb8-8672df8515e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintNotificationEventDetails {
    type Vtable = IPrintNotificationEventDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPrintNotificationEventDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintNotificationEventDetails {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
}
impl ::core::convert::From<PrintNotificationEventDetails> for ::windows_core::IUnknown {
    fn from(value: PrintNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintNotificationEventDetails> for ::windows_core::IUnknown {
    fn from(value: &PrintNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintNotificationEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintNotificationEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintNotificationEventDetails> for ::windows_core::IInspectable {
    fn from(value: PrintNotificationEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintNotificationEventDetails> for ::windows_core::IInspectable {
    fn from(value: &PrintNotificationEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintNotificationEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintNotificationEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintNotificationEventDetails {}
unsafe impl ::core::marker::Sync for PrintNotificationEventDetails {}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfiguration(::windows_core::IUnknown);
impl PrintTaskConfiguration {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn PrinterExtensionContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterExtensionContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SaveRequested)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSaveRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSaveRequested)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfiguration {}
impl ::core::fmt::Debug for PrintTaskConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintTaskConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfiguration;{e3c22451-3aa4-4885-9240-311f5f8fbe9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintTaskConfiguration {
    type Vtable = IPrintTaskConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IPrintTaskConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskConfiguration {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
}
impl ::core::convert::From<PrintTaskConfiguration> for ::windows_core::IUnknown {
    fn from(value: PrintTaskConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfiguration> for ::windows_core::IUnknown {
    fn from(value: &PrintTaskConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintTaskConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintTaskConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfiguration> for ::windows_core::IInspectable {
    fn from(value: PrintTaskConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfiguration> for ::windows_core::IInspectable {
    fn from(value: &PrintTaskConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintTaskConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintTaskConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequest(::windows_core::IUnknown);
impl PrintTaskConfigurationSaveRequest {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn Save<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, printerextensioncontext: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Save)(::windows_core::Interface::as_raw(this), printerextensioncontext.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn GetDeferral(&self) -> ::windows_core::Result<PrintTaskConfigurationSaveRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskConfigurationSaveRequestedDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequest {}
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintTaskConfigurationSaveRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest;{eeaf2fcb-621e-4b62-ac77-b281cce08d60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintTaskConfigurationSaveRequest {
    type Vtable = IPrintTaskConfigurationSaveRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPrintTaskConfigurationSaveRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskConfigurationSaveRequest {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequest> for ::windows_core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequest> for ::windows_core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequest> for ::windows_core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequest> for ::windows_core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintTaskConfigurationSaveRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedDeferral(::windows_core::IUnknown);
impl PrintTaskConfigurationSaveRequestedDeferral {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequestedDeferral {}
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintTaskConfigurationSaveRequestedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral;{e959d568-f729-44a4-871d-bd0628696a33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintTaskConfigurationSaveRequestedDeferral {
    type Vtable = IPrintTaskConfigurationSaveRequestedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IPrintTaskConfigurationSaveRequestedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskConfigurationSaveRequestedDeferral {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedDeferral> for ::windows_core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedDeferral> for ::windows_core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedDeferral> for ::windows_core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedDeferral> for ::windows_core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintTaskConfigurationSaveRequestedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
pub struct PrintTaskConfigurationSaveRequestedEventArgs(::windows_core::IUnknown);
impl PrintTaskConfigurationSaveRequestedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    pub fn Request(&self) -> ::windows_core::Result<PrintTaskConfigurationSaveRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintTaskConfigurationSaveRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskConfigurationSaveRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskConfigurationSaveRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskConfigurationSaveRequestedEventArgs {}
impl ::core::fmt::Debug for PrintTaskConfigurationSaveRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskConfigurationSaveRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintTaskConfigurationSaveRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs;{e06c2879-0d61-4938-91d0-96a45bee8479})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintTaskConfigurationSaveRequestedEventArgs {
    type Vtable = IPrintTaskConfigurationSaveRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintTaskConfigurationSaveRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskConfigurationSaveRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskConfigurationSaveRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskConfigurationSaveRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintTaskConfigurationSaveRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintTaskConfigurationSaveRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
