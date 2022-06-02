#[repr(C)]
pub struct IPrint3DWorkflow {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceID: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetPrintModelPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPrintReady: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPrintReady: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrintRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintRequested: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintRequested: usize,
}
#[repr(C)]
pub struct IPrint3DWorkflow2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PrinterChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrinterChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrinterChanged: usize,
}
#[repr(C)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DWorkflowStatus) -> ::windows_sys::core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, value: Print3DWorkflowDetail) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceChanged: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintExtensionContextStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintNotificationEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrinterName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EventData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEventData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTaskConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrinterExtensionContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSaveRequested: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSaveRequested: usize,
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, printerextensioncontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type Print3DWorkflow = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
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
pub type Print3DWorkflowPrintRequestedEventArgs = *mut ::core::ffi::c_void;
pub type Print3DWorkflowPrinterChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
#[repr(transparent)]
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
pub type PrintNotificationEventDetails = *mut ::core::ffi::c_void;
pub type PrintTaskConfiguration = *mut ::core::ffi::c_void;
pub type PrintTaskConfigurationSaveRequest = *mut ::core::ffi::c_void;
pub type PrintTaskConfigurationSaveRequestedDeferral = *mut ::core::ffi::c_void;
pub type PrintTaskConfigurationSaveRequestedEventArgs = *mut ::core::ffi::c_void;
