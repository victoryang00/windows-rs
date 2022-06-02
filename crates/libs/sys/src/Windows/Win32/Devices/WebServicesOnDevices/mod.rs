#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceHost(pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, ppdevicehost: *mut *mut *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceHost2(pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut *mut *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceHostAdvanced(pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, pphostaddresses: *const *mut *mut IWSDAddress, dwhostaddresscount: u32, ppdevicehost: *mut *mut *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceProxy(pszdeviceid: ::windows_sys::core::PCWSTR, pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, ppdeviceproxy: *mut *mut *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceProxy2(pszdeviceid: ::windows_sys::core::PCWSTR, pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut *mut *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDeviceProxyAdvanced(pszdeviceid: ::windows_sys::core::PCWSTR, pdeviceaddress: *mut *mut IWSDAddress, pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, ppdeviceproxy: *mut *mut *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDiscoveryProvider(pcontext: *mut *mut IWSDXMLContext, ppprovider: *mut *mut *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDiscoveryProvider2(pcontext: *mut *mut IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut *mut *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDiscoveryPublisher(pcontext: *mut *mut IWSDXMLContext, pppublisher: *mut *mut *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateDiscoveryPublisher2(pcontext: *mut *mut IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut *mut *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateHttpAddress(ppaddress: *mut *mut *mut IWSDHttpAddress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateHttpMessageParameters(pptxparams: *mut *mut *mut IWSDHttpMessageParameters) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateOutboundAttachment(ppattachment: *mut *mut *mut IWSDOutboundAttachment) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateUdpAddress(ppaddress: *mut *mut *mut IWSDUdpAddress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDCreateUdpMessageParameters(pptxparams: *mut *mut *mut IWSDUdpMessageParameters) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDGenerateFault(pszcode: ::windows_sys::core::PCWSTR, pszsubcode: ::windows_sys::core::PCWSTR, pszreason: ::windows_sys::core::PCWSTR, pszdetail: ::windows_sys::core::PCWSTR, pcontext: *mut *mut IWSDXMLContext, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: ::windows_sys::core::PCWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDUriDecode(source: ::windows_sys::core::PCWSTR, cchsource: u32, destout: *mut ::windows_sys::core::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDUriEncode(source: ::windows_sys::core::PCWSTR, cchsource: u32, destout: *mut ::windows_sys::core::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: ::windows_sys::core::PCWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLCreateContext(ppcontext: *mut *mut *mut IWSDXMLContext) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: ::windows_sys::core::PCWSTR, pszname: ::windows_sys::core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
    pub fn WSDXMLGetValueFromAny(psznamespace: ::windows_sys::core::PCWSTR, pszname: ::windows_sys::core::PCWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type DeviceDiscoveryMechanism = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2i32;
#[repr(C)]
pub struct IWSDAddress {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pszbuffer: ::windows_sys::core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Serialize: usize,
    pub Deserialize: unsafe extern "system" fn(this: *mut *mut Self, pszbuffer: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDAsyncCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub AsyncOperationComplete: unsafe extern "system" fn(this: *mut *mut Self, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDAsyncResult {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetCallback: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWaitHandle: unsafe extern "system" fn(this: *mut *mut Self, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWaitHandle: usize,
    pub HasCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(this: *mut *mut Self, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut *mut Self, pevent: *mut WSD_EVENT) -> ::windows_sys::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut *mut Self, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDAttachment {
    pub base__: ::windows_sys::core::IUnknown,
}
#[repr(C)]
pub struct IWSDDeviceHost {
    pub base__: ::windows_sys::core::IUnknown,
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RegisterPortType: unsafe extern "system" fn(this: *mut *mut Self, pporttype: *const WSD_PORT_TYPE) -> ::windows_sys::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut *mut Self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_sys::core::HRESULT,
    pub RegisterService: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RetireService: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub AddDynamicService: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, pszendpointaddress: ::windows_sys::core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveDynamicService: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceDiscoverable: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceDiscoverable: usize,
    pub SignalEvent: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDDeviceHostNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetService: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDDeviceProxy {
    pub base__: ::windows_sys::core::IUnknown,
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pszdeviceid: ::windows_sys::core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows_sys::core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHostMetadata: unsafe extern "system" fn(this: *mut *mut Self, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows_sys::core::HRESULT,
    pub GetThisModelMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows_sys::core::HRESULT,
    pub GetThisDeviceMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows_sys::core::HRESULT,
    pub GetAllMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_sys::core::HRESULT,
    pub GetServiceProxyById: unsafe extern "system" fn(this: *mut *mut Self, pszserviceid: ::windows_sys::core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetServiceProxyByType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut *mut Self, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDEndpointProxy {
    pub base__: ::windows_sys::core::IUnknown,
    pub SendOneWayRequest: unsafe extern "system" fn(this: *mut *mut Self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendTwoWayRequest: unsafe extern "system" fn(this: *mut *mut Self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendTwoWayRequest: usize,
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AbortAsyncOperation: unsafe extern "system" fn(this: *mut *mut Self, pasyncresult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessFault: unsafe extern "system" fn(this: *mut *mut Self, pfault: *const WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut *mut Self, ppszerrorinfo: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetFaultInfo: unsafe extern "system" fn(this: *mut *mut Self, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDEventingStatus {
    pub base__: ::windows_sys::core::IUnknown,
    pub SubscriptionRenewed: unsafe extern "system" fn(this: *mut *mut Self, pszsubscriptionaction: ::windows_sys::core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(this: *mut *mut Self, pszsubscriptionaction: ::windows_sys::core::PCWSTR, hr: ::windows_sys::core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(this: *mut *mut Self, pszsubscriptionaction: ::windows_sys::core::PCWSTR),
}
#[repr(C)]
pub struct IWSDHttpAddress {
    pub base__: IWSDTransportAddress,
    pub GetSecure: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecure: unsafe extern "system" fn(this: *mut *mut Self, fsecure: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecure: usize,
    pub GetPath: unsafe extern "system" fn(this: *mut *mut Self, ppszpath: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, pszpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDHttpAuthParameters {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientAccessToken: unsafe extern "system" fn(this: *mut *mut Self, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(this: *mut *mut Self, pauthtype: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDHttpMessageParameters {
    pub base__: IWSDMessageParameters,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(this: *mut *mut Self, pszheaders: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(this: *mut *mut Self, ppszheaders: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut *mut Self, pszheaders: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(this: *mut *mut Self, ppszheaders: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetID: unsafe extern "system" fn(this: *mut *mut Self, pszid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut *mut Self, ppszid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDInboundAttachment {
    pub base__: IWSDAttachment,
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDMessageParameters {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLocalAddress: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(this: *mut *mut Self, paddress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, paddress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(this: *mut *mut Self, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDMetadataExchange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMetadata: unsafe extern "system" fn(this: *mut *mut Self, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDOutboundAttachment {
    pub base__: IWSDAttachment,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDSSLClientCertificate {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetClientCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMappedAccessToken: unsafe extern "system" fn(this: *mut *mut Self, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMappedAccessToken: usize,
}
#[repr(C)]
pub struct IWSDScopeMatchingRule {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetScopeRule: unsafe extern "system" fn(this: *mut *mut Self, ppszscopematchingrule: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchScopes: unsafe extern "system" fn(this: *mut *mut Self, pszscope1: ::windows_sys::core::PCWSTR, pszscope2: ::windows_sys::core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchScopes: usize,
}
#[repr(C)]
pub struct IWSDServiceMessaging {
    pub base__: ::windows_sys::core::IUnknown,
    pub SendResponse: unsafe extern "system" fn(this: *mut *mut Self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FaultRequest: unsafe extern "system" fn(this: *mut *mut Self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDServiceProxy {
    pub base__: IWSDMetadataExchange,
    pub BeginGetMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(this: *mut *mut Self, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_sys::core::HRESULT,
    pub GetServiceMetadata: unsafe extern "system" fn(this: *mut *mut Self, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows_sys::core::HRESULT,
    pub SubscribeToOperation: unsafe extern "system" fn(this: *mut *mut Self, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub UnsubscribeToOperation: unsafe extern "system" fn(this: *mut *mut Self, poperation: *const WSD_OPERATION) -> ::windows_sys::core::HRESULT,
    pub SetEventingStatusCallback: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(this: *mut *mut Self, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDServiceProxyEventing {
    pub base__: IWSDServiceProxy,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginSubscribeToMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndSubscribeToMultipleOperations: usize,
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenewMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndRenewMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndRenewMultipleOperations: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStatusForMultipleOperations: usize,
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(this: *mut *mut Self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndGetStatusForMultipleOperations: usize,
}
#[repr(C)]
pub struct IWSDSignatureProperty {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSigned: unsafe extern "system" fn(this: *mut *mut Self, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(this: *mut *mut Self, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMessageSignatureTrusted: usize,
    pub GetKeyInfo: unsafe extern "system" fn(this: *mut *mut Self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut *mut Self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(this: *mut *mut Self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDTransportAddress {
    pub base__: IWSDAddress,
    pub GetPort: unsafe extern "system" fn(this: *mut *mut Self, pwport: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut *mut Self, wport: u16) -> ::windows_sys::core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(this: *mut *mut Self, ppszaddress: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTransportAddressEx: unsafe extern "system" fn(this: *mut *mut Self, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTransportAddressEx: usize,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut *mut Self, pszaddress: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDUdpAddress {
    pub base__: IWSDTransportAddress,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub SetSockaddr: unsafe extern "system" fn(this: *mut *mut Self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    SetSockaddr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub GetSockaddr: unsafe extern "system" fn(this: *mut *mut Self, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock")))]
    GetSockaddr: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExclusive: unsafe extern "system" fn(this: *mut *mut Self, fexclusive: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExclusive: usize,
    pub GetExclusive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut *mut Self, messagetype: WSDUdpMessageType) -> ::windows_sys::core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(this: *mut *mut Self, pmessagetype: *mut WSDUdpMessageType) -> ::windows_sys::core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(this: *mut *mut Self, dwttl: u32) -> ::windows_sys::core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(this: *mut *mut Self, pdwttl: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(this: *mut *mut Self, palias: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(this: *mut *mut Self, palias: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDUdpMessageParameters {
    pub base__: IWSDMessageParameters,
    pub SetRetransmitParams: unsafe extern "system" fn(this: *mut *mut Self, pparams: *const WSDUdpRetransmitParams) -> ::windows_sys::core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(this: *mut *mut Self, pparams: *mut WSDUdpRetransmitParams) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDXMLContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddNamespace: unsafe extern "system" fn(this: *mut *mut Self, pszuri: ::windows_sys::core::PCWSTR, pszsuggestedprefix: ::windows_sys::core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_sys::core::HRESULT,
    pub AddNameToNamespace: unsafe extern "system" fn(this: *mut *mut Self, pszuri: ::windows_sys::core::PCWSTR, pszname: ::windows_sys::core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_sys::core::HRESULT,
    pub SetNamespaces: unsafe extern "system" fn(this: *mut *mut Self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_sys::core::HRESULT,
    pub SetTypes: unsafe extern "system" fn(this: *mut *mut Self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDiscoveredService {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetEndpointReference: unsafe extern "system" fn(this: *mut *mut Self, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows_sys::core::HRESULT,
    pub GetTypes: unsafe extern "system" fn(this: *mut *mut Self, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows_sys::core::HRESULT,
    pub GetScopes: unsafe extern "system" fn(this: *mut *mut Self, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows_sys::core::HRESULT,
    pub GetXAddrs: unsafe extern "system" fn(this: *mut *mut Self, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows_sys::core::HRESULT,
    pub GetMetadataVersion: unsafe extern "system" fn(this: *mut *mut Self, pullmetadataversion: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetExtendedDiscoXML: unsafe extern "system" fn(this: *mut *mut Self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub GetProbeResolveTag: unsafe extern "system" fn(this: *mut *mut Self, ppsztag: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(this: *mut *mut Self, ppszremotetransportaddress: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(this: *mut *mut Self, ppszlocaltransportaddress: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pullinstanceid: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDiscoveryProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut *mut Self, dwaddressfamily: u32) -> ::windows_sys::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SearchById: unsafe extern "system" fn(this: *mut *mut Self, pszid: ::windows_sys::core::PCWSTR, psztag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(this: *mut *mut Self, pszaddress: ::windows_sys::core::PCWSTR, psztag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SearchByType: unsafe extern "system" fn(this: *mut *mut Self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows_sys::core::PCWSTR, psztag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDiscoveryProviderNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pservice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, pservice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, psztag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(this: *mut *mut Self, psztag: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDiscoveryPublisher {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAddressFamily: unsafe extern "system" fn(this: *mut *mut Self, dwaddressfamily: u32) -> ::windows_sys::core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(this: *mut *mut Self, psink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Publish: unsafe extern "system" fn(this: *mut *mut Self, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_sys::core::HRESULT,
    pub UnPublish: unsafe extern "system" fn(this: *mut *mut Self, pszid: ::windows_sys::core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub MatchProbe: unsafe extern "system" fn(this: *mut *mut Self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_sys::core::HRESULT,
    pub MatchResolve: unsafe extern "system" fn(this: *mut *mut Self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_sys::core::HRESULT,
    pub PublishEx: unsafe extern "system" fn(this: *mut *mut Self, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub MatchProbeEx: unsafe extern "system" fn(this: *mut *mut Self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub MatchResolveEx: unsafe extern "system" fn(this: *mut *mut Self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_sys::core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_sys::core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut *mut Self, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(this: *mut *mut Self, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetXMLContext: unsafe extern "system" fn(this: *mut *mut Self, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWSDiscoveryPublisherNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub ProbeHandler: unsafe extern "system" fn(this: *mut *mut Self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResolveHandler: unsafe extern "system" fn(this: *mut *mut Self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type PWSD_SOAP_MESSAGE_HANDLER = ::core::option::Option<unsafe extern "system" fn(thisunknown: *mut *mut ::windows_sys::core::IUnknown, event: *mut WSD_EVENT) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_GetStatus {}
impl ::core::clone::Clone for REQUESTBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQUESTBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for REQUESTBODY_Unsubscribe {}
impl ::core::clone::Clone for REQUESTBODY_Unsubscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl ::core::marker::Copy for RESPONSEBODY_GetMetadata {}
impl ::core::clone::Clone for RESPONSEBODY_GetMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_GetStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Renew {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESPONSEBODY_Subscribe {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: ::windows_sys::core::PCWSTR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for RESPONSEBODY_SubscriptionEnd {}
impl ::core::clone::Clone for RESPONSEBODY_SubscriptionEnd {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSDEventType = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_NONE: WSDEventType = 0i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_INCOMING_FAULT: WSDEventType = 2i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSDUdpMessageType = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const ONE_WAY: WSDUdpMessageType = 0i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const TWO_WAY: WSDUdpMessageType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
impl ::core::marker::Copy for WSDUdpRetransmitParams {}
impl ::core::clone::Clone for WSDUdpRetransmitParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_ATTRIBUTE {}
impl ::core::clone::Clone for WSDXML_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl ::core::marker::Copy for WSDXML_ELEMENT {}
impl ::core::clone::Clone for WSDXML_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSDXML_ELEMENT_LIST {}
impl ::core::clone::Clone for WSDXML_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_NAME {}
impl ::core::clone::Clone for WSDXML_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NAMESPACE {
    pub Uri: ::windows_sys::core::PCWSTR,
    pub PreferredPrefix: ::windows_sys::core::PCWSTR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl ::core::marker::Copy for WSDXML_NAMESPACE {}
impl ::core::clone::Clone for WSDXML_NAMESPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
impl WSDXML_NODE {
    pub const ElementType: i32 = 0i32;
    pub const TextType: i32 = 1i32;
}
impl ::core::marker::Copy for WSDXML_NODE {}
impl ::core::clone::Clone for WSDXML_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSDXML_OP = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpNone: WSDXML_OP = 0i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndOfTable: WSDXML_OP = 1i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginElement_: WSDXML_OP = 2i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAnyElement: WSDXML_OP = 3i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndElement: WSDXML_OP = 4i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpElement_: WSDXML_OP = 5i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElement: WSDXML_OP = 6i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyElements: WSDXML_OP = 7i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyText: WSDXML_OP = 8i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAttribute_: WSDXML_OP = 9i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginChoice: WSDXML_OP = 10i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndChoice: WSDXML_OP = 11i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginSequence: WSDXML_OP = 12i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndSequence: WSDXML_OP = 13i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpBeginAll: WSDXML_OP = 14i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpEndAll: WSDXML_OP = 15i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnything: WSDXML_OP = 16i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpAnyNumber: WSDXML_OP = 17i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOneOrMore: WSDXML_OP = 18i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpOptional: WSDXML_OP = 19i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatBool_: WSDXML_OP = 20i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt8_: WSDXML_OP = 21i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt16_: WSDXML_OP = 22i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt32_: WSDXML_OP = 23i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatInt64_: WSDXML_OP = 24i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt8_: WSDXML_OP = 25i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt16_: WSDXML_OP = 26i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt32_: WSDXML_OP = 27i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUInt64_: WSDXML_OP = 28i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUnicodeString_: WSDXML_OP = 29i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDom_: WSDXML_OP = 30i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatStruct_: WSDXML_OP = 31i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUri_: WSDXML_OP = 32i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatUuidUri_: WSDXML_OP = 33i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatName_: WSDXML_OP = 34i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatListInsertTail_: WSDXML_OP = 35i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatType_: WSDXML_OP = 36i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDynamicType_: WSDXML_OP = 37i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatLookupType_: WSDXML_OP = 38i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDuration_: WSDXML_OP = 39i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDateTime_: WSDXML_OP = 40i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatFloat_: WSDXML_OP = 41i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatDouble_: WSDXML_OP = 42i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpProcess_: WSDXML_OP = 43i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpQualifiedAttribute_: WSDXML_OP = 44i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatXMLDeclaration_: WSDXML_OP = 45i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const OpFormatMax: WSDXML_OP = 46i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_PREFIX_MAPPING {}
impl ::core::clone::Clone for WSDXML_PREFIX_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSDXML_TEXT {}
impl ::core::clone::Clone for WSDXML_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSDXML_TYPE {
    pub Uri: ::windows_sys::core::PCWSTR,
    pub Table: *const u8,
}
impl ::core::marker::Copy for WSDXML_TYPE {}
impl ::core::clone::Clone for WSDXML_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: ::windows_sys::core::PCWSTR,
    pub MessageNumber: u64,
}
impl ::core::marker::Copy for WSD_APP_SEQUENCE {}
impl ::core::clone::Clone for WSD_APP_SEQUENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_BYE {}
impl ::core::clone::Clone for WSD_BYE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut *mut *mut IWSDAddress,
    pub dwAddressCount: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_ADDRESSES {}
impl ::core::clone::Clone for WSD_CONFIG_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut ::core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl ::core::marker::Copy for WSD_CONFIG_PARAM {}
impl ::core::clone::Clone for WSD_CONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DATETIME {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: super::super::Foundation::BOOL,
    pub TZIsPositive: super::super::Foundation::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_EVENTING_ADDRESS: &str = "http://*:5357/";
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_HOSTING_ADDRESS: &str = "http://*:5357/";
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: &str = "https://*:5358/";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_DURATION {
    pub isPositive: super::super::Foundation::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: ::windows_sys::core::PCWSTR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::clone::Clone for WSD_ENDPOINT_REFERENCE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENT {
    pub Hr: ::windows_sys::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: ::windows_sys::core::PWSTR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: *mut *mut *mut *mut IWSDMessageParameters,
}
impl ::core::marker::Copy for WSD_EVENT {}
impl ::core::clone::Clone for WSD_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: ::windows_sys::core::PCWSTR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl ::core::marker::Copy for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::clone::Clone for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_EVENTING_EXPIRES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: ::windows_sys::core::PCWSTR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_EVENTING_FILTER_ACTION {}
impl ::core::clone::Clone for WSD_EVENTING_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut ::core::ffi::c_void,
    pub Unknown: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for WSD_HANDLER_CONTEXT {}
impl ::core::clone::Clone for WSD_HANDLER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_HEADER_RELATESTO {}
impl ::core::clone::Clone for WSD_HEADER_RELATESTO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_HELLO {}
impl ::core::clone::Clone for WSD_HELLO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl ::core::marker::Copy for WSD_HOST_METADATA {}
impl ::core::clone::Clone for WSD_HOST_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING {
    pub lang: ::windows_sys::core::PCWSTR,
    pub String: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl ::core::marker::Copy for WSD_LOCALIZED_STRING_LIST {}
impl ::core::clone::Clone for WSD_LOCALIZED_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION {
    pub Dialect: ::windows_sys::core::PCWSTR,
    pub Identifier: ::windows_sys::core::PCWSTR,
    pub Data: *mut ::core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: ::windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION {}
impl ::core::clone::Clone for WSD_METADATA_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl ::core::marker::Copy for WSD_METADATA_SECTION_LIST {}
impl ::core::clone::Clone for WSD_METADATA_SECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
impl ::core::marker::Copy for WSD_NAME_LIST {}
impl ::core::clone::Clone for WSD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl ::core::marker::Copy for WSD_OPERATION {}
impl ::core::clone::Clone for WSD_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl ::core::marker::Copy for WSD_PORT_TYPE {}
impl ::core::clone::Clone for WSD_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE {}
impl ::core::clone::Clone for WSD_PROBE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH {}
impl ::core::clone::Clone for WSD_PROBE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_PROBE_MATCHES {}
impl ::core::clone::Clone for WSD_PROBE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl ::core::marker::Copy for WSD_PROBE_MATCH_LIST {}
impl ::core::clone::Clone for WSD_PROBE_MATCH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSD_PROTOCOL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PARAMETERS {}
impl ::core::clone::Clone for WSD_REFERENCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_REFERENCE_PROPERTIES {}
impl ::core::clone::Clone for WSD_REFERENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: ::windows_sys::core::PCWSTR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RELATIONSHIP_METADATA {}
impl ::core::clone::Clone for WSD_RELATIONSHIP_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE {}
impl ::core::clone::Clone for WSD_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCH {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_RESOLVE_MATCHES {}
impl ::core::clone::Clone for WSD_RESOLVE_MATCHES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SCOPES {
    pub MatchBy: ::windows_sys::core::PCWSTR,
    pub Scopes: *mut WSD_URI_LIST,
}
impl ::core::marker::Copy for WSD_SCOPES {}
impl ::core::clone::Clone for WSD_SCOPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: ::windows_sys::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::super::Security::Cryptography::HCERTSTORE,
    pub hCertIssuerStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::super::Security::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: ::windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl ::core::marker::Copy for WSD_SERVICE_METADATA_LIST {}
impl ::core::clone::Clone for WSD_SERVICE_METADATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: ::windows_sys::core::PCWSTR,
    pub Role: ::windows_sys::core::PCWSTR,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT {}
impl ::core::clone::Clone for WSD_SOAP_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_CODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_REASON {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl ::core::marker::Copy for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::clone::Clone for WSD_SOAP_FAULT_SUBCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_HEADER {
    pub To: ::windows_sys::core::PCWSTR,
    pub Action: ::windows_sys::core::PCWSTR,
    pub MessageID: ::windows_sys::core::PCWSTR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_SOAP_HEADER {}
impl ::core::clone::Clone for WSD_SOAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut ::core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl ::core::marker::Copy for WSD_SOAP_MESSAGE {}
impl ::core::clone::Clone for WSD_SOAP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub type WSD_STUB_FUNCTION = ::core::option::Option<unsafe extern "system" fn(server: *mut *mut ::windows_sys::core::IUnknown, session: *mut *mut IWSDServiceMessaging, event: *mut WSD_EVENT) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: ::windows_sys::core::HRESULT,
    pub eventHandle: super::super::Foundation::HANDLE,
    pub messageParameters: *mut *mut *mut *mut IWSDMessageParameters,
    pub results: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: ::windows_sys::core::PCWSTR,
    pub SerialNumber: ::windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_DEVICE_METADATA {}
impl ::core::clone::Clone for WSD_THIS_DEVICE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: ::windows_sys::core::PCWSTR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: ::windows_sys::core::PCWSTR,
    pub ModelUrl: ::windows_sys::core::PCWSTR,
    pub PresentationUrl: ::windows_sys::core::PCWSTR,
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_THIS_MODEL_METADATA {}
impl ::core::clone::Clone for WSD_THIS_MODEL_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl ::core::marker::Copy for WSD_UNKNOWN_LOOKUP {}
impl ::core::clone::Clone for WSD_UNKNOWN_LOOKUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`*"]
pub struct WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSD_URI_LIST {}
impl ::core::clone::Clone for WSD_URI_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
