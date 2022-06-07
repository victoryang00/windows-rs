#[repr(C)]
pub struct IPrintWorkflowBackgroundSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut *mut Self, setupeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Submitted: unsafe extern "system" fn(this: *mut *mut Self, submittedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitted: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintWorkflowSessionStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowBackgroundSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1534661562, data2: 3166, data3: 21130, data4: [116, 88, 134, 164, 108, 189, 220, 69] };
}
#[repr(C)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRequiresUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1139372866, data2: 5968, data3: 22985, data4: [97, 251, 56, 55, 72, 162, 3, 98] };
}
#[repr(C)]
pub struct IPrintWorkflowConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceAppDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3500852461, data2: 64843, data3: 24053, data4: [75, 182, 141, 13, 21, 158, 190, 63] };
}
#[repr(C)]
pub struct IPrintWorkflowConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AbortPrintFlow: unsafe extern "system" fn(this: *mut *mut Self, reason: PrintWorkflowJobAbortReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowConfiguration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3728018000, data2: 42708, data3: 23522, data4: [139, 154, 9, 211, 211, 158, 167, 128] };
}
#[repr(C)]
pub struct IPrintWorkflowForegroundSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut *mut Self, setupeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub XpsDataAvailable: unsafe extern "system" fn(this: *mut *mut Self, xpsdataavailableeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XpsDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXpsDataAvailable: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXpsDataAvailable: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintWorkflowSessionStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowForegroundSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3348849616, data2: 63724, data3: 19691, data4: [149, 58, 200, 135, 97, 87, 221, 51] };
}
#[repr(C)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowForegroundSetupRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3152249415, data2: 39963, data3: 19923, data4: [155, 43, 200, 4, 104, 217, 65, 179] };
}
#[repr(C)]
pub struct IPrintWorkflowJobActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3569180269, data2: 846, data3: 24064, data4: [166, 22, 249, 97, 160, 51, 220, 200] };
}
#[repr(C)]
pub struct IPrintWorkflowJobBackgroundSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintWorkflowSessionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub JobStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub PdlModificationRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlModificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlModificationRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlModificationRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobBackgroundSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3320605400, data2: 8393, data3: 23889, data4: [133, 7, 39, 52, 180, 111, 150, 197] };
}
#[repr(C)]
pub struct IPrintWorkflowJobNotificationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobNotificationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 182546362, data2: 21400, data3: 24250, data4: [180, 114, 151, 134, 80, 24, 106, 154] };
}
#[repr(C)]
pub struct IPrintWorkflowJobStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub SetSkipSystemRendering: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822689192, data2: 12717, data3: 24073, data4: [176, 215, 96, 27, 151, 241, 97, 173] };
}
#[repr(C)]
pub struct IPrintWorkflowJobTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4280901929, data2: 24802, data3: 20955, data4: [186, 140, 226, 204, 221, 181, 22, 185] };
}
#[repr(C)]
pub struct IPrintWorkflowJobUISession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintWorkflowSessionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PdlDataAvailable: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlDataAvailable: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub JobNotification: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobNotification: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobNotification: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobNotification: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowJobUISession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13136747, data2: 30263, data3: 22151, data4: [163, 2, 15, 102, 77, 42, 172, 101] };
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContent {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPrintWorkflowObjectModelSourceFileContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3278670442, data2: 35370, data3: 16794, data4: [179, 195, 32, 144, 230, 191, 171, 47] };
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, xpsstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowObjectModelSourceFileContentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2477897987, data2: 61459, data3: 22230, data4: [183, 8, 153, 172, 44, 203, 18, 238] };
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelTargetPackage {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPrintWorkflowObjectModelTargetPackage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107030644, data2: 39764, data3: 19617, data4: [173, 58, 151, 156, 61, 68, 221, 172] };
}
#[repr(C)]
pub struct IPrintWorkflowPdlConverter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut *mut Self, printticket: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPdlConverter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1080052578, data2: 2788, data3: 20977, data4: [129, 143, 115, 29, 192, 176, 5, 171] };
}
#[repr(C)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPdlDataAvailableEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3568134992, data2: 5447, data3: 22929, data4: [160, 239, 226, 238, 32, 33, 21, 24] };
}
#[repr(C)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateJobOnPrinter: unsafe extern "system" fn(this: *mut *mut Self, targetcontenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(this: *mut *mut Self, jobattributes: *mut ::core::ffi::c_void, targetcontenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(this: *mut *mut Self, jobattributesbuffer: *mut ::core::ffi::c_void, targetcontenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
    pub GetPdlConverter: unsafe extern "system" fn(this: *mut *mut Self, conversiontype: PrintWorkflowPdlConversionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPdlModificationRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 439589473, data2: 11795, data3: 24285, data4: [167, 7, 206, 236, 97, 215, 51, 59] };
}
#[repr(C)]
pub struct IPrintWorkflowPdlSourceContent {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetContentFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetContentFileAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPdlSourceContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2465725505, data2: 12984, data3: 22187, data4: [132, 94, 177, 230, 139, 58, 237, 213] };
}
#[repr(C)]
pub struct IPrintWorkflowPdlTargetStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(this: *mut *mut Self, status: PrintWorkflowSubmittedStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPdlTargetStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2806177765, data2: 7907, data3: 21161, data4: [159, 159, 46, 32, 67, 24, 15, 209] };
}
#[repr(C)]
pub struct IPrintWorkflowPrinterJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub JobId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintWorkflowPrinterJobStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetJobAttributesAsBuffer: unsafe extern "system" fn(this: *mut *mut Self, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetJobAttributesAsBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub GetJobAttributes: unsafe extern "system" fn(this: *mut *mut Self, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    GetJobAttributes: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub SetJobAttributesFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, jobattributesbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Storage_Streams")))]
    SetJobAttributesFromBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub SetJobAttributes: unsafe extern "system" fn(this: *mut *mut Self, jobattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    SetJobAttributes: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowPrinterJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 302030740, data2: 3348, data3: 21571, data4: [188, 9, 37, 3, 17, 206, 87, 11] };
}
#[repr(C)]
pub struct IPrintWorkflowSourceContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetJobPrintTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetJobPrintTicketAsync: usize,
    pub GetSourceSpoolDataAsStreamContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceSpoolDataAsXpsObjectModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowSourceContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 438879809, data2: 52913, data3: 17715, data4: [187, 115, 251, 230, 62, 239, 219, 24] };
}
#[repr(C)]
pub struct IPrintWorkflowSpoolStreamContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowSpoolStreamContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1927634638, data2: 58374, data3: 19316, data4: [132, 225, 63, 243, 253, 205, 175, 112] };
}
#[repr(C)]
pub struct IPrintWorkflowStreamTarget {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowStreamTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2990258820, data2: 34149, data3: 18571, data4: [152, 57, 28, 158, 124, 122, 169, 22] };
}
#[repr(C)]
pub struct IPrintWorkflowSubmittedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetTarget: unsafe extern "system" fn(this: *mut *mut Self, jobprintticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetTarget: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowSubmittedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 987564609, data2: 14228, data3: 21865, data4: [92, 135, 64, 232, 255, 114, 15, 131] };
}
#[repr(C)]
pub struct IPrintWorkflowSubmittedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self, status: PrintWorkflowSubmittedStatus) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowSubmittedOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 776888854, data2: 15329, data3: 24335, data4: [92, 129, 165, 162, 189, 78, 171, 14] };
}
#[repr(C)]
pub struct IPrintWorkflowTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetAsStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 702162796, data2: 2675, data3: 23277, data4: [79, 61, 151, 13, 50, 81, 240, 87] };
}
#[repr(C)]
pub struct IPrintWorkflowTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1463408744, data2: 40326, data3: 16466, data4: [176, 203, 243, 16, 190, 205, 89, 187] };
}
#[repr(C)]
pub struct IPrintWorkflowUIActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowUIActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3163194445, data2: 2539, data3: 22342, data4: [114, 166, 141, 200, 181, 237, 190, 155] };
}
#[repr(C)]
pub struct IPrintWorkflowUILauncher {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsUILaunchEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAndCompleteUIAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAndCompleteUIAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowUILauncher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1693049391, data2: 5324, data3: 22568, data4: [150, 251, 57, 22, 63, 182, 195, 120] };
}
#[repr(C)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowXpsDataAvailableEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1293009713, data2: 21713, data3: 17230, data4: [190, 14, 130, 197, 250, 88, 229, 178] };
}
pub type PrintWorkflowBackgroundSession = *mut ::core::ffi::c_void;
pub type PrintWorkflowBackgroundSetupRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowConfiguration = *mut ::core::ffi::c_void;
pub type PrintWorkflowForegroundSession = *mut ::core::ffi::c_void;
pub type PrintWorkflowForegroundSetupRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
pub type PrintWorkflowJobActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowJobBackgroundSession = *mut ::core::ffi::c_void;
pub type PrintWorkflowJobNotificationEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowJobStartingEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowJobTriggerDetails = *mut ::core::ffi::c_void;
pub type PrintWorkflowJobUISession = *mut ::core::ffi::c_void;
pub type PrintWorkflowObjectModelSourceFileContent = *mut ::core::ffi::c_void;
pub type PrintWorkflowObjectModelTargetPackage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
pub type PrintWorkflowPdlConverter = *mut ::core::ffi::c_void;
pub type PrintWorkflowPdlDataAvailableEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowPdlModificationRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowPdlSourceContent = *mut ::core::ffi::c_void;
pub type PrintWorkflowPdlTargetStream = *mut ::core::ffi::c_void;
pub type PrintWorkflowPrinterJob = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
pub type PrintWorkflowSourceContent = *mut ::core::ffi::c_void;
pub type PrintWorkflowSpoolStreamContent = *mut ::core::ffi::c_void;
pub type PrintWorkflowStreamTarget = *mut ::core::ffi::c_void;
pub type PrintWorkflowSubmittedEventArgs = *mut ::core::ffi::c_void;
pub type PrintWorkflowSubmittedOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
pub type PrintWorkflowTarget = *mut ::core::ffi::c_void;
pub type PrintWorkflowTriggerDetails = *mut ::core::ffi::c_void;
pub type PrintWorkflowUIActivatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
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
pub type PrintWorkflowUILauncher = *mut ::core::ffi::c_void;
pub type PrintWorkflowXpsDataAvailableEventArgs = *mut ::core::ffi::c_void;
