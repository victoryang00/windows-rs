#[repr(C)]
pub struct IPrintManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309525570, data2: 36163, data3: 20091, data4: [166, 138, 239, 49, 30, 57, 32, 135] };
}
#[repr(C)]
pub struct IPrintWorkflowConfigurationNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowConfigurationNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3226910218, data2: 40674, data3: 17674, data4: [152, 35, 150, 79, 0, 6, 242, 187] };
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut *mut Self, receiver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1758061687, data2: 39230, data3: 16466, data4: [138, 198, 69, 78, 255, 88, 219, 157] };
}
#[repr(C)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
impl ::windows_sys::core::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107030644, data2: 39764, data3: 19617, data4: [173, 58, 151, 156, 61, 68, 221, 172] };
}
#[repr(C)]
pub struct IPrintWorkflowXpsReceiver {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(this: *mut *mut Self, documentsequenceprintticket: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(this: *mut *mut Self, documentsequenceuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(this: *mut *mut Self, documentid: u32, documentprintticket: *mut ::core::ffi::c_void, documenturi: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(this: *mut *mut Self, documentid: u32, pageid: u32, pagereference: *mut ::core::ffi::c_void, pageuri: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowXpsReceiver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67728244, data2: 30648, data3: 18422, data4: [129, 103, 170, 226, 157, 76, 248, 75] };
}
#[repr(C)]
pub struct IPrintWorkflowXpsReceiver2 {
    pub base__: IPrintWorkflowXpsReceiver,
    pub Failed: unsafe extern "system" fn(this: *mut *mut Self, xpserror: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintWorkflowXpsReceiver2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 37473292, data2: 57259, data3: 19041, data4: [176, 116, 73, 12, 105, 149, 88, 13] };
}
#[repr(C)]
pub struct IPrinting3DManagerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
impl ::windows_sys::core::Interface for IPrinting3DManagerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2627932176, data2: 5252, data3: 17799, data4: [178, 107, 221, 223, 159, 156, 174, 205] };
}
