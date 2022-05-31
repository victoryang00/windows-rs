#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b7913ba_0c5e_528a_7458_86a46cbddc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Submitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, submittedeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43e97342_1750_59c9_61fb_383748a20362);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRequiresUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0aac4ed_fd4b_5df5_4bb6_8d0d159ebe3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceAppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowConfiguration2 {
    type Vtable = IPrintWorkflowConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde350a50_a6d4_5be2_8b9a_09d3d39ea780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AbortPrintFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: PrintWorkflowJobAbortReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc79b63d0_f8ec_4ceb_953a_c8876157dd33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub XpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdataavailableeventhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveXpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbe38247_9c1b_4dd3_9b2b_c80468d941b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4bd5e6d_034e_5e00_a616_f961a033dcc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobBackgroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5ec6ad8_20c9_5d51_8507_2734b46f96c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobBackgroundSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub JobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveJobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobNotificationEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ae16fba_5398_5eba_b472_978650186a9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobNotificationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d99ba8_31ad_5e09_b0d7_601b97f161ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Printer: usize,
    pub SetSkipSystemRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff296129_60e2_51db_ba8c_e2ccddb516b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobUISession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00c8736b_7637_5687_a302_0f664d2aac65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobUISession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub PdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub JobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveJobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc36c8a6a_8a2a_419a_b3c3_2090e6bfab2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContentFactory {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93b1b903_f013_56d6_b708_99ac2ccb12ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelTargetPackage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelTargetPackage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40604b62_0ae4_51f1_818f_731dc0b005ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlConverter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-storage"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticket: ::windows_core::RawPtr, inputstream: ::windows_core::RawPtr, outputstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-storage")))]
    ConvertPdlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4ad6b50_1547_5991_a0ef_e2ee20211518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a339a61_2e13_5edd_a707_ceec61d7333b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateJobOnPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetcontenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: ::windows_core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-foundation")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "winrt-storage")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows_core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
    pub GetPdlConverter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversiontype: PrintWorkflowPdlConversionType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlSourceContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92f7fc41_32b8_56ab_845e_b1e68b3aedd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlSourceContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetInputStream: usize,
    #[cfg(feature = "winrt-storage")]
    pub GetContentFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetContentFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlTargetStream(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa742dfe5_1ee3_52a9_9f9f_2e2043180fd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlTargetStream_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPrinterJob(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12009f94_0d14_5443_bc09_250311ce570b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPrinterJob_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub JobId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Printer: usize,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowPrinterJobStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub GetJobPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    GetJobPrintTicket: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub GetJobAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    GetJobAttributesAsBuffer: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub GetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-foundation")))]
    GetJobAttributes: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-storage"))]
    pub SetJobAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-storage")))]
    SetJobAttributesFromBuffer: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub SetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-foundation")))]
    SetJobAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSourceContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a28c641_ceb1_4533_bb73_fbe63eefdb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSourceContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub GetJobPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    GetJobPrintTicketAsync: usize,
    pub GetSourceSpoolDataAsStreamContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSourceSpoolDataAsXpsObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSpoolStreamContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72e55ece_e406_4b74_84e1_3ff3fdcdaf70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSpoolStreamContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetInputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowStreamTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb23bba84_8565_488b_9839_1c9e7c7aa916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowStreamTarget_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetOutputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3add0a41_3794_5569_5c87_40e8ff720f83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub GetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobprintticket: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    GetTarget: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e4e6216_3be1_5f0f_5c81_a5a2bd4eab0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29da276c_0a73_5aed_4f3d_970d3251f057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTarget_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetAsStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5739d868_9d86_4052_b0cb_f310becd59bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUIActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc8a844d_09eb_5746_72a6_8dc8b5edbe9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUIActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUILauncher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e9e22f_14cc_5828_96fb_39163fb6c378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUILauncher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsUILaunchEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub LaunchAndCompleteUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d11c331_54d1_434e_be0e_82c5fa58e5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSession(::windows_core::IUnknown);
impl PrintWorkflowBackgroundSession {
    pub fn SetupRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>>(&self, setupeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SetupRequested)(::windows_core::Interface::as_raw(this), setupeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSetupRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetupRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Submitted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>>(&self, submittedeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Submitted)(::windows_core::Interface::as_raw(this), submittedeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSubmitted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSubmitted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintWorkflowSessionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowBackgroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowBackgroundSession {}
impl ::core::fmt::Debug for PrintWorkflowBackgroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowBackgroundSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowBackgroundSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession;{5b7913ba-0c5e-528a-7458-86a46cbddc45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowBackgroundSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession";
}
impl ::core::convert::From<PrintWorkflowBackgroundSession> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSession> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowBackgroundSession> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSession> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSession {}
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowBackgroundSetupRequestedEventArgs {
    #[cfg(feature = "winrt-graphics")]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn SetRequiresUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiresUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowBackgroundSetupRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowBackgroundSetupRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs;{43e97342-1750-59c9-61fb-383748a20362})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowBackgroundSetupRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSetupRequestedEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowConfiguration(::windows_core::IUnknown);
impl PrintWorkflowConfiguration {
    pub fn SourceAppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn JobTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).JobTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPrintWorkflowConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AbortPrintFlow)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowConfiguration {}
impl ::core::fmt::Debug for PrintWorkflowConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration;{d0aac4ed-fd4b-5df5-4bb6-8d0d159ebe3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration";
}
impl ::core::convert::From<PrintWorkflowConfiguration> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowConfiguration> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowConfiguration> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowConfiguration> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowConfiguration {}
unsafe impl ::core::marker::Sync for PrintWorkflowConfiguration {}
#[repr(transparent)]
pub struct PrintWorkflowForegroundSession(::windows_core::IUnknown);
impl PrintWorkflowForegroundSession {
    pub fn SetupRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>>(&self, setupeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SetupRequested)(::windows_core::Interface::as_raw(this), setupeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSetupRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetupRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn XpsDataAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>>(&self, xpsdataavailableeventhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).XpsDataAvailable)(::windows_core::Interface::as_raw(this), xpsdataavailableeventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveXpsDataAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveXpsDataAvailable)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintWorkflowSessionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowForegroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowForegroundSession {}
impl ::core::fmt::Debug for PrintWorkflowForegroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowForegroundSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowForegroundSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession;{c79b63d0-f8ec-4ceb-953a-c8876157dd33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowForegroundSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession";
}
impl ::core::convert::From<PrintWorkflowForegroundSession> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSession> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowForegroundSession> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSession> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSession {}
#[repr(transparent)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowForegroundSetupRequestedEventArgs {
    #[cfg(feature = "winrt-graphics")]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowForegroundSetupRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowForegroundSetupRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowForegroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs;{bbe38247-9c1b-4dd3-9b2b-c80468d941b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowForegroundSetupRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSetupRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowJobAbortReason(pub i32);
impl PrintWorkflowJobAbortReason {
    pub const JobFailed: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintWorkflowJobAbortReason {}
impl ::core::clone::Clone for PrintWorkflowJobAbortReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowJobAbortReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowJobAbortReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowJobAbortReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobAbortReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobAbortReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowJobAbortReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintWorkflowJobActivatedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobActivatedEventArgs {
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-system"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn Session(&self) -> ::windows_core::Result<PrintWorkflowJobUISession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowJobUISession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobActivatedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs;{d4bd5e6d-034e-5e00-a616-f961a033dcc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<PrintWorkflowJobActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintWorkflowJobActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<&PrintWorkflowJobActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintWorkflowJobActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<PrintWorkflowJobActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintWorkflowJobActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<&PrintWorkflowJobActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintWorkflowJobActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobActivatedEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowJobBackgroundSession(::windows_core::IUnknown);
impl PrintWorkflowJobBackgroundSession {
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintWorkflowSessionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    pub fn JobStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).JobStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveJobStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveJobStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PdlModificationRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PdlModificationRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePdlModificationRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePdlModificationRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobBackgroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobBackgroundSession {}
impl ::core::fmt::Debug for PrintWorkflowJobBackgroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobBackgroundSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobBackgroundSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession;{c5ec6ad8-20c9-5d51-8507-2734b46f96c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobBackgroundSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession";
}
impl ::core::convert::From<PrintWorkflowJobBackgroundSession> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobBackgroundSession> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobBackgroundSession> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobBackgroundSession> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobBackgroundSession {}
#[repr(transparent)]
pub struct PrintWorkflowJobNotificationEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobNotificationEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobNotificationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobNotificationEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobNotificationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobNotificationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobNotificationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs;{0ae16fba-5398-5eba-b472-978650186a9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobNotificationEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobNotificationEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobNotificationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobNotificationEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobNotificationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobNotificationEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobNotificationEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowJobStartingEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobStartingEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn Printer(&self) -> ::windows_core::Result<::winrt_devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Printer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::IppPrintDevice>(result__)
        }
    }
    pub fn SetSkipSystemRendering(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSkipSystemRendering)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobStartingEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs;{e3d99ba8-31ad-5e09-b0d7-601b97f161ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobStartingEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobStartingEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowJobTriggerDetails(::windows_core::IUnknown);
impl PrintWorkflowJobTriggerDetails {
    pub fn PrintWorkflowJobSession(&self) -> ::windows_core::Result<PrintWorkflowJobBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowJobSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowJobBackgroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobTriggerDetails {}
impl ::core::fmt::Debug for PrintWorkflowJobTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails;{ff296129-60e2-51db-ba8c-e2ccddb516b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails";
}
impl ::core::convert::From<PrintWorkflowJobTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobTriggerDetails {}
#[repr(transparent)]
pub struct PrintWorkflowJobUISession(::windows_core::IUnknown);
impl PrintWorkflowJobUISession {
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintWorkflowSessionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    pub fn PdlDataAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PdlDataAvailable)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePdlDataAvailable<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePdlDataAvailable)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn JobNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).JobNotification)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveJobNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveJobNotification)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobUISession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobUISession {}
impl ::core::fmt::Debug for PrintWorkflowJobUISession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobUISession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowJobUISession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession;{00c8736b-7637-5687-a302-0f664d2aac65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowJobUISession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession";
}
impl ::core::convert::From<PrintWorkflowJobUISession> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowJobUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobUISession> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowJobUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobUISession> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowJobUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobUISession> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowJobUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobUISession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobUISession {}
#[repr(transparent)]
pub struct PrintWorkflowObjectModelSourceFileContent(::windows_core::IUnknown);
impl PrintWorkflowObjectModelSourceFileContent {
    #[cfg(feature = "winrt-storage")]
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(xpsstream: Param0) -> ::windows_core::Result<PrintWorkflowObjectModelSourceFileContent> {
        Self::IPrintWorkflowObjectModelSourceFileContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), xpsstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<PrintWorkflowObjectModelSourceFileContent>(result__)
        })
    }
    pub fn IPrintWorkflowObjectModelSourceFileContentFactory<R, F: FnOnce(&IPrintWorkflowObjectModelSourceFileContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PrintWorkflowObjectModelSourceFileContent, IPrintWorkflowObjectModelSourceFileContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowObjectModelSourceFileContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowObjectModelSourceFileContent {}
impl ::core::fmt::Debug for PrintWorkflowObjectModelSourceFileContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowObjectModelSourceFileContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowObjectModelSourceFileContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent;{c36c8a6a-8a2a-419a-b3c3-2090e6bfab2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowObjectModelSourceFileContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowObjectModelSourceFileContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent";
}
impl ::core::convert::From<PrintWorkflowObjectModelSourceFileContent> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowObjectModelSourceFileContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelSourceFileContent> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowObjectModelSourceFileContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowObjectModelSourceFileContent> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowObjectModelSourceFileContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelSourceFileContent> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowObjectModelSourceFileContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelSourceFileContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelSourceFileContent {}
#[repr(transparent)]
pub struct PrintWorkflowObjectModelTargetPackage(::windows_core::IUnknown);
impl PrintWorkflowObjectModelTargetPackage {}
impl ::core::clone::Clone for PrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowObjectModelTargetPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowObjectModelTargetPackage {}
impl ::core::fmt::Debug for PrintWorkflowObjectModelTargetPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowObjectModelTargetPackage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowObjectModelTargetPackage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage;{7d96bc74-9b54-4ca1-ad3a-979c3d44ddac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowObjectModelTargetPackage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowObjectModelTargetPackage {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage";
}
impl ::core::convert::From<PrintWorkflowObjectModelTargetPackage> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowObjectModelTargetPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelTargetPackage> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowObjectModelTargetPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowObjectModelTargetPackage> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowObjectModelTargetPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelTargetPackage> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowObjectModelTargetPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelTargetPackage {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelTargetPackage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowPdlConversionType(pub i32);
impl PrintWorkflowPdlConversionType {
    pub const XpsToPdf: Self = Self(0i32);
    pub const XpsToPwgr: Self = Self(1i32);
    pub const XpsToPclm: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowPdlConversionType {}
impl ::core::clone::Clone for PrintWorkflowPdlConversionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowPdlConversionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowPdlConversionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowPdlConversionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConversionType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlConversionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConversionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintWorkflowPdlConverter(::windows_core::IUnknown);
impl PrintWorkflowPdlConverter {
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-storage"))]
    pub fn ConvertPdlAsync<'a, Param0: ::windows_core::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, printticket: Param0, inputstream: Param1, outputstream: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertPdlAsync)(::windows_core::Interface::as_raw(this), printticket.into_param().abi(), inputstream.into_param().abi(), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlConverter {}
impl ::core::fmt::Debug for PrintWorkflowPdlConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlConverter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter;{40604b62-0ae4-51f1-818f-731dc0b005ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlConverter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter";
}
impl ::core::convert::From<PrintWorkflowPdlConverter> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPdlConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlConverter> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPdlConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlConverter> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPdlConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlConverter> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPdlConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlConverter {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlConverter {}
#[repr(transparent)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(::windows_core::IUnknown);
impl PrintWorkflowPdlDataAvailableEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    pub fn SourceContent(&self) -> ::windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlSourceContent>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlDataAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlDataAvailableEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowPdlDataAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlDataAvailableEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlDataAvailableEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs;{d4ad6b50-1547-5991-a0ef-e2ee20211518})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlDataAvailableEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs";
}
impl ::core::convert::From<PrintWorkflowPdlDataAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlDataAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlDataAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlDataAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlDataAvailableEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowPdlModificationRequestedEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    pub fn SourceContent(&self) -> ::windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlSourceContent>(result__)
        }
    }
    pub fn UILauncher(&self) -> ::windows_core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UILauncher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowUILauncher>(result__)
        }
    }
    pub fn CreateJobOnPrinter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetcontenttype: Param0) -> ::windows_core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinter)(::windows_core::Interface::as_raw(this), targetcontenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub fn CreateJobOnPrinterWithAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::winrt_devices::Printers::IppAttributeValue>>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, jobattributes: Param0, targetcontenttype: Param1) -> ::windows_core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(::windows_core::Interface::as_raw(this), jobattributes.into_param().abi(), targetcontenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CreateJobOnPrinterWithAttributesBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, jobattributesbuffer: Param0, targetcontenttype: Param1) -> ::windows_core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(::windows_core::Interface::as_raw(this), jobattributesbuffer.into_param().abi(), targetcontenttype.into_param().abi(), result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    pub fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows_core::Result<PrintWorkflowPdlConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPdlConverter)(::windows_core::Interface::as_raw(this), conversiontype, result__.as_mut_ptr()).from_abi::<PrintWorkflowPdlConverter>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlModificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlModificationRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowPdlModificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlModificationRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlModificationRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs;{1a339a61-2e13-5edd-a707-ceec61d7333b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlModificationRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowPdlModificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlModificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlModificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlModificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlModificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlModificationRequestedEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowPdlSourceContent(::windows_core::IUnknown);
impl PrintWorkflowPdlSourceContent {
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetInputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetContentFileAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContentFileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlSourceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlSourceContent {}
impl ::core::fmt::Debug for PrintWorkflowPdlSourceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlSourceContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlSourceContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent;{92f7fc41-32b8-56ab-845e-b1e68b3aedd5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlSourceContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent";
}
impl ::core::convert::From<PrintWorkflowPdlSourceContent> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPdlSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlSourceContent> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPdlSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlSourceContent> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPdlSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlSourceContent> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPdlSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlSourceContent {}
#[repr(transparent)]
pub struct PrintWorkflowPdlTargetStream(::windows_core::IUnknown);
impl PrintWorkflowPdlTargetStream {
    #[cfg(feature = "winrt-storage")]
    pub fn GetOutputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    pub fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CompleteStreamSubmission)(::windows_core::Interface::as_raw(this), status).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlTargetStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlTargetStream {}
impl ::core::fmt::Debug for PrintWorkflowPdlTargetStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlTargetStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPdlTargetStream {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream;{a742dfe5-1ee3-52a9-9f9f-2e2043180fd1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlTargetStream as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream";
}
impl ::core::convert::From<PrintWorkflowPdlTargetStream> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPdlTargetStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlTargetStream> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPdlTargetStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlTargetStream> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPdlTargetStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlTargetStream> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPdlTargetStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlTargetStream {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlTargetStream {}
#[repr(transparent)]
pub struct PrintWorkflowPrinterJob(::windows_core::IUnknown);
impl PrintWorkflowPrinterJob {
    pub fn JobId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).JobId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn Printer(&self) -> ::windows_core::Result<::winrt_devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Printer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::IppPrintDevice>(result__)
        }
    }
    pub fn GetJobStatus(&self) -> ::windows_core::Result<PrintWorkflowPrinterJobStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintWorkflowPrinterJobStatus>::zeroed();
            (::windows_core::Interface::vtable(this).GetJobStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowPrinterJobStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn GetJobPrintTicket(&self) -> ::windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetJobPrintTicket)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn GetJobAttributesAsBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, attributenames: Param0) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetJobAttributesAsBuffer)(::windows_core::Interface::as_raw(this), attributenames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub fn GetJobAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, attributenames: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::winrt_devices::Printers::IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetJobAttributes)(::windows_core::Interface::as_raw(this), attributenames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::winrt_devices::Printers::IppAttributeValue>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-storage"))]
    pub fn SetJobAttributesFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, jobattributesbuffer: Param0) -> ::windows_core::Result<::winrt_devices::Printers::IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetJobAttributesFromBuffer)(::windows_core::Interface::as_raw(this), jobattributesbuffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::IppSetAttributesResult>(result__)
        }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-foundation"))]
    pub fn SetJobAttributes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::winrt_devices::Printers::IppAttributeValue>>>>(&self, jobattributes: Param0) -> ::windows_core::Result<::winrt_devices::Printers::IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetJobAttributes)(::windows_core::Interface::as_raw(this), jobattributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::IppSetAttributesResult>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPrinterJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPrinterJob {}
impl ::core::fmt::Debug for PrintWorkflowPrinterJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJob").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPrinterJob {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob;{12009f94-0d14-5443-bc09-250311ce570b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowPrinterJob as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob";
}
impl ::core::convert::From<PrintWorkflowPrinterJob> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPrinterJob> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPrinterJob> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPrinterJob> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPrinterJob {}
unsafe impl ::core::marker::Sync for PrintWorkflowPrinterJob {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowPrinterJobStatus(pub i32);
impl PrintWorkflowPrinterJobStatus {
    pub const Error: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const InProgress: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowPrinterJobStatus {}
impl ::core::clone::Clone for PrintWorkflowPrinterJobStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowPrinterJobStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowPrinterJobStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowPrinterJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJobStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowPrinterJobStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJobStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowSessionStatus(pub i32);
impl PrintWorkflowSessionStatus {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Closed: Self = Self(3i32);
    pub const PdlDataAvailableForModification: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintWorkflowSessionStatus {}
impl ::core::clone::Clone for PrintWorkflowSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowSessionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowSessionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSessionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSessionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSessionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintWorkflowSourceContent(::windows_core::IUnknown);
impl PrintWorkflowSourceContent {
    #[cfg(feature = "winrt-graphics")]
    pub fn GetJobPrintTicketAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetJobPrintTicketAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    pub fn GetSourceSpoolDataAsStreamContent(&self) -> ::windows_core::Result<PrintWorkflowSpoolStreamContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSourceSpoolDataAsStreamContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSpoolStreamContent>(result__)
        }
    }
    pub fn GetSourceSpoolDataAsXpsObjectModel(&self) -> ::windows_core::Result<PrintWorkflowObjectModelSourceFileContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSourceSpoolDataAsXpsObjectModel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowObjectModelSourceFileContent>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSourceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSourceContent {}
impl ::core::fmt::Debug for PrintWorkflowSourceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSourceContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSourceContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent;{1a28c641-ceb1-4533-bb73-fbe63eefdb18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowSourceContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent";
}
impl ::core::convert::From<PrintWorkflowSourceContent> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSourceContent> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSourceContent> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSourceContent> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSourceContent {}
#[repr(transparent)]
pub struct PrintWorkflowSpoolStreamContent(::windows_core::IUnknown);
impl PrintWorkflowSpoolStreamContent {
    #[cfg(feature = "winrt-storage")]
    pub fn GetInputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSpoolStreamContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSpoolStreamContent {}
impl ::core::fmt::Debug for PrintWorkflowSpoolStreamContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSpoolStreamContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSpoolStreamContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent;{72e55ece-e406-4b74-84e1-3ff3fdcdaf70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowSpoolStreamContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent";
}
impl ::core::convert::From<PrintWorkflowSpoolStreamContent> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowSpoolStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSpoolStreamContent> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowSpoolStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSpoolStreamContent> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowSpoolStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSpoolStreamContent> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowSpoolStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSpoolStreamContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSpoolStreamContent {}
#[repr(transparent)]
pub struct PrintWorkflowStreamTarget(::windows_core::IUnknown);
impl PrintWorkflowStreamTarget {
    #[cfg(feature = "winrt-storage")]
    pub fn GetOutputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowStreamTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowStreamTarget {}
impl ::core::fmt::Debug for PrintWorkflowStreamTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowStreamTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowStreamTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget;{b23bba84-8565-488b-9839-1c9e7c7aa916})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowStreamTarget as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget";
}
impl ::core::convert::From<PrintWorkflowStreamTarget> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowStreamTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowStreamTarget> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowStreamTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowStreamTarget> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowStreamTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowStreamTarget> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowStreamTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowStreamTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowStreamTarget {}
#[repr(transparent)]
pub struct PrintWorkflowSubmittedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowSubmittedEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSubmittedOperation>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn GetTarget<'a, Param0: ::windows_core::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>>(&self, jobprintticket: Param0) -> ::windows_core::Result<PrintWorkflowTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTarget)(::windows_core::Interface::as_raw(this), jobprintticket.into_param().abi(), result__.as_mut_ptr()).from_abi::<PrintWorkflowTarget>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSubmittedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowSubmittedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSubmittedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs;{3add0a41-3794-5569-5c87-40e8ff720f83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowSubmittedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs";
}
impl ::core::convert::From<PrintWorkflowSubmittedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowSubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowSubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSubmittedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowSubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowSubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedEventArgs {}
#[repr(transparent)]
pub struct PrintWorkflowSubmittedOperation(::windows_core::IUnknown);
impl PrintWorkflowSubmittedOperation {
    pub fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    pub fn XpsContent(&self) -> ::windows_core::Result<PrintWorkflowSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).XpsContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSourceContent>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSubmittedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSubmittedOperation {}
impl ::core::fmt::Debug for PrintWorkflowSubmittedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSubmittedOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation;{2e4e6216-3be1-5f0f-5c81-a5a2bd4eab0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowSubmittedOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation";
}
impl ::core::convert::From<PrintWorkflowSubmittedOperation> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowSubmittedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedOperation> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowSubmittedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSubmittedOperation> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowSubmittedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedOperation> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowSubmittedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedOperation {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowSubmittedStatus(pub i32);
impl PrintWorkflowSubmittedStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowSubmittedStatus {}
impl ::core::clone::Clone for PrintWorkflowSubmittedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowSubmittedStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowSubmittedStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowSubmittedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowSubmittedStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintWorkflowTarget(::windows_core::IUnknown);
impl PrintWorkflowTarget {
    pub fn TargetAsStream(&self) -> ::windows_core::Result<PrintWorkflowStreamTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetAsStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowStreamTarget>(result__)
        }
    }
    pub fn TargetAsXpsObjectModelPackage(&self) -> ::windows_core::Result<PrintWorkflowObjectModelTargetPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetAsXpsObjectModelPackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowObjectModelTargetPackage>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowTarget {}
impl ::core::fmt::Debug for PrintWorkflowTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTarget;{29da276c-0a73-5aed-4f3d-970d3251f057})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowTarget as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTarget";
}
impl ::core::convert::From<PrintWorkflowTarget> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTarget> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowTarget> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTarget> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowTarget {}
#[repr(transparent)]
pub struct PrintWorkflowTriggerDetails(::windows_core::IUnknown);
impl PrintWorkflowTriggerDetails {
    pub fn PrintWorkflowSession(&self) -> ::windows_core::Result<PrintWorkflowBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowBackgroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowTriggerDetails {}
impl ::core::fmt::Debug for PrintWorkflowTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails;{5739d868-9d86-4052-b0cb-f310becd59bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails";
}
impl ::core::convert::From<PrintWorkflowTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowTriggerDetails {}
#[repr(transparent)]
pub struct PrintWorkflowUIActivatedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowUIActivatedEventArgs {
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-system"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn PrintWorkflowSession(&self) -> ::windows_core::Result<PrintWorkflowForegroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowForegroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowUIActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowUIActivatedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowUIActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUIActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowUIActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs;{bc8a844d-09eb-5746-72a6-8dc8b5edbe9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowUIActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs";
}
impl ::core::convert::From<PrintWorkflowUIActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowUIActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUIActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowUIActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowUIActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowUIActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUIActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowUIActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<PrintWorkflowUIActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintWorkflowUIActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<&PrintWorkflowUIActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintWorkflowUIActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<PrintWorkflowUIActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintWorkflowUIActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl ::core::convert::TryFrom<&PrintWorkflowUIActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintWorkflowUIActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-applicationmodel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowUIActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowUIActivatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintWorkflowUICompletionStatus(pub i32);
impl PrintWorkflowUICompletionStatus {
    pub const Completed: Self = Self(0i32);
    pub const LaunchFailed: Self = Self(1i32);
    pub const JobFailed: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowUICompletionStatus {}
impl ::core::clone::Clone for PrintWorkflowUICompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowUICompletionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintWorkflowUICompletionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowUICompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUICompletionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowUICompletionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowUICompletionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintWorkflowUILauncher(::windows_core::IUnknown);
impl PrintWorkflowUILauncher {
    pub fn IsUILaunchEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUILaunchEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn LaunchAndCompleteUIAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAndCompleteUIAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowUILauncher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowUILauncher {}
impl ::core::fmt::Debug for PrintWorkflowUILauncher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUILauncher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowUILauncher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher;{64e9e22f-14cc-5828-96fb-39163fb6c378})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowUILauncher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher";
}
impl ::core::convert::From<PrintWorkflowUILauncher> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowUILauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUILauncher> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowUILauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowUILauncher> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowUILauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUILauncher> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowUILauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowUILauncher {}
unsafe impl ::core::marker::Sync for PrintWorkflowUILauncher {}
#[repr(transparent)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(::windows_core::IUnknown);
impl PrintWorkflowXpsDataAvailableEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintWorkflowSubmittedOperation>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowXpsDataAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowXpsDataAvailableEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowXpsDataAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowXpsDataAvailableEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintWorkflowXpsDataAvailableEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs;{4d11c331-54d1-434e-be0e-82c5fa58e5b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintWorkflowXpsDataAvailableEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs";
}
impl ::core::convert::From<PrintWorkflowXpsDataAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowXpsDataAvailableEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowXpsDataAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowXpsDataAvailableEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowXpsDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowXpsDataAvailableEventArgs {}
