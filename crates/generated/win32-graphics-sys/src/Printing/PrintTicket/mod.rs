#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "win32-storage-sys")]
    pub fn PTCloseProvider(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTConvertDevModeToPrintTicket(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: ::win32_system_sys::Com::IStream) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-graphics-sys", feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTConvertPrintTicketToDevMode(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, pprintticket: ::win32_system_sys::Com::IStream, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTGetPrintCapabilities(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, pprintticket: ::win32_system_sys::Com::IStream, pcapabilities: ::win32_system_sys::Com::IStream, pbstrerrormessage: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTGetPrintDeviceCapabilities(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, pprintticket: ::win32_system_sys::Com::IStream, pdevicecapabilities: ::win32_system_sys::Com::IStream, pbstrerrormessage: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTGetPrintDeviceResources(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, pszlocalename: ::windows_core_sys::PCWSTR, pprintticket: ::win32_system_sys::Com::IStream, pdeviceresources: ::win32_system_sys::Com::IStream, pbstrerrormessage: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(all(feature = "win32-storage-sys", feature = "win32-system-sys"))]
    pub fn PTMergeAndValidatePrintTicket(hprovider: ::win32_storage_sys::Xps::HPTPROVIDER, pbaseticket: ::win32_system_sys::Com::IStream, pdeltaticket: ::win32_system_sys::Com::IStream, scope: EPrintTicketScope, presultticket: ::win32_system_sys::Com::IStream, pbstrerrormessage: *mut ::win32_foundation_sys::BSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-storage-sys")]
    pub fn PTOpenProvider(pszprintername: ::windows_core_sys::PCWSTR, dwversion: u32, phprovider: *mut ::win32_storage_sys::Xps::HPTPROVIDER) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-storage-sys")]
    pub fn PTOpenProviderEx(pszprintername: ::windows_core_sys::PCWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut ::win32_storage_sys::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn PTQuerySchemaVersionSupport(pszprintername: ::windows_core_sys::PCWSTR, pmaxversion: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
}
pub type EDefaultDevmodeType = i32;
pub const kUserDefaultDevmode: EDefaultDevmodeType = 0i32;
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = 1i32;
pub type EPrintTicketScope = i32;
pub const kPTPageScope: EPrintTicketScope = 0i32;
pub const kPTDocumentScope: EPrintTicketScope = 1i32;
pub const kPTJobScope: EPrintTicketScope = 2i32;
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
