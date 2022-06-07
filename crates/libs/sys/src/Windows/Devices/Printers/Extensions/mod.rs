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
impl ::windows_sys::core::Interface for IPrint3DWorkflow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3312415933, data2: 13929, data3: 19046, data4: [171, 66, 200, 21, 25, 48, 205, 52] };
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
impl ::windows_sys::core::Interface for IPrint3DWorkflow2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2728838479, data2: 35521, data3: 18712, data4: [151, 65, 227, 79, 48, 4, 35, 158] };
}
#[repr(C)]
pub struct IPrint3DWorkflowPrintRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DWorkflowStatus) -> ::windows_sys::core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, value: Print3DWorkflowDetail) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSourceChanged: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DWorkflowPrintRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 435734616, data2: 23240, data3: 19285, data4: [138, 95, 230, 21, 103, 218, 251, 77] };
}
#[repr(C)]
pub struct IPrint3DWorkflowPrinterChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DWorkflowPrinterChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1159881730, data2: 38396, data3: 18503, data4: [147, 179, 19, 77, 191, 92, 96, 247] };
}
#[repr(C)]
pub struct IPrintExtensionContextStatic {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintExtensionContextStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3876429761, data2: 65401, data3: 19108, data4: [140, 155, 12, 147, 174, 223, 222, 138] };
}
#[repr(C)]
pub struct IPrintNotificationEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrinterName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EventData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEventData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintNotificationEventDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3759033482, data2: 18472, data3: 19873, data4: [139, 184, 134, 114, 223, 133, 21, 231] };
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
impl ::windows_sys::core::Interface for IPrintTaskConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3821151313, data2: 15012, data3: 18565, data4: [146, 64, 49, 31, 95, 143, 190, 157] };
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
impl ::windows_sys::core::Interface for IPrintTaskConfigurationSaveRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4004458443, data2: 25118, data3: 19298, data4: [172, 119, 178, 129, 204, 224, 141, 96] };
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskConfigurationSaveRequestedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3914978664, data2: 63273, data3: 17572, data4: [135, 29, 189, 6, 40, 105, 106, 51] };
}
#[repr(C)]
pub struct IPrintTaskConfigurationSaveRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintTaskConfigurationSaveRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765184633, data2: 3425, data3: 18744, data4: [145, 208, 150, 164, 91, 238, 132, 121] };
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
