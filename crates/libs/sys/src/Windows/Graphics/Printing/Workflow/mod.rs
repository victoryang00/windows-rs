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
#[repr(C)]
pub struct IPrintWorkflowConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceAppDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintWorkflowConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AbortPrintFlow: unsafe extern "system" fn(this: *mut *mut Self, reason: PrintWorkflowJobAbortReason) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPrintWorkflowJobActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPrintWorkflowJobTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContent {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, xpsstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelTargetPackage {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPrintWorkflowPdlConverter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut *mut Self, printticket: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
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
#[repr(C)]
pub struct IPrintWorkflowPdlTargetStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(this: *mut *mut Self, status: PrintWorkflowSubmittedStatus) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPrintWorkflowSpoolStreamContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
}
#[repr(C)]
pub struct IPrintWorkflowStreamTarget {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
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
#[repr(C)]
pub struct IPrintWorkflowSubmittedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self, status: PrintWorkflowSubmittedStatus) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintWorkflowTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetAsStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintWorkflowTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintWorkflowUIActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
