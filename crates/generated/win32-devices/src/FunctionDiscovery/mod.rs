pub const DEVICEDISPLAY_DISCOVERYMETHOD_AD_PRINTER: &str = "Published Printer";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_ASP_INFRA: &str = "AspInfra";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH: &str = "Bluetooth";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH_LE: &str = "Bluetooth Low Energy";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_NETBIOS: &str = "NetBIOS";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_PNP: &str = "PnP";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_UPNP: &str = "UPnP";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WFD: &str = "WiFiDirect";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WSD: &str = "WSD";
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WUSB: &str = "WUSB";
pub const E_FDPAIRING_AUTHFAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193917i32);
pub const E_FDPAIRING_AUTHNOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193914i32);
pub const E_FDPAIRING_CONNECTTIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193916i32);
pub const E_FDPAIRING_HWFAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193918i32);
pub const E_FDPAIRING_IPBUSDISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193913i32);
pub const E_FDPAIRING_NOCONNECTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193919i32);
pub const E_FDPAIRING_NOPROFILES: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193912i32);
pub const E_FDPAIRING_TOOMANYCONNECTIONS: ::windows_core::HRESULT = ::windows_core::HRESULT(-1882193915i32);
pub const FCTN_CATEGORY_BT: &str = "Provider\\Microsoft.Devices.Bluetooth";
pub const FCTN_CATEGORY_DEVICEDISPLAYOBJECTS: &str = "Provider\\Microsoft.Base.DeviceDisplayObjects";
pub const FCTN_CATEGORY_DEVICEFUNCTIONENUMERATORS: &str = "Layered\\Microsoft.Devices.FunctionEnumerators";
pub const FCTN_CATEGORY_DEVICEPAIRING: &str = "Layered\\Microsoft.Base.DevicePairing";
pub const FCTN_CATEGORY_DEVICES: &str = "Layered\\Microsoft.Base.Devices";
pub const FCTN_CATEGORY_DEVQUERYOBJECTS: &str = "Provider\\Microsoft.Base.DevQueryObjects";
pub const FCTN_CATEGORY_NETBIOS: &str = "Provider\\Microsoft.Networking.Netbios";
pub const FCTN_CATEGORY_NETWORKDEVICES: &str = "Layered\\Microsoft.Networking.Devices";
pub const FCTN_CATEGORY_PNP: &str = "Provider\\Microsoft.Base.PnP";
pub const FCTN_CATEGORY_PNPXASSOCIATION: &str = "Provider\\Microsoft.PnPX.Association";
pub const FCTN_CATEGORY_PUBLICATION: &str = "Provider\\Microsoft.Base.Publication";
pub const FCTN_CATEGORY_REGISTRY: &str = "Provider\\Microsoft.Base.Registry";
pub const FCTN_CATEGORY_SSDP: &str = "Provider\\Microsoft.Networking.SSDP";
pub const FCTN_CATEGORY_WCN: &str = "Provider\\Microsoft.Networking.WCN";
pub const FCTN_CATEGORY_WSDISCOVERY: &str = "Provider\\Microsoft.Networking.WSD";
pub const FCTN_CATEGORY_WUSB: &str = "Provider\\Microsoft.Devices.WirelessUSB";
pub const FCTN_SUBCAT_DEVICES_WSDPRINTERS: &str = "WSDPrinters";
pub const FCTN_SUBCAT_NETWORKDEVICES_SSDP: &str = "SSDP";
pub const FCTN_SUBCAT_NETWORKDEVICES_WSD: &str = "WSD";
pub const FCTN_SUBCAT_REG_DIRECTED: &str = "Directed";
pub const FCTN_SUBCAT_REG_PUBLICATION: &str = "Publication";
pub const FD_CONSTRAINTVALUE_ALL: &str = "All";
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_INPROC_SERVER: &str = "1";
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_LOCAL_SERVER: &str = "4";
pub const FD_CONSTRAINTVALUE_FALSE: &str = "FALSE";
pub const FD_CONSTRAINTVALUE_PAIRED: &str = "Paired";
pub const FD_CONSTRAINTVALUE_RECURSESUBCATEGORY_TRUE: &str = "TRUE";
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_ALL: &str = "All";
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_DIRECT: &str = "Direct";
pub const FD_CONSTRAINTVALUE_TRUE: &str = "TRUE";
pub const FD_CONSTRAINTVALUE_UNPAIRED: &str = "UnPaired";
pub const FD_CONSTRAINTVALUE_VISIBILITY_ALL: &str = "1";
pub const FD_CONSTRAINTVALUE_VISIBILITY_DEFAULT: &str = "0";
pub const FD_EVENTID: u32 = 1000u32;
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
pub const FD_LONGHORN: u32 = 1u32;
pub const FD_QUERYCONSTRAINT_COMCLSCONTEXT: &str = "COMClsContext";
pub const FD_QUERYCONSTRAINT_INQUIRY_TIMEOUT: &str = "InquiryModeTimeout";
pub const FD_QUERYCONSTRAINT_PAIRING_STATE: &str = "PairingState";
pub const FD_QUERYCONSTRAINT_PROVIDERINSTANCEID: &str = "ProviderInstanceID";
pub const FD_QUERYCONSTRAINT_RECURSESUBCATEGORY: &str = "RecurseSubcategory";
pub const FD_QUERYCONSTRAINT_ROUTINGSCOPE: &str = "RoutingScope";
pub const FD_QUERYCONSTRAINT_SUBCATEGORY: &str = "Subcategory";
pub const FD_QUERYCONSTRAINT_VISIBILITY: &str = "Visibility";
pub const FD_SUBKEY: &str = "SOFTWARE\\Microsoft\\Function Discovery\\";
pub const FD_Visibility_Default: u32 = 0u32;
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57);
pub const FMTID_DeviceInterface: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560);
pub const FMTID_FD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146);
pub const FMTID_PNPX: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd);
pub const FMTID_PNPXDynamicProperty: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd);
pub const FMTID_Pairing: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc);
pub const FMTID_WSD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92);
pub const FunctionDiscovery: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc72be2ec_8e90_452c_b29a_ab8ff1c071fc);
pub const FunctionInstanceCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba818ce5_b55f_443f_ad39_2fe89be6191f);
#[repr(transparent)]
pub struct IFunctionDiscovery(::windows_core::IUnknown);
impl IFunctionDiscovery {
    pub unsafe fn GetInstanceCollection<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszcategory: Param0, pszsubcategory: Param1, fincludeallsubcategories: Param2) -> ::windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceCollection)(::windows_core::Interface::as_raw(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszfunctioninstanceidentity: Param0) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInstance)(::windows_core::Interface::as_raw(self), pszfunctioninstanceidentity.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn CreateInstanceCollectionQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszcategory: Param0, pszsubcategory: Param1, fincludeallsubcategories: Param2, pifunctiondiscoverynotification: Param3, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(::windows_core::Interface::as_raw(self), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), fincludeallsubcategories.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancecollectionquery)).ok()
    }
    pub unsafe fn CreateInstanceQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszfunctioninstanceidentity: Param0, pifunctiondiscoverynotification: Param1, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateInstanceQuery)(::windows_core::Interface::as_raw(self), pszfunctioninstanceidentity.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancequery)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn RemoveInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: Param1, pszsubcategory: Param2, pszcategoryidentity: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enumsystemvisibility), pszcategory.into_param().abi(), pszsubcategory.into_param().abi(), pszcategoryidentity.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFunctionDiscovery> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscovery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscovery> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscovery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscovery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscovery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscovery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscovery {}
impl ::core::fmt::Debug for IFunctionDiscovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscovery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscovery {
    type Vtable = IFunctionDiscovery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4df99b70_e148_4432_b004_4c9eeb535a5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetInstanceCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: ::win32_foundation::BOOL, ppifunctioninstancecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstance: usize,
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: ::win32_foundation::BOOL, pifunctiondiscoverynotification: ::windows_core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, pifunctiondiscoverynotification: ::windows_core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionDiscoveryNotification(::windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnUpdate<'a, Param2: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnUpdate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enumqueryupdateaction), ::core::mem::transmute(fdqcquerycontext), pifunctioninstance.into_param().abi()).ok()
    }
    pub unsafe fn OnError<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr), ::core::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
    pub unsafe fn OnEvent<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dweventid), ::core::mem::transmute(fdqcquerycontext), pszprovider.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFunctionDiscoveryNotification> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscoveryNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscoveryNotification> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscoveryNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscoveryNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscoveryNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryNotification {}
impl ::core::fmt::Debug for IFunctionDiscoveryNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscoveryNotification {
    type Vtable = IFunctionDiscoveryNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f6c1ba8_5330_422e_a368_572b244d3f87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnUpdate: usize,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionDiscoveryProvider(::windows_core::IUnknown);
impl IFunctionDiscoveryProvider {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, IFunctionDiscoveryProviderFactory>, Param1: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pifunctiondiscoveryproviderfactory: Param0, pifunctiondiscoverynotification: Param1, lciduserdefault: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pifunctiondiscoveryproviderfactory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi(), ::core::mem::transmute(lciduserdefault), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Query<'a, Param0: ::windows_core::IntoParam<'a, IFunctionDiscoveryProviderQuery>>(&self, pifunctiondiscoveryproviderquery: Param0) -> ::windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), pifunctiondiscoveryproviderquery.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
    pub unsafe fn EndQuery(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndQuery)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreValidateAccess<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstancePropertyStoreValidateAccess)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwstgaccess)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn InstancePropertyStoreOpen<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InstancePropertyStoreOpen)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwstgaccess), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreFlush<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstancePropertyStoreFlush)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceQueryService<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).InstanceQueryService)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceReleased<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstanceReleased)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext)).ok()
    }
}
impl ::core::convert::From<IFunctionDiscoveryProvider> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscoveryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProvider> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscoveryProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscoveryProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscoveryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscoveryProvider {
    type Vtable = IFunctionDiscoveryProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcde394f_1478_4813_a402_f6fb10657222);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: ::windows_core::RawPtr, pifunctiondiscoverynotification: ::windows_core::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: ::windows_core::RawPtr, ppifunctioninstancecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreValidateAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreValidateAccess: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub InstancePropertyStoreOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    InstancePropertyStoreOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreFlush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreFlush: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceQueryService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceQueryService: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceReleased: usize,
}
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderFactory(::windows_core::IUnknown);
impl IFunctionDiscoveryProviderFactory {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreatePropertyStore(&self) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreatePropertyStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>, Param4: ::windows_core::IntoParam<'a, IFunctionDiscoveryProvider>>(&self, pszsubcategory: Param0, pszproviderinstanceidentity: Param1, iproviderinstancecontext: isize, pipropertystore: Param3, pifunctiondiscoveryprovider: Param4) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), pipropertystore.into_param().abi(), pifunctiondiscoveryprovider.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn CreateFunctionInstanceCollection(&self) -> ::windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFunctionInstanceCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
impl ::core::convert::From<IFunctionDiscoveryProviderFactory> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscoveryProviderFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProviderFactory> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscoveryProviderFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscoveryProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderFactory {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscoveryProviderFactory {
    type Vtable = IFunctionDiscoveryProviderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86443ff0_1ad5_4e68_a45a_40c2c329de3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreatePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreatePropertyStore: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows_core::RawPtr, pifunctiondiscoveryprovider: ::windows_core::RawPtr, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateInstance: usize,
    pub CreateFunctionInstanceCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderQuery(::windows_core::IUnknown);
impl IFunctionDiscoveryProviderQuery {
    pub unsafe fn IsInstanceQuery(&self, pisinstancequery: *mut ::win32_foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsInstanceQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pisinstancequery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut ::win32_foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSubcategoryQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pissubcategoryquery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn GetQueryConstraints(&self) -> ::windows_core::Result<IProviderQueryConstraintCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetQueryConstraints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProviderQueryConstraintCollection>(result__)
    }
    pub unsafe fn GetPropertyConstraints(&self) -> ::windows_core::Result<IProviderPropertyConstraintCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyConstraints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProviderPropertyConstraintCollection>(result__)
    }
}
impl ::core::convert::From<IFunctionDiscoveryProviderQuery> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscoveryProviderQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscoveryProviderQuery> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscoveryProviderQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscoveryProviderQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscoveryProviderQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderQuery {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscoveryProviderQuery {
    type Vtable = IFunctionDiscoveryProviderQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6876ea98_baec_46db_bc20_75a76e267a3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderQuery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsInstanceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinstancequery: *mut ::win32_foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT,
    pub IsSubcategoryQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut ::win32_foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT,
    pub GetQueryConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPropertyConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionDiscoveryServiceProvider(::windows_core::IUnknown);
impl IFunctionDiscoveryServiceProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>, T: ::windows_core::Interface>(&self, pifunctioninstance: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IFunctionDiscoveryServiceProvider> for ::windows_core::IUnknown {
    fn from(value: IFunctionDiscoveryServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionDiscoveryServiceProvider> for ::windows_core::IUnknown {
    fn from(value: &IFunctionDiscoveryServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionDiscoveryServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionDiscoveryServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryServiceProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionDiscoveryServiceProvider {
    type Vtable = IFunctionDiscoveryServiceProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c81ed02_1b04_43f2_a451_69966cbcd1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryServiceProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFunctionInstance(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstance {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.QueryService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows_core::Result<*mut u16> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    pub unsafe fn GetProviderInstanceID(&self) -> ::windows_core::Result<*mut u16> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderInstanceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: ::win32_system::Com::StructuredStorage::STGM) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenPropertyStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwstgaccess), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppszcomemcategory), ::core::mem::transmute(ppszcomemsubcategory)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFunctionInstance> for ::windows_core::IUnknown {
    fn from(value: IFunctionInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFunctionInstance> for ::windows_core::IUnknown {
    fn from(value: &IFunctionInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFunctionInstance> for ::win32_system::Com::IServiceProvider {
    fn from(value: IFunctionInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFunctionInstance> for ::win32_system::Com::IServiceProvider {
    fn from(value: &IFunctionInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IServiceProvider> for IFunctionInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IServiceProvider> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IServiceProvider> for &'a IFunctionInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IServiceProvider> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFunctionInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFunctionInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFunctionInstance {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFunctionInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstance").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFunctionInstance {
    type Vtable = IFunctionInstance_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: ::win32_system::Com::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows_core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstgaccess: ::win32_system::Com::StructuredStorage::STGM, ppipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionInstanceCollection(::windows_core::IUnknown);
impl IFunctionInstanceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), pszinstanceidentity.into_param().abi(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppifunctioninstance)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex)).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IFunctionInstanceCollection> for ::windows_core::IUnknown {
    fn from(value: IFunctionInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionInstanceCollection> for ::windows_core::IUnknown {
    fn from(value: &IFunctionInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionInstanceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionInstanceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollection {}
impl ::core::fmt::Debug for IFunctionInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionInstanceCollection {
    type Vtable = IFunctionInstanceCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Get: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionInstanceCollectionQuery(::windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszconstraintname: Param0, pszconstraintvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddQueryConstraint)(::windows_core::Interface::as_raw(self), pszconstraintname.into_param().abi(), pszconstraintvalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn AddPropertyConstraint(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pv: *const ::win32_system::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyConstraint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(pv), ::core::mem::transmute(enumpropertyconstraint)).ok()
    }
    pub unsafe fn Execute(&self) -> ::windows_core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Execute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
impl ::core::convert::From<IFunctionInstanceCollectionQuery> for ::windows_core::IUnknown {
    fn from(value: IFunctionInstanceCollectionQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionInstanceCollectionQuery> for ::windows_core::IUnknown {
    fn from(value: &IFunctionInstanceCollectionQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionInstanceCollectionQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionInstanceCollectionQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollectionQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollectionQuery {}
impl ::core::fmt::Debug for IFunctionInstanceCollectionQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollectionQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionInstanceCollectionQuery {
    type Vtable = IFunctionInstanceCollectionQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57cc6fd2_c09a_4289_bb72_25f04142058e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddQueryConstraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, pszconstraintvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, pv: *const ::win32_system::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFunctionInstanceQuery(::windows_core::IUnknown);
impl IFunctionInstanceQuery {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Execute(&self) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Execute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
}
impl ::core::convert::From<IFunctionInstanceQuery> for ::windows_core::IUnknown {
    fn from(value: IFunctionInstanceQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFunctionInstanceQuery> for ::windows_core::IUnknown {
    fn from(value: &IFunctionInstanceQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFunctionInstanceQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFunctionInstanceQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFunctionInstanceQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceQuery {}
impl ::core::fmt::Debug for IFunctionInstanceQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFunctionInstanceQuery {
    type Vtable = IFunctionInstanceQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Execute: usize,
}
#[repr(transparent)]
pub struct IPNPXAssociation(::windows_core::IUnknown);
impl IPNPXAssociation {
    pub unsafe fn Associate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubcategory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Associate)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi()).ok()
    }
    pub unsafe fn Unassociate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubcategory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unassociate)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi()).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszsubcategory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPNPXAssociation> for ::windows_core::IUnknown {
    fn from(value: IPNPXAssociation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPNPXAssociation> for ::windows_core::IUnknown {
    fn from(value: &IPNPXAssociation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPNPXAssociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPNPXAssociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPNPXAssociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPNPXAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXAssociation {}
impl ::core::fmt::Debug for IPNPXAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPNPXAssociation {
    type Vtable = IPNPXAssociation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bd7e521_4da6_42d5_81ba_1981b6b94075);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXAssociation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Associate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPNPXDeviceAssociation(::windows_core::IUnknown);
impl IPNPXDeviceAssociation {
    pub unsafe fn Associate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Associate)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    pub unsafe fn Unassociate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unassociate)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IFunctionDiscoveryNotification>>(&self, pszsubcategory: Param0, pifunctiondiscoverynotification: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), pszsubcategory.into_param().abi(), pifunctiondiscoverynotification.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPNPXDeviceAssociation> for ::windows_core::IUnknown {
    fn from(value: IPNPXDeviceAssociation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPNPXDeviceAssociation> for ::windows_core::IUnknown {
    fn from(value: &IPNPXDeviceAssociation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPNPXDeviceAssociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPNPXDeviceAssociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPNPXDeviceAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXDeviceAssociation {}
impl ::core::fmt::Debug for IPNPXDeviceAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXDeviceAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPNPXDeviceAssociation {
    type Vtable = IPNPXDeviceAssociation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeed366d0_35b8_4fc5_8d20_7e5bd31f6ded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXDeviceAssociation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Associate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyStoreCollection(::windows_core::IUnknown);
impl IPropertyStoreCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszinstanceidentity: Param0, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), pszinstanceidentity.into_param().abi(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppipropertystore)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_ui::Shell::PropertiesSystem::IPropertyStore>>(&self, pipropertystore: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pipropertystore.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex)).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPropertyStoreCollection> for ::windows_core::IUnknown {
    fn from(value: IPropertyStoreCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStoreCollection> for ::windows_core::IUnknown {
    fn from(value: &IPropertyStoreCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyStoreCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyStoreCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyStoreCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCollection {}
impl ::core::fmt::Debug for IPropertyStoreCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreCollection {
    type Vtable = IPropertyStoreCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Get: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Item: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipropertystore: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProviderProperties(::windows_core::IUnknown);
impl IProviderProperties {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCount<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetAt<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, dwindex: u32) -> ::windows_core::Result<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_ui::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::win32_system::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::StructuredStorage::PROPVARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, IFunctionInstance>>(&self, pifunctioninstance: Param0, iproviderinstancecontext: isize, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), pifunctioninstance.into_param().abi(), ::core::mem::transmute(iproviderinstancecontext), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar)).ok()
    }
}
impl ::core::convert::From<IProviderProperties> for ::windows_core::IUnknown {
    fn from(value: IProviderProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderProperties> for ::windows_core::IUnknown {
    fn from(value: &IProviderProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProviderProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProviderProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProviderProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderProperties {}
impl ::core::fmt::Debug for IProviderProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProviderProperties {
    type Vtable = IProviderProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf986ea6_3b5f_4c5f_b88a_2f8b20ceef17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderProperties_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCount: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows_core::RawPtr, iproviderinstancecontext: isize, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const ::win32_system::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetValue: usize,
}
#[repr(transparent)]
pub struct IProviderPropertyConstraintCollection(::windows_core::IUnknown);
impl IProviderPropertyConstraintCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Get(&self, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Item(&self, dwindex: u32, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Next(&self, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    pub unsafe fn Skip(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IProviderPropertyConstraintCollection> for ::windows_core::IUnknown {
    fn from(value: IProviderPropertyConstraintCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderPropertyConstraintCollection> for ::windows_core::IUnknown {
    fn from(value: &IProviderPropertyConstraintCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProviderPropertyConstraintCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProviderPropertyConstraintCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderPropertyConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPropertyConstraintCollection {}
impl ::core::fmt::Debug for IProviderPropertyConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPropertyConstraintCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProviderPropertyConstraintCollection {
    type Vtable = IProviderPropertyConstraintCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4fae42f_5778_4a13_8540_b5fd8c1398dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPropertyConstraintCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Get: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Item: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::win32_system::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProviderPublishing(::windows_core::IUnknown);
impl IProviderPublishing {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows_core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn RemoveInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: Param1, pszproviderinstanceidentity: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enumvisibilityflags), pszsubcategory.into_param().abi(), pszproviderinstanceidentity.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IProviderPublishing> for ::windows_core::IUnknown {
    fn from(value: IProviderPublishing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderPublishing> for ::windows_core::IUnknown {
    fn from(value: &IProviderPublishing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProviderPublishing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProviderPublishing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProviderPublishing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderPublishing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPublishing {}
impl ::core::fmt::Debug for IProviderPublishing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPublishing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProviderPublishing {
    type Vtable = IProviderPublishing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd1b9a04_206c_4a05_a0c8_1635a21a2b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPublishing_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProviderQueryConstraintCollection(::windows_core::IUnknown);
impl IProviderQueryConstraintCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Get<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszconstraintname: Param0) -> ::windows_core::Result<*mut u16> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u16>::zeroed();
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), pszconstraintname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn Skip(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IProviderQueryConstraintCollection> for ::windows_core::IUnknown {
    fn from(value: IProviderQueryConstraintCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderQueryConstraintCollection> for ::windows_core::IUnknown {
    fn from(value: &IProviderQueryConstraintCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProviderQueryConstraintCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProviderQueryConstraintCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderQueryConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderQueryConstraintCollection {}
impl ::core::fmt::Debug for IProviderQueryConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderQueryConstraintCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProviderQueryConstraintCollection {
    type Vtable = IProviderQueryConstraintCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c243e11_3261_4bcd_b922_84a873d460ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderQueryConstraintCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: &str = "OwnerName";
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Characteristics: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassCoInstallers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassInstaller: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DefaultService: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DevType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Exclusive: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 28u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_IconPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_LowerFilters: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Name: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoDisplayClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoInstallClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoUseClass: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_PropPageProvider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Security: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SecuritySDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SilentInstall: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_UpperFilters: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Address: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 51u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AssociationArray: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 80u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_BaselineExperienceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 78u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 95u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 93u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription1: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription2: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DiscoveryMethod: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ExperienceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 89u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_InstallInProgress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsAuthenticated: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 54u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsConnected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDefaultDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 79u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsEncrypted: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 53u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsLocalMachine: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 72u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNetworkDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 74u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsPaired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsSharedDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 68u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Connected: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 67u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Seen: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 66u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 76u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataCabinet: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 87u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataChecksum: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 73u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_PrimaryCategory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 97u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 88u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 99u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_UnpairUninstall: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 98u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Version: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 65u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_ClassGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Enabled: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_AdditionalSoftwareRequested: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Address: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 30u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BIOSVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BaseContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 38u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusRelations: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusReportedDeviceDesc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusTypeGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Capabilities: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Characteristics: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Children: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Class: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ClassGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_CompatibleIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ConfigFlags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ContainerId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DHP_Rebalance_Policy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevNodeStatus: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DeviceDesc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Driver: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverCoInstallers: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDesc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfPath: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSection: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSectionExt: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverLogoLevel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverPropPageProvider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverProvider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverRank: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EjectionRelations: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EnumeratorName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Exclusive: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 28u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyNameAttributes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_GenericDriverInstalled: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_HardwareIds: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallInProgress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 36u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstanceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_IsAssociateableByUserAction: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Legacy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LegacyBusType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 22u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationInfo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationPaths: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LowerFilters: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ManufacturerAttributes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_MatchingDeviceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ModelId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_NoConnectSound: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Numa_Node: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PDOName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Parent: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerData: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 32u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerRelations: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PresenceNotForDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ProblemCode: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 33u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyDefault: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 34u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 35u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalRelations: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Reported: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerExceptions: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerTags: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequired: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequiredOverride: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Security: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SecuritySDS: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Service: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Siblings: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SignalStrength: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_TransportRelations: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumberDescFormat: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 31u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UpperFilters: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_BrandingIcon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DetailedDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DocumentationLink: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Model: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_VendorWebSite: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FunctionInstance: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Devinst: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4097u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DisplayAttribute: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverDate: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverProvider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Function: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4099u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Icon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Image: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4098u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Manufacturer: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Model: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Name: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_SerialNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_ShellAttributes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Status: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4096u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NAME: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Numa_Proximity_Domain: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Associated: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Category_Desc_NonPlural: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12304u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompactSignature: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28674u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompatibleTypes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12292u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory_Desc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12293u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCertHash: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28675u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DomainName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20480u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_FirmwareVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12289u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_GlobalIdentity: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4096u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ID: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4101u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IPBusEnumerated: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28688u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_InstallState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Installable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IpAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ManufacturerUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8193u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_MetadataVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4100u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ModelUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8196u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceGuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12296u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceLuid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12295u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PhysicalAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12294u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PresentationUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RemoteAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4102u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Removable: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28672u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RootProxy: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4103u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Scopes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4098u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SecureChannel: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28673u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SerialNumber: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12290u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceAddress: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceControlUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16388u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceDescUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16389u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceEventSubUrl: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16390u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceTypes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16386u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ShareName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20482u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Types: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4097u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Upc: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8197u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_XAddrs: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4099u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_IsWifiOnlyDevice: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDefault: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDescription: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemIcon: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemText: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_AltLocationInfo: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24576u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_DevLifeTime: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24577u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_NetworkInterface: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24578u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AssocState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AuthType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigError: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigMethods: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigState: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConnType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DevicePasswordId: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_EncryptType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_OSVersion: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RegistrarType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RequestType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RfBand: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_VendorExtension: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_Version: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Comment: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_DisplayType: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_LocalName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Provider: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_RemoteName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Scope: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Type: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Usage: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 4u32 };
pub const PNPXAssociation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const PNPXPairingHandler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8a27942_ade7_4085_aa6e_4fadc7ada1ef);
pub const PNPX_DEVICECATEGORY_CAMERA: &str = "Cameras";
pub const PNPX_DEVICECATEGORY_COMPUTER: &str = "Computers";
pub const PNPX_DEVICECATEGORY_DISPLAYS: &str = "Displays";
pub const PNPX_DEVICECATEGORY_FAX: &str = "FAX";
pub const PNPX_DEVICECATEGORY_GAMING_DEVICE: &str = "Gaming";
pub const PNPX_DEVICECATEGORY_HOME_AUTOMATION_SYSTEM: &str = "HomeAutomation";
pub const PNPX_DEVICECATEGORY_HOME_SECURITY_SYSTEM: &str = "HomeSecurity";
pub const PNPX_DEVICECATEGORY_INPUTDEVICE: &str = "Input";
pub const PNPX_DEVICECATEGORY_MFP: &str = "MFP";
pub const PNPX_DEVICECATEGORY_MULTIMEDIA_DEVICE: &str = "MediaDevices";
pub const PNPX_DEVICECATEGORY_NETWORK_INFRASTRUCTURE: &str = "NetworkInfrastructure";
pub const PNPX_DEVICECATEGORY_OTHER: &str = "Other";
pub const PNPX_DEVICECATEGORY_PRINTER: &str = "Printers";
pub const PNPX_DEVICECATEGORY_SCANNER: &str = "Scanners";
pub const PNPX_DEVICECATEGORY_STORAGE: &str = "Storage";
pub const PNPX_DEVICECATEGORY_TELEPHONE: &str = "Phones";
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
pub const PNP_CONSTRAINTVALUE_NOTIFICATIONSONLY: &str = "TRUE";
pub const PNP_CONSTRAINTVALUE_NOTPRESENT: &str = "TRUE";
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEFUNCTIONDISPLAYOBJECTS: &str = "DeviceFunctionDisplayObjects";
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEINTERFACES: &str = "DeviceInterfaces";
pub const PROVIDERDDO_QUERYCONSTRAINT_ONLYCONNECTEDDEVICES: &str = "OnlyConnectedDevices";
pub const PROVIDERPNP_QUERYCONSTRAINT_INTERFACECLASS: &str = "InterfaceClass";
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTIFICATIONSONLY: &str = "NotifyOnly";
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTPRESENT: &str = "NotPresent";
pub const PROVIDERSSDP_QUERYCONSTRAINT_CUSTOMXMLPROPERTY: &str = "CustomXmlProperty";
pub const PROVIDERSSDP_QUERYCONSTRAINT_TYPE: &str = "Type";
pub const PROVIDERWNET_QUERYCONSTRAINT_PROPERTIES: &str = "Properties";
pub const PROVIDERWNET_QUERYCONSTRAINT_RESOURCETYPE: &str = "ResourceType";
pub const PROVIDERWNET_QUERYCONSTRAINT_TYPE: &str = "Type";
pub const PROVIDERWSD_QUERYCONSTRAINT_DIRECTEDADDRESS: &str = "RemoteAddress";
pub const PROVIDERWSD_QUERYCONSTRAINT_SCOPE: &str = "Scope";
pub const PROVIDERWSD_QUERYCONSTRAINT_SECURITY_REQUIREMENTS: &str = "SecurityRequirements";
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERTHASH_FOR_SERVER_AUTH: &str = "SSLServerAuthCertHash";
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERT_FOR_CLIENT_AUTH: &str = "SSLClientAuthCert";
pub const PROVIDERWSD_QUERYCONSTRAINT_TYPE: &str = "Type";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PropertyConstraint(pub i32);
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
impl ::core::marker::Copy for PropertyConstraint {}
impl ::core::clone::Clone for PropertyConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyConstraint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PropertyConstraint {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyConstraint").field(&self.0).finish()
    }
}
pub const PropertyStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4796550_df61_448b_9193_13fc1341b163);
pub const PropertyStoreCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedd36029_d753_4862_aa5b_5bccad2a4d29);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QueryCategoryType(pub i32);
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
impl ::core::marker::Copy for QueryCategoryType {}
impl ::core::clone::Clone for QueryCategoryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QueryCategoryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for QueryCategoryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for QueryCategoryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryCategoryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QueryUpdateAction(pub i32);
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
impl ::core::marker::Copy for QueryUpdateAction {}
impl ::core::clone::Clone for QueryUpdateAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QueryUpdateAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for QueryUpdateAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for QueryUpdateAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryUpdateAction").field(&self.0).finish()
    }
}
pub const SID_DeviceDisplayStatusManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf59aa553_8309_46ca_9736_1ac3c62d6031);
pub const SID_EnumDeviceFunction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13e0e9e2_c3fa_4e3c_906e_64502fa4dc95);
pub const SID_EnumInterface: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40eab0b9_4d7f_4b53_a334_1581dd9041f4);
pub const SID_FDPairingHandler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x383b69fa_5486_49da_91f5_d63c24c8e9d0);
pub const SID_FunctionDiscoveryProviderRefresh: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b4cbdc9_31c4_40d4_a62d_772aa174ed52);
pub const SID_PNPXAssociation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const SID_PNPXPropertyStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa86530b1_542f_439f_b71c_b0756b13677a);
pub const SID_PNPXServiceCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x439e80ee_a217_4712_9fa6_deabd9c2a727);
pub const SID_PnpProvider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8101368e_cabb_4426_acff_96c410812000);
pub const SID_UPnPActivator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d0d66eb_cf74_4164_b52f_08344672dd46);
pub const SID_UninstallDeviceFunction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc920566e_5671_4496_8025_bf0b89bd44cd);
pub const SID_UnpairProvider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89a502fc_857b_4698_a0b7_027192002f9e);
pub const SSDP_CONSTRAINTVALUE_TYPE_ALL: &str = "ssdp:all";
pub const SSDP_CONSTRAINTVALUE_TYPE_DEVICE_PREFIX: &str = "urn:schemas-upnp-org:device:";
pub const SSDP_CONSTRAINTVALUE_TYPE_ROOT: &str = "upnp:rootdevice";
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_PREFIX: &str = "urn:schemas-upnp-org:service:";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemVisibilityFlags(pub i32);
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
impl ::core::marker::Copy for SystemVisibilityFlags {}
impl ::core::clone::Clone for SystemVisibilityFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemVisibilityFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemVisibilityFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemVisibilityFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemVisibilityFlags").field(&self.0).finish()
    }
}
pub const WNET_CONSTRAINTVALUE_PROPERTIES_ALL: &str = "All";
pub const WNET_CONSTRAINTVALUE_PROPERTIES_LIMITED: &str = "Limited";
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISK: &str = "Disk";
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISKORPRINTER: &str = "DiskOrPrinter";
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_PRINTER: &str = "Printer";
pub const WNET_CONSTRAINTVALUE_TYPE_ALL: &str = "All";
pub const WNET_CONSTRAINTVALUE_TYPE_DOMAIN: &str = "Domain";
pub const WNET_CONSTRAINTVALUE_TYPE_SERVER: &str = "Server";
pub const WSD_CONSTRAINTVALUE_NO_TRUST_VERIFICATION: &str = "3";
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL: &str = "1";
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL_AND_COMPACTSIGNATURE: &str = "2";
