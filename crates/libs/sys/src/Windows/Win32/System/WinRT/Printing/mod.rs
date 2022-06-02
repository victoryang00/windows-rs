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
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut *mut Self, receiver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[repr(C)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
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
#[repr(C)]
pub struct IPrintWorkflowXpsReceiver2 {
    pub base__: IPrintWorkflowXpsReceiver,
    pub Failed: unsafe extern "system" fn(this: *mut *mut Self, xpserror: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
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
