#[repr(C)]
pub struct IPrintSupportExtensionSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    #[cfg(feature = "Foundation")]
    pub PrintTicketValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintTicketValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintTicketValidationRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintTicketValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PrintDeviceCapabilitiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintDeviceCapabilitiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintDeviceCapabilitiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintDeviceCapabilitiesChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintSupportExtensionSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003749658, data2: 62662, data3: 21683, data4: [160, 184, 165, 89, 131, 154, 164, 195] };
}
#[repr(C)]
pub struct IPrintSupportExtensionTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintSupportExtensionTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2919773969, data2: 39689, data3: 21969, data4: [160, 174, 42, 20, 197, 248, 61, 106] };
}
#[repr(C)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceCapabilities: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, updatedpdc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceCapabilities: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 362191856, data2: 36904, data3: 22306, data4: [138, 55, 125, 124, 52, 180, 29, 214] };
}
#[repr(C)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    pub SetPrintTicketValidationStatus: unsafe extern "system" fn(this: *mut *mut Self, status: WorkflowPrintTicketValidationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 864964201, data2: 56149, data3: 21959, data4: [131, 56, 239, 100, 104, 10, 143, 144] };
}
#[repr(C)]
pub struct IPrintSupportSessionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
}
impl ::windows_sys::core::Interface for IPrintSupportSessionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2233551279, data2: 30589, data3: 21481, data4: [158, 233, 69, 211, 244, 181, 190, 156] };
}
#[repr(C)]
pub struct IPrintSupportSettingsActivatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IPrintSupportSettingsActivatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 505108062, data2: 40979, data3: 21994, data4: [155, 140, 238, 163, 157, 159, 182, 193] };
}
#[repr(C)]
pub struct IPrintSupportSettingsUISession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SessionPrintTicket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SessionPrintTicket: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LaunchKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SettingsLaunchKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub UpdatePrintTicket: unsafe extern "system" fn(this: *mut *mut Self, printticket: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    UpdatePrintTicket: usize,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrintSupportSettingsUISession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336184401, data2: 33731, data3: 21988, data4: [160, 248, 93, 232, 176, 98, 173, 191] };
}
pub type PrintSupportExtensionSession = *mut ::core::ffi::c_void;
pub type PrintSupportExtensionTriggerDetails = *mut ::core::ffi::c_void;
pub type PrintSupportPrintDeviceCapabilitiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportPrintTicketValidationRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportSessionInfo = *mut ::core::ffi::c_void;
pub type PrintSupportSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportSettingsUISession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl ::core::marker::Copy for SettingsLaunchKind {}
impl ::core::clone::Clone for SettingsLaunchKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl ::core::marker::Copy for WorkflowPrintTicketValidationStatus {}
impl ::core::clone::Clone for WorkflowPrintTicketValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
