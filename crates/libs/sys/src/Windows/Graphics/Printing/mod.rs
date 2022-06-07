#[cfg(feature = "Graphics_Printing_OptionDetails")]
pub mod OptionDetails;
#[cfg(feature = "Graphics_Printing_PrintSupport")]
pub mod PrintSupport;
#[cfg(feature = "Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[cfg(feature = "Graphics_Printing_Workflow")]
pub mod Workflow;
#[repr(C)]
pub struct IPrintDocumentSource {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPrintDocumentSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3738962992, data2: 61931, data3: 18399, data4: [170, 230, 237, 84, 39, 81, 31, 1] };
}
#[repr(C)]
pub struct IPrintManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PrintTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintTaskRequested: usize,
}
impl ::windows_sys::core::Interface for IPrintManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4280981140, data2: 35993, data3: 17661, data4: [174, 74, 25, 217, 170, 154, 15, 10] };
}
#[repr(C)]
pub struct IPrintManagerStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowPrintUIAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowPrintUIAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintManagerStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1477991885, data2: 58932, data3: 18004, data4: [132, 240, 224, 21, 42, 130, 23, 172] };
}
#[repr(C)]
pub struct IPrintManagerStatic2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintManagerStatic2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 900307285, data2: 59051, data3: 16697, data4: [154, 189, 184, 106, 114, 155, 53, 152] };
}
#[repr(C)]
pub struct IPrintPageInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMediaSize: unsafe extern "system" fn(this: *mut *mut Self, value: PrintMediaSize) -> ::windows_sys::core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintMediaSize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPageSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    pub SetDpiX: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: PrintOrientation) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintOrientation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintPageInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3712739785, data2: 42657, data3: 19162, data4: [147, 14, 218, 135, 42, 79, 35, 211] };
}
#[repr(C)]
pub struct IPrintPageRange {
    pub base__: ::windows_sys::core::IInspectable,
    pub FirstPageNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub LastPageNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintPageRange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4171263060, data2: 28284, data3: 20933, data4: [87, 253, 6, 96, 194, 215, 21, 19] };
}
#[repr(C)]
pub struct IPrintPageRangeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, firstpage: i32, lastpage: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSinglePage: unsafe extern "system" fn(this: *mut *mut Self, page: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintPageRangeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1083167839, data2: 57415, data3: 24453, data4: [113, 41, 251, 8, 90, 79, 173, 20] };
}
#[repr(C)]
pub struct IPrintPageRangeOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAllowAllPages: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowAllPages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowCurrentPage: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowCurrentPage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowCustomSetOfPages: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowCustomSetOfPages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintPageRangeOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3463296808, data2: 4951, data3: 18098, data4: [169, 35, 121, 249, 149, 244, 72, 252] };
}
#[repr(C)]
pub struct IPrintTask {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Properties: usize,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Previewing: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Previewing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewing: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewing: usize,
    #[cfg(feature = "Foundation")]
    pub Submitting: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitting: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitting: usize,
    #[cfg(feature = "Foundation")]
    pub Progressing: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Progressing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgressing: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgressing: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
}
impl ::windows_sys::core::Interface for IPrintTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1641546311, data2: 27894, data3: 20397, data4: [132, 226, 165, 232, 46, 45, 76, 235] };
}
#[repr(C)]
pub struct IPrintTask2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsPreviewEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPreviewEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTask2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 908281975, data2: 15955, data3: 19869, data4: [143, 94, 49, 106, 200, 222, 218, 225] };
}
#[repr(C)]
pub struct IPrintTaskCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Completion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintTaskCompletion) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1540175023, data2: 9449, data3: 19472, data4: [141, 7, 20, 195, 70, 186, 63, 206] };
}
#[repr(C)]
pub struct IPrintTaskOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBordering: unsafe extern "system" fn(this: *mut *mut Self, value: PrintBordering) -> ::windows_sys::core::HRESULT,
    pub Bordering: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintBordering) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPagePrintTicket: unsafe extern "system" fn(this: *mut *mut Self, printpageinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPagePrintTicket: usize,
}
impl ::windows_sys::core::Interface for IPrintTaskOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1510631099, data2: 53897, data3: 16827, data4: [150, 221, 87, 226, 131, 56, 174, 63] };
}
#[repr(C)]
pub struct IPrintTaskOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PageRangeOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomPageRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomPageRanges: usize,
}
impl ::windows_sys::core::Interface for IPrintTaskOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3952809478, data2: 39478, data3: 19289, data4: [134, 23, 178, 23, 132, 146, 98, 225] };
}
#[repr(C)]
pub struct IPrintTaskOptionsCore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetPageDescription: unsafe extern "system" fn(this: *mut *mut Self, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPageDescription: usize,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionsCore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 467383412, data2: 20177, data3: 16875, data4: [190, 60, 114, 209, 142, 214, 115, 55] };
}
#[repr(C)]
pub struct IPrintTaskOptionsCoreProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMediaSize: unsafe extern "system" fn(this: *mut *mut Self, value: PrintMediaSize) -> ::windows_sys::core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintMediaSize) -> ::windows_sys::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut *mut Self, value: PrintMediaType) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintMediaType) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: PrintOrientation) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintOrientation) -> ::windows_sys::core::HRESULT,
    pub SetPrintQuality: unsafe extern "system" fn(this: *mut *mut Self, value: PrintQuality) -> ::windows_sys::core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintQuality) -> ::windows_sys::core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(this: *mut *mut Self, value: PrintColorMode) -> ::windows_sys::core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintColorMode) -> ::windows_sys::core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(this: *mut *mut Self, value: PrintDuplex) -> ::windows_sys::core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintDuplex) -> ::windows_sys::core::HRESULT,
    pub SetCollation: unsafe extern "system" fn(this: *mut *mut Self, value: PrintCollation) -> ::windows_sys::core::HRESULT,
    pub Collation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintCollation) -> ::windows_sys::core::HRESULT,
    pub SetStaple: unsafe extern "system" fn(this: *mut *mut Self, value: PrintStaple) -> ::windows_sys::core::HRESULT,
    pub Staple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintStaple) -> ::windows_sys::core::HRESULT,
    pub SetHolePunch: unsafe extern "system" fn(this: *mut *mut Self, value: PrintHolePunch) -> ::windows_sys::core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintHolePunch) -> ::windows_sys::core::HRESULT,
    pub SetBinding: unsafe extern "system" fn(this: *mut *mut Self, value: PrintBinding) -> ::windows_sys::core::HRESULT,
    pub Binding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintBinding) -> ::windows_sys::core::HRESULT,
    pub MinCopies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxCopies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNumberOfCopies: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfCopies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionsCoreProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3250001970, data2: 40595, data3: 20053, data4: [129, 75, 51, 38, 165, 158, 252, 225] };
}
#[repr(C)]
pub struct IPrintTaskOptionsCoreUIConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayedOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayedOptions: usize,
}
impl ::windows_sys::core::Interface for IPrintTaskOptionsCoreUIConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1659280931, data2: 39454, data3: 17206, data4: [183, 79, 60, 199, 244, 207, 247, 9] };
}
#[repr(C)]
pub struct IPrintTaskProgressingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DocumentPageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskProgressingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2165101515, data2: 46096, data3: 17026, data4: [160, 115, 90, 195, 120, 35, 65, 116] };
}
#[repr(C)]
pub struct IPrintTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub CreatePrintTask: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1878400558, data2: 10018, data3: 16960, data4: [166, 124, 243, 100, 132, 154, 23, 243] };
}
#[repr(C)]
pub struct IPrintTaskRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskRequestedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488592880, data2: 52798, data3: 17095, data4: [148, 150, 100, 128, 12, 98, 44, 68] };
}
#[repr(C)]
pub struct IPrintTaskRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3501193508, data2: 41755, data3: 17740, data4: [167, 182, 93, 12, 197, 34, 252, 22] };
}
#[repr(C)]
pub struct IPrintTaskSourceRequestedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskSourceRequestedArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4193281982, data2: 62550, data3: 16880, data4: [156, 152, 92, 231, 62, 133, 20, 16] };
}
#[repr(C)]
pub struct IPrintTaskSourceRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskSourceRequestedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1242915025, data2: 27026, data3: 19869, data4: [133, 85, 76, 164, 86, 63, 177, 102] };
}
#[repr(C)]
pub struct IPrintTaskTargetDeviceSupport {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsPrinterTargetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPrinterTargetEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIs3DManufacturingTargetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Is3DManufacturingTargetEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskTargetDeviceSupport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 693989568, data2: 49867, data3: 19325, data4: [176, 234, 147, 9, 80, 145, 162, 32] };
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PrintQuality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Duplex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Collation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Staple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HolePunch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Binding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Copies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InputBin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardPrintTaskOptionsStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024633126, data2: 3536, data3: 19668, data4: [186, 255, 147, 15, 199, 214, 165, 116] };
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bordering: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardPrintTaskOptionsStatic2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1004768244, data2: 31300, data3: 17001, data4: [154, 82, 129, 38, 30, 40, 158, 233] };
}
#[repr(C)]
pub struct IStandardPrintTaskOptionsStatic3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CustomPageRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardPrintTaskOptionsStatic3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3153497734, data2: 14424, data3: 16819, data4: [167, 153, 85, 221, 152, 136, 212, 117] };
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const Bale: Self = Self(4i32);
    pub const BindBottom: Self = Self(5i32);
    pub const BindLeft: Self = Self(6i32);
    pub const BindRight: Self = Self(7i32);
    pub const BindTop: Self = Self(8i32);
    pub const Booklet: Self = Self(9i32);
    pub const EdgeStitchBottom: Self = Self(10i32);
    pub const EdgeStitchLeft: Self = Self(11i32);
    pub const EdgeStitchRight: Self = Self(12i32);
    pub const EdgeStitchTop: Self = Self(13i32);
    pub const Fold: Self = Self(14i32);
    pub const JogOffset: Self = Self(15i32);
    pub const Trim: Self = Self(16i32);
}
impl ::core::marker::Copy for PrintBinding {}
impl ::core::clone::Clone for PrintBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Bordered: Self = Self(3i32);
    pub const Borderless: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintBordering {}
impl ::core::clone::Clone for PrintBordering {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Collated: Self = Self(3i32);
    pub const Uncollated: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintCollation {}
impl ::core::clone::Clone for PrintCollation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Color: Self = Self(3i32);
    pub const Grayscale: Self = Self(4i32);
    pub const Monochrome: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintColorMode {}
impl ::core::clone::Clone for PrintColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const OneSided: Self = Self(3i32);
    pub const TwoSidedShortEdge: Self = Self(4i32);
    pub const TwoSidedLongEdge: Self = Self(5i32);
}
impl ::core::marker::Copy for PrintDuplex {}
impl ::core::clone::Clone for PrintDuplex {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const LeftEdge: Self = Self(4i32);
    pub const RightEdge: Self = Self(5i32);
    pub const TopEdge: Self = Self(6i32);
    pub const BottomEdge: Self = Self(7i32);
}
impl ::core::marker::Copy for PrintHolePunch {}
impl ::core::clone::Clone for PrintHolePunch {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const BusinessCard: Self = Self(3i32);
    pub const CreditCard: Self = Self(4i32);
    pub const IsoA0: Self = Self(5i32);
    pub const IsoA1: Self = Self(6i32);
    pub const IsoA10: Self = Self(7i32);
    pub const IsoA2: Self = Self(8i32);
    pub const IsoA3: Self = Self(9i32);
    pub const IsoA3Extra: Self = Self(10i32);
    pub const IsoA3Rotated: Self = Self(11i32);
    pub const IsoA4: Self = Self(12i32);
    pub const IsoA4Extra: Self = Self(13i32);
    pub const IsoA4Rotated: Self = Self(14i32);
    pub const IsoA5: Self = Self(15i32);
    pub const IsoA5Extra: Self = Self(16i32);
    pub const IsoA5Rotated: Self = Self(17i32);
    pub const IsoA6: Self = Self(18i32);
    pub const IsoA6Rotated: Self = Self(19i32);
    pub const IsoA7: Self = Self(20i32);
    pub const IsoA8: Self = Self(21i32);
    pub const IsoA9: Self = Self(22i32);
    pub const IsoB0: Self = Self(23i32);
    pub const IsoB1: Self = Self(24i32);
    pub const IsoB10: Self = Self(25i32);
    pub const IsoB2: Self = Self(26i32);
    pub const IsoB3: Self = Self(27i32);
    pub const IsoB4: Self = Self(28i32);
    pub const IsoB4Envelope: Self = Self(29i32);
    pub const IsoB5Envelope: Self = Self(30i32);
    pub const IsoB5Extra: Self = Self(31i32);
    pub const IsoB7: Self = Self(32i32);
    pub const IsoB8: Self = Self(33i32);
    pub const IsoB9: Self = Self(34i32);
    pub const IsoC0: Self = Self(35i32);
    pub const IsoC1: Self = Self(36i32);
    pub const IsoC10: Self = Self(37i32);
    pub const IsoC2: Self = Self(38i32);
    pub const IsoC3: Self = Self(39i32);
    pub const IsoC3Envelope: Self = Self(40i32);
    pub const IsoC4: Self = Self(41i32);
    pub const IsoC4Envelope: Self = Self(42i32);
    pub const IsoC5: Self = Self(43i32);
    pub const IsoC5Envelope: Self = Self(44i32);
    pub const IsoC6: Self = Self(45i32);
    pub const IsoC6C5Envelope: Self = Self(46i32);
    pub const IsoC6Envelope: Self = Self(47i32);
    pub const IsoC7: Self = Self(48i32);
    pub const IsoC8: Self = Self(49i32);
    pub const IsoC9: Self = Self(50i32);
    pub const IsoDLEnvelope: Self = Self(51i32);
    pub const IsoDLEnvelopeRotated: Self = Self(52i32);
    pub const IsoSRA3: Self = Self(53i32);
    pub const Japan2LPhoto: Self = Self(54i32);
    pub const JapanChou3Envelope: Self = Self(55i32);
    pub const JapanChou3EnvelopeRotated: Self = Self(56i32);
    pub const JapanChou4Envelope: Self = Self(57i32);
    pub const JapanChou4EnvelopeRotated: Self = Self(58i32);
    pub const JapanDoubleHagakiPostcard: Self = Self(59i32);
    pub const JapanDoubleHagakiPostcardRotated: Self = Self(60i32);
    pub const JapanHagakiPostcard: Self = Self(61i32);
    pub const JapanHagakiPostcardRotated: Self = Self(62i32);
    pub const JapanKaku2Envelope: Self = Self(63i32);
    pub const JapanKaku2EnvelopeRotated: Self = Self(64i32);
    pub const JapanKaku3Envelope: Self = Self(65i32);
    pub const JapanKaku3EnvelopeRotated: Self = Self(66i32);
    pub const JapanLPhoto: Self = Self(67i32);
    pub const JapanQuadrupleHagakiPostcard: Self = Self(68i32);
    pub const JapanYou1Envelope: Self = Self(69i32);
    pub const JapanYou2Envelope: Self = Self(70i32);
    pub const JapanYou3Envelope: Self = Self(71i32);
    pub const JapanYou4Envelope: Self = Self(72i32);
    pub const JapanYou4EnvelopeRotated: Self = Self(73i32);
    pub const JapanYou6Envelope: Self = Self(74i32);
    pub const JapanYou6EnvelopeRotated: Self = Self(75i32);
    pub const JisB0: Self = Self(76i32);
    pub const JisB1: Self = Self(77i32);
    pub const JisB10: Self = Self(78i32);
    pub const JisB2: Self = Self(79i32);
    pub const JisB3: Self = Self(80i32);
    pub const JisB4: Self = Self(81i32);
    pub const JisB4Rotated: Self = Self(82i32);
    pub const JisB5: Self = Self(83i32);
    pub const JisB5Rotated: Self = Self(84i32);
    pub const JisB6: Self = Self(85i32);
    pub const JisB6Rotated: Self = Self(86i32);
    pub const JisB7: Self = Self(87i32);
    pub const JisB8: Self = Self(88i32);
    pub const JisB9: Self = Self(89i32);
    pub const NorthAmerica10x11: Self = Self(90i32);
    pub const NorthAmerica10x12: Self = Self(91i32);
    pub const NorthAmerica10x14: Self = Self(92i32);
    pub const NorthAmerica11x17: Self = Self(93i32);
    pub const NorthAmerica14x17: Self = Self(94i32);
    pub const NorthAmerica4x6: Self = Self(95i32);
    pub const NorthAmerica4x8: Self = Self(96i32);
    pub const NorthAmerica5x7: Self = Self(97i32);
    pub const NorthAmerica8x10: Self = Self(98i32);
    pub const NorthAmerica9x11: Self = Self(99i32);
    pub const NorthAmericaArchitectureASheet: Self = Self(100i32);
    pub const NorthAmericaArchitectureBSheet: Self = Self(101i32);
    pub const NorthAmericaArchitectureCSheet: Self = Self(102i32);
    pub const NorthAmericaArchitectureDSheet: Self = Self(103i32);
    pub const NorthAmericaArchitectureESheet: Self = Self(104i32);
    pub const NorthAmericaCSheet: Self = Self(105i32);
    pub const NorthAmericaDSheet: Self = Self(106i32);
    pub const NorthAmericaESheet: Self = Self(107i32);
    pub const NorthAmericaExecutive: Self = Self(108i32);
    pub const NorthAmericaGermanLegalFanfold: Self = Self(109i32);
    pub const NorthAmericaGermanStandardFanfold: Self = Self(110i32);
    pub const NorthAmericaLegal: Self = Self(111i32);
    pub const NorthAmericaLegalExtra: Self = Self(112i32);
    pub const NorthAmericaLetter: Self = Self(113i32);
    pub const NorthAmericaLetterExtra: Self = Self(114i32);
    pub const NorthAmericaLetterPlus: Self = Self(115i32);
    pub const NorthAmericaLetterRotated: Self = Self(116i32);
    pub const NorthAmericaMonarchEnvelope: Self = Self(117i32);
    pub const NorthAmericaNote: Self = Self(118i32);
    pub const NorthAmericaNumber10Envelope: Self = Self(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: Self = Self(120i32);
    pub const NorthAmericaNumber11Envelope: Self = Self(121i32);
    pub const NorthAmericaNumber12Envelope: Self = Self(122i32);
    pub const NorthAmericaNumber14Envelope: Self = Self(123i32);
    pub const NorthAmericaNumber9Envelope: Self = Self(124i32);
    pub const NorthAmericaPersonalEnvelope: Self = Self(125i32);
    pub const NorthAmericaQuarto: Self = Self(126i32);
    pub const NorthAmericaStatement: Self = Self(127i32);
    pub const NorthAmericaSuperA: Self = Self(128i32);
    pub const NorthAmericaSuperB: Self = Self(129i32);
    pub const NorthAmericaTabloid: Self = Self(130i32);
    pub const NorthAmericaTabloidExtra: Self = Self(131i32);
    pub const OtherMetricA3Plus: Self = Self(132i32);
    pub const OtherMetricA4Plus: Self = Self(133i32);
    pub const OtherMetricFolio: Self = Self(134i32);
    pub const OtherMetricInviteEnvelope: Self = Self(135i32);
    pub const OtherMetricItalianEnvelope: Self = Self(136i32);
    pub const Prc10Envelope: Self = Self(137i32);
    pub const Prc10EnvelopeRotated: Self = Self(138i32);
    pub const Prc16K: Self = Self(139i32);
    pub const Prc16KRotated: Self = Self(140i32);
    pub const Prc1Envelope: Self = Self(141i32);
    pub const Prc1EnvelopeRotated: Self = Self(142i32);
    pub const Prc2Envelope: Self = Self(143i32);
    pub const Prc2EnvelopeRotated: Self = Self(144i32);
    pub const Prc32K: Self = Self(145i32);
    pub const Prc32KBig: Self = Self(146i32);
    pub const Prc32KRotated: Self = Self(147i32);
    pub const Prc3Envelope: Self = Self(148i32);
    pub const Prc3EnvelopeRotated: Self = Self(149i32);
    pub const Prc4Envelope: Self = Self(150i32);
    pub const Prc4EnvelopeRotated: Self = Self(151i32);
    pub const Prc5Envelope: Self = Self(152i32);
    pub const Prc5EnvelopeRotated: Self = Self(153i32);
    pub const Prc6Envelope: Self = Self(154i32);
    pub const Prc6EnvelopeRotated: Self = Self(155i32);
    pub const Prc7Envelope: Self = Self(156i32);
    pub const Prc7EnvelopeRotated: Self = Self(157i32);
    pub const Prc8Envelope: Self = Self(158i32);
    pub const Prc8EnvelopeRotated: Self = Self(159i32);
    pub const Prc9Envelope: Self = Self(160i32);
    pub const Prc9EnvelopeRotated: Self = Self(161i32);
    pub const Roll04Inch: Self = Self(162i32);
    pub const Roll06Inch: Self = Self(163i32);
    pub const Roll08Inch: Self = Self(164i32);
    pub const Roll12Inch: Self = Self(165i32);
    pub const Roll15Inch: Self = Self(166i32);
    pub const Roll18Inch: Self = Self(167i32);
    pub const Roll22Inch: Self = Self(168i32);
    pub const Roll24Inch: Self = Self(169i32);
    pub const Roll30Inch: Self = Self(170i32);
    pub const Roll36Inch: Self = Self(171i32);
    pub const Roll54Inch: Self = Self(172i32);
}
impl ::core::marker::Copy for PrintMediaSize {}
impl ::core::clone::Clone for PrintMediaSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const AutoSelect: Self = Self(3i32);
    pub const Archival: Self = Self(4i32);
    pub const BackPrintFilm: Self = Self(5i32);
    pub const Bond: Self = Self(6i32);
    pub const CardStock: Self = Self(7i32);
    pub const Continuous: Self = Self(8i32);
    pub const EnvelopePlain: Self = Self(9i32);
    pub const EnvelopeWindow: Self = Self(10i32);
    pub const Fabric: Self = Self(11i32);
    pub const HighResolution: Self = Self(12i32);
    pub const Label: Self = Self(13i32);
    pub const MultiLayerForm: Self = Self(14i32);
    pub const MultiPartForm: Self = Self(15i32);
    pub const Photographic: Self = Self(16i32);
    pub const PhotographicFilm: Self = Self(17i32);
    pub const PhotographicGlossy: Self = Self(18i32);
    pub const PhotographicHighGloss: Self = Self(19i32);
    pub const PhotographicMatte: Self = Self(20i32);
    pub const PhotographicSatin: Self = Self(21i32);
    pub const PhotographicSemiGloss: Self = Self(22i32);
    pub const Plain: Self = Self(23i32);
    pub const Screen: Self = Self(24i32);
    pub const ScreenPaged: Self = Self(25i32);
    pub const Stationery: Self = Self(26i32);
    pub const TabStockFull: Self = Self(27i32);
    pub const TabStockPreCut: Self = Self(28i32);
    pub const Transparency: Self = Self(29i32);
    pub const TShirtTransfer: Self = Self(30i32);
    pub const None: Self = Self(31i32);
}
impl ::core::marker::Copy for PrintMediaType {}
impl ::core::clone::Clone for PrintMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const PortraitFlipped: Self = Self(4i32);
    pub const Landscape: Self = Self(5i32);
    pub const LandscapeFlipped: Self = Self(6i32);
}
impl ::core::marker::Copy for PrintOrientation {}
impl ::core::clone::Clone for PrintOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Printing\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription {
    pub PageSize: super::super::Foundation::Size,
    pub ImageableRect: super::super::Foundation::Rect,
    pub DpiX: u32,
    pub DpiY: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for PrintPageDescription {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintPageInfo = *mut ::core::ffi::c_void;
pub type PrintPageRange = *mut ::core::ffi::c_void;
pub type PrintPageRangeOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const Automatic: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
    pub const Fax: Self = Self(5i32);
    pub const High: Self = Self(6i32);
    pub const Normal: Self = Self(7i32);
    pub const Photographic: Self = Self(8i32);
    pub const Text: Self = Self(9i32);
}
impl ::core::marker::Copy for PrintQuality {}
impl ::core::clone::Clone for PrintQuality {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: Self = Self(0i32);
    pub const NotAvailable: Self = Self(1i32);
    pub const PrinterCustom: Self = Self(2i32);
    pub const None: Self = Self(3i32);
    pub const StapleTopLeft: Self = Self(4i32);
    pub const StapleTopRight: Self = Self(5i32);
    pub const StapleBottomLeft: Self = Self(6i32);
    pub const StapleBottomRight: Self = Self(7i32);
    pub const StapleDualLeft: Self = Self(8i32);
    pub const StapleDualRight: Self = Self(9i32);
    pub const StapleDualTop: Self = Self(10i32);
    pub const StapleDualBottom: Self = Self(11i32);
    pub const SaddleStitch: Self = Self(12i32);
}
impl ::core::marker::Copy for PrintStaple {}
impl ::core::clone::Clone for PrintStaple {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintTask = *mut ::core::ffi::c_void;
pub type PrintTaskCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Submitted: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintTaskCompletion {}
impl ::core::clone::Clone for PrintTaskCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintTaskOptions = *mut ::core::ffi::c_void;
pub type PrintTaskProgressingEventArgs = *mut ::core::ffi::c_void;
pub type PrintTaskRequest = *mut ::core::ffi::c_void;
pub type PrintTaskRequestedDeferral = *mut ::core::ffi::c_void;
pub type PrintTaskRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PrintTaskSourceRequestedArgs = *mut ::core::ffi::c_void;
pub type PrintTaskSourceRequestedDeferral = *mut ::core::ffi::c_void;
pub type PrintTaskSourceRequestedHandler = *mut ::core::ffi::c_void;
