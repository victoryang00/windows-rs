#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn StartXpsPrintJob(printername: ::windows_sys::core::PCWSTR, jobname: ::windows_sys::core::PCWSTR, outputfilename: ::windows_sys::core::PCWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, printablepageson: *const u8, printablepagesoncount: u32, xpsprintjob: *mut *mut *mut IXpsPrintJob, documentstream: *mut *mut *mut IXpsPrintJobStream, printticketstream: *mut *mut *mut IXpsPrintJobStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob1(printername: ::windows_sys::core::PCWSTR, jobname: ::windows_sys::core::PCWSTR, outputfilename: ::windows_sys::core::PCWSTR, progressevent: super::super::super::Foundation::HANDLE, completionevent: super::super::super::Foundation::HANDLE, xpsprintjob: *mut *mut *mut IXpsPrintJob, printcontentreceiver: *mut *mut *mut super::IXpsOMPackageTarget) -> ::windows_sys::core::HRESULT;
}
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2628665512, data2: 57041, data3: 16841, data4: [169, 253, 215, 53, 239, 51, 174, 218] };
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5684082, data2: 35996, data3: 17938, data4: [189, 15, 147, 1, 42, 135, 9, 157] };
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1675351840, data2: 35604, data3: 17783, data4: [176, 116, 123, 177, 27, 89, 109, 40] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrintDocumentPackageStatusEvent {
    pub base__: super::super::super::System::Com::IDispatch,
    pub PackageStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPrintDocumentPackageStatusEvent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3985688749, data2: 23604, data3: 19717, data4: [161, 236, 14, 138, 155, 58, 215, 175] };
}
#[repr(C)]
pub struct IPrintDocumentPackageTarget {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageTargetTypes: unsafe extern "system" fn(this: *mut *mut Self, targetcount: *mut u32, targettypes: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetPackageTarget: unsafe extern "system" fn(this: *mut *mut Self, guidtargettype: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintDocumentPackageTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 462356164, data2: 12313, data3: 19495, data4: [150, 78, 54, 114, 2, 21, 105, 6] };
}
#[repr(C)]
pub struct IPrintDocumentPackageTargetFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDocumentPackageTargetForPrintJob: unsafe extern "system" fn(this: *mut *mut Self, printername: ::windows_sys::core::PCWSTR, jobname: ::windows_sys::core::PCWSTR, joboutputstream: *mut ::core::ffi::c_void, jobprintticketstream: *mut ::core::ffi::c_void, docpackagetarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDocumentPackageTargetForPrintJob: usize,
}
impl ::windows_sys::core::Interface for IPrintDocumentPackageTargetFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3533020151, data2: 45851, data3: 19005, data4: [150, 0, 113, 46, 177, 51, 91, 164] };
}
#[repr(C)]
pub struct IXpsPrintJob {
    pub base__: ::windows_sys::core::IUnknown,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut *mut Self, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXpsPrintJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1522047750, data2: 33172, data3: 16991, data4: [171, 59, 215, 169, 110, 53, 1, 97] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IXpsPrintJobStream {
    pub base__: super::super::super::System::Com::ISequentialStream,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IXpsPrintJobStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2054675551, data2: 17878, data3: 19967, data4: [147, 7, 216, 203, 132, 99, 71, 202] };
}
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub type PrintDocumentPackageCompletion = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for PrintDocumentPackageStatus {}
impl ::core::clone::Clone for PrintDocumentPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PrintDocumentPackageTarget: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1212311198, data2: 39239, data3: 18154, data4: [139, 162, 216, 204, 228, 50, 194, 202] };
pub const PrintDocumentPackageTargetFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 881783165, data2: 27777, data3: 18818, data4: [146, 180, 238, 24, 138, 67, 134, 122] };
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub type XPS_JOB_COMPLETION = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for XPS_JOB_STATUS {}
impl ::core::clone::Clone for XPS_JOB_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
