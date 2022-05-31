#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EDefaultDevmodeType(pub i32);
pub const kUserDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(0i32);
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(1i32);
impl ::core::marker::Copy for EDefaultDevmodeType {}
impl ::core::clone::Clone for EDefaultDevmodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDefaultDevmodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EDefaultDevmodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDefaultDevmodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDefaultDevmodeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EPrintTicketScope(pub i32);
pub const kPTPageScope: EPrintTicketScope = EPrintTicketScope(0i32);
pub const kPTDocumentScope: EPrintTicketScope = EPrintTicketScope(1i32);
pub const kPTJobScope: EPrintTicketScope = EPrintTicketScope(2i32);
impl ::core::marker::Copy for EPrintTicketScope {}
impl ::core::clone::Clone for EPrintTicketScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPrintTicketScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EPrintTicketScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPrintTicketScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintTicketScope").field(&self.0).finish()
    }
}
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
#[cfg(feature = "win32-storage")]
#[inline]
pub unsafe fn PTCloseProvider<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>>(hprovider: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTCloseProvider(hprovider: ::win32_storage::Xps::HPTPROVIDER) -> ::windows_core::HRESULT;
        }
        PTCloseProvider(hprovider.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param4: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: Param4) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertDevModeToPrintTicket(hprovider: ::win32_storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        PTConvertDevModeToPrintTicket(hprovider.into_param().abi(), ::core::mem::transmute(cbdevmode), ::core::mem::transmute(pdevmode), ::core::mem::transmute(scope), pprintticket.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-graphics", feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, pprintticket: Param1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertPrintTicketToDevMode(hprovider: ::win32_storage::Xps::HPTPROVIDER, pprintticket: ::windows_core::RawPtr, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        PTConvertPrintTicketToDevMode(hprovider.into_param().abi(), pprintticket.into_param().abi(), ::core::mem::transmute(basedevmodetype), ::core::mem::transmute(scope), ::core::mem::transmute(pcbdevmode), ::core::mem::transmute(ppdevmode), ::core::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTGetPrintCapabilities<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pcapabilities: Param2) -> ::windows_core::Result<::win32_foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintCapabilities(hprovider: ::win32_storage::Xps::HPTPROVIDER, pprintticket: ::windows_core::RawPtr, pcapabilities: ::windows_core::RawPtr, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        PTGetPrintCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pcapabilities.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pdevicecapabilities: Param2) -> ::windows_core::Result<::win32_foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceCapabilities(hprovider: ::win32_storage::Xps::HPTPROVIDER, pprintticket: ::windows_core::RawPtr, pdevicecapabilities: ::windows_core::RawPtr, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        PTGetPrintDeviceCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pdevicecapabilities.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, pszlocalename: Param1, pprintticket: Param2, pdeviceresources: Param3) -> ::windows_core::Result<::win32_foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceResources(hprovider: ::win32_storage::Xps::HPTPROVIDER, pszlocalename: ::windows_core::PCWSTR, pprintticket: ::windows_core::RawPtr, pdeviceresources: ::windows_core::RawPtr, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        PTGetPrintDeviceResources(hprovider.into_param().abi(), pszlocalename.into_param().abi(), pprintticket.into_param().abi(), pdeviceresources.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "win32-storage", feature = "win32-system"))]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<'a, Param0: ::windows_core::IntoParam<'a, ::win32_storage::Xps::HPTPROVIDER>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param4: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hprovider: Param0, pbaseticket: Param1, pdeltaticket: Param2, scope: EPrintTicketScope, presultticket: Param4) -> ::windows_core::Result<::win32_foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTMergeAndValidatePrintTicket(hprovider: ::win32_storage::Xps::HPTPROVIDER, pbaseticket: ::windows_core::RawPtr, pdeltaticket: ::windows_core::RawPtr, scope: EPrintTicketScope, presultticket: ::windows_core::RawPtr, pbstrerrormessage: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        PTMergeAndValidatePrintTicket(hprovider.into_param().abi(), pbaseticket.into_param().abi(), pdeltaticket.into_param().abi(), ::core::mem::transmute(scope), presultticket.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-storage")]
#[inline]
pub unsafe fn PTOpenProvider<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszprintername: Param0, dwversion: u32) -> ::windows_core::Result<::win32_storage::Xps::HPTPROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProvider(pszprintername: ::windows_core::PCWSTR, dwversion: u32, phprovider: *mut ::win32_storage::Xps::HPTPROVIDER) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_storage::Xps::HPTPROVIDER>::zeroed();
        PTOpenProvider(pszprintername.into_param().abi(), ::core::mem::transmute(dwversion), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_storage::Xps::HPTPROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-storage")]
#[inline]
pub unsafe fn PTOpenProviderEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszprintername: Param0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut ::win32_storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProviderEx(pszprintername: ::windows_core::PCWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut ::win32_storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows_core::HRESULT;
        }
        PTOpenProviderEx(pszprintername.into_param().abi(), ::core::mem::transmute(dwmaxversion), ::core::mem::transmute(dwprefversion), ::core::mem::transmute(phprovider), ::core::mem::transmute(pusedversion)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszprintername: Param0) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTQuerySchemaVersionSupport(pszprintername: ::windows_core::PCWSTR, pmaxversion: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        PTQuerySchemaVersionSupport(pszprintername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        PTReleaseMemory(::core::mem::transmute(pbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
