#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDummyMBNUCMExt {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IDummyMBNUCMExt {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 65535, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionID: unsafe extern "system" fn(this: *mut *mut Self, connectionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, connectionmode: MBN_CONNECTION_MODE, strprofile: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConnectionState: unsafe extern "system" fn(this: *mut *mut Self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConnectionState: usize,
    pub GetVoiceCallState: unsafe extern "system" fn(this: *mut *mut Self, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows_sys::core::HRESULT,
    pub GetActivationNetworkError: unsafe extern "system" fn(this: *mut *mut Self, networkerror: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8205, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionContext {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProvisionedContexts: unsafe extern "system" fn(this: *mut *mut Self, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProvisionedContexts: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProvisionedContext: unsafe extern "system" fn(this: *mut *mut Self, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProvisionedContext: usize,
}
impl ::windows_sys::core::Interface for IMbnConnectionContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8203, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionContextEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnProvisionedContextListChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetProvisionedContextComplete: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionContextEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8204, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConnectComplete: unsafe extern "system" fn(this: *mut *mut Self, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnDisconnectComplete: unsafe extern "system" fn(this: *mut *mut Self, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnConnectStateChange: unsafe extern "system" fn(this: *mut *mut Self, newconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnVoiceCallStateChange: unsafe extern "system" fn(this: *mut *mut Self, newconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8206, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, connectionid: ::windows_sys::core::PCWSTR, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnections: unsafe extern "system" fn(this: *mut *mut Self, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnections: usize,
}
impl ::windows_sys::core::Interface for IMbnConnectionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8221, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConnectionArrival: unsafe extern "system" fn(this: *mut *mut Self, newconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnConnectionRemoval: unsafe extern "system" fn(this: *mut *mut Self, oldconnection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8222, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionProfile {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProfileXmlData: unsafe extern "system" fn(this: *mut *mut Self, profiledata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProfileXmlData: usize,
    pub UpdateProfile: unsafe extern "system" fn(this: *mut *mut Self, strprofile: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionProfile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8208, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionProfileEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnProfileUpdate: unsafe extern "system" fn(this: *mut *mut Self, newprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionProfileEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8209, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionProfileManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnectionProfiles: usize,
    pub GetConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void, profilename: ::windows_sys::core::PCWSTR, connectionprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, xmlprofile: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionProfileManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8207, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnConnectionProfileManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConnectionProfileArrival: unsafe extern "system" fn(this: *mut *mut Self, newconnectionprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnConnectionProfileRemoval: unsafe extern "system" fn(this: *mut *mut Self, oldconnectionprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnConnectionProfileManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8223, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnDeviceService {
    pub base__: ::windows_sys::core::IUnknown,
    pub QuerySupportedCommands: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CloseCommandSession: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCommand: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCommand: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryCommand: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryCommand: usize,
    pub OpenDataSession: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CloseDataSession: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteData: unsafe extern "system" fn(this: *mut *mut Self, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceServiceID: unsafe extern "system" fn(this: *mut *mut Self, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceServiceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCommandSessionOpen: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCommandSessionOpen: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDataSessionOpen: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDataSessionOpen: usize,
}
impl ::windows_sys::core::Interface for IMbnDeviceService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3015416433, data2: 56432, data3: 19433, data4: [164, 218, 120, 134, 174, 139, 25, 27] };
}
#[repr(C)]
pub struct IMbnDeviceServiceStateEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSessionsStateChange: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSessionsStateChange: usize,
}
impl ::windows_sys::core::Interface for IMbnDeviceServiceStateEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1564471702, data2: 35310, data3: 18904, data4: [139, 96, 51, 255, 221, 255, 197, 141] };
}
#[repr(C)]
pub struct IMbnDeviceServicesContext {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateDeviceServices: unsafe extern "system" fn(this: *mut *mut Self, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateDeviceServices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceService: unsafe extern "system" fn(this: *mut *mut Self, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceService: usize,
    pub MaxCommandSize: unsafe extern "system" fn(this: *mut *mut Self, maxcommandsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxDataSize: unsafe extern "system" fn(this: *mut *mut Self, maxdatasize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnDeviceServicesContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4233806663, data2: 5522, data3: 16488, data4: [128, 187, 106, 87, 88, 1, 80, 216] };
}
#[repr(C)]
pub struct IMbnDeviceServicesEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuerySupportedCommandsComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuerySupportedCommandsComplete: usize,
    pub OnOpenCommandSessionComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    pub OnCloseCommandSessionComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetCommandComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueryCommandComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueryCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    pub OnOpenDataSessionComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    pub OnCloseDataSessionComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    pub OnWriteDataComplete: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, status: ::windows_sys::core::HRESULT, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReadData: unsafe extern "system" fn(this: *mut *mut Self, deviceservice: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReadData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInterfaceStateChange: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInterfaceStateChange: usize,
}
impl ::windows_sys::core::Interface for IMbnDeviceServicesEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 177212441, data2: 26660, data3: 20119, data4: [183, 110, 207, 35, 157, 12, 166, 66] };
}
#[repr(C)]
pub struct IMbnDeviceServicesManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeviceServicesContext: unsafe extern "system" fn(this: *mut *mut Self, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeviceServicesContext: usize,
}
impl ::windows_sys::core::Interface for IMbnDeviceServicesManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 547512920, data2: 26641, data3: 17528, data4: [172, 29, 19, 50, 78, 69, 228, 28] };
}
#[repr(C)]
pub struct IMbnInterface {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInterfaceCapability: unsafe extern "system" fn(this: *mut *mut Self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInterfaceCapability: usize,
    pub GetSubscriberInformation: unsafe extern "system" fn(this: *mut *mut Self, subscriberinformation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(this: *mut *mut Self, readystate: *mut MBN_READY_STATE) -> ::windows_sys::core::HRESULT,
    pub InEmergencyMode: unsafe extern "system" fn(this: *mut *mut Self, emergencymode: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHomeProvider: unsafe extern "system" fn(this: *mut *mut Self, homeprovider: *mut MBN_PROVIDER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHomeProvider: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(this: *mut *mut Self, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPreferredProviders: unsafe extern "system" fn(this: *mut *mut Self, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(this: *mut *mut Self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    pub ScanNetwork: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut *mut Self, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnInterface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8193, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnInterfaceEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnInterfaceCapabilityAvailable: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSubscriberInformationChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnReadyStateChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnEmergencyModeChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnHomeProviderAvailable: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetPreferredProvidersComplete: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnInterfaceEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8194, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnInterfaceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInterface: unsafe extern "system" fn(this: *mut *mut Self, interfaceid: ::windows_sys::core::PCWSTR, mbninterface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInterfaces: unsafe extern "system" fn(this: *mut *mut Self, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInterfaces: usize,
}
impl ::windows_sys::core::Interface for IMbnInterfaceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8219, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnInterfaceManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnInterfaceArrival: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnInterfaceRemoval: unsafe extern "system" fn(this: *mut *mut Self, oldinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnInterfaceManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8220, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnMultiCarrier {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHomeProvider: unsafe extern "system" fn(this: *mut *mut Self, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHomeProvider: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(this: *mut *mut Self, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(this: *mut *mut Self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSupportedCellularClasses: unsafe extern "system" fn(this: *mut *mut Self, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSupportedCellularClasses: usize,
    pub GetCurrentCellularClass: unsafe extern "system" fn(this: *mut *mut Self, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows_sys::core::HRESULT,
    pub ScanNetwork: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnMultiCarrier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8224, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnMultiCarrierEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnSetHomeProviderComplete: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnCurrentCellularClassChange: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnInterfaceCapabilityChange: unsafe extern "system" fn(this: *mut *mut Self, mbninterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnMultiCarrierEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3705526966, data2: 8225, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnPin {
    pub base__: ::windows_sys::core::IUnknown,
    pub PinType: unsafe extern "system" fn(this: *mut *mut Self, pintype: *mut MBN_PIN_TYPE) -> ::windows_sys::core::HRESULT,
    pub PinFormat: unsafe extern "system" fn(this: *mut *mut Self, pinformat: *mut MBN_PIN_FORMAT) -> ::windows_sys::core::HRESULT,
    pub PinLengthMin: unsafe extern "system" fn(this: *mut *mut Self, pinlengthmin: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PinLengthMax: unsafe extern "system" fn(this: *mut *mut Self, pinlengthmax: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PinMode: unsafe extern "system" fn(this: *mut *mut Self, pinmode: *mut MBN_PIN_MODE) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Enter: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Change: unsafe extern "system" fn(this: *mut *mut Self, pin: ::windows_sys::core::PCWSTR, newpin: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unblock: unsafe extern "system" fn(this: *mut *mut Self, puk: ::windows_sys::core::PCWSTR, newpin: ::windows_sys::core::PCWSTR, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPinManager: unsafe extern "system" fn(this: *mut *mut Self, pinmanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnPin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8199, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnPinEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnEnableComplete: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnDisableComplete: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnEnterComplete: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnChangeComplete: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnUnblockComplete: unsafe extern "system" fn(this: *mut *mut Self, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnPinEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8200, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnPinManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPinList: unsafe extern "system" fn(this: *mut *mut Self, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPinList: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut *mut Self, pintype: MBN_PIN_TYPE, pin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPinState: unsafe extern "system" fn(this: *mut *mut Self, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnPinManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8197, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnPinManagerEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnPinListAvailable: unsafe extern "system" fn(this: *mut *mut Self, pinmanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnGetPinStateComplete: unsafe extern "system" fn(this: *mut *mut Self, pinmanager: *mut ::core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnPinManagerEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8198, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnRadio {
    pub base__: ::windows_sys::core::IUnknown,
    pub SoftwareRadioState: unsafe extern "system" fn(this: *mut *mut Self, softwareradiostate: *mut MBN_RADIO) -> ::windows_sys::core::HRESULT,
    pub HardwareRadioState: unsafe extern "system" fn(this: *mut *mut Self, hardwareradiostate: *mut MBN_RADIO) -> ::windows_sys::core::HRESULT,
    pub SetSoftwareRadioState: unsafe extern "system" fn(this: *mut *mut Self, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnRadio {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3704408758, data2: 8223, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnRadioEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnRadioStateChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetSoftwareRadioStateComplete: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnRadioEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3705526966, data2: 8223, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnRegistration {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRegisterState: unsafe extern "system" fn(this: *mut *mut Self, registerstate: *mut MBN_REGISTER_STATE) -> ::windows_sys::core::HRESULT,
    pub GetRegisterMode: unsafe extern "system" fn(this: *mut *mut Self, registermode: *mut MBN_REGISTER_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProviderID: unsafe extern "system" fn(this: *mut *mut Self, providerid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProviderID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProviderName: unsafe extern "system" fn(this: *mut *mut Self, providername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRoamingText: unsafe extern "system" fn(this: *mut *mut Self, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRoamingText: usize,
    pub GetAvailableDataClasses: unsafe extern "system" fn(this: *mut *mut Self, availabledataclasses: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut *mut Self, currentdataclass: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRegistrationNetworkError: unsafe extern "system" fn(this: *mut *mut Self, registrationnetworkerror: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPacketAttachNetworkError: unsafe extern "system" fn(this: *mut *mut Self, packetattachnetworkerror: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetRegisterMode: unsafe extern "system" fn(this: *mut *mut Self, registermode: MBN_REGISTER_MODE, providerid: ::windows_sys::core::PCWSTR, dataclass: u32, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8201, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnRegistrationEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnRegisterModeAvailable: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnRegisterStateChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnPacketServiceStateChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetRegisterModeComplete: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnRegistrationEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8202, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnServiceActivation {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
}
impl ::windows_sys::core::Interface for IMbnServiceActivation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8215, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnServiceActivationEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivationComplete: unsafe extern "system" fn(this: *mut *mut Self, serviceactivation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows_sys::core::HRESULT, networkerror: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivationComplete: usize,
}
impl ::windows_sys::core::Interface for IMbnServiceActivationEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8216, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSignal {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSignalStrength: unsafe extern "system" fn(this: *mut *mut Self, signalstrength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSignalError: unsafe extern "system" fn(this: *mut *mut Self, signalerror: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnSignal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8195, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSignalEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnSignalStateChange: unsafe extern "system" fn(this: *mut *mut Self, newinterface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnSignalEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8196, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSms {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSmsConfiguration: unsafe extern "system" fn(this: *mut *mut Self, smsconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSmsConfiguration: unsafe extern "system" fn(this: *mut *mut Self, smsconfiguration: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SmsSendPdu: unsafe extern "system" fn(this: *mut *mut Self, pdudata: ::windows_sys::core::PCWSTR, size: u8, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdma: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdma: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdmaPdu: unsafe extern "system" fn(this: *mut *mut Self, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdmaPdu: usize,
    pub SmsRead: unsafe extern "system" fn(this: *mut *mut Self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SmsDelete: unsafe extern "system" fn(this: *mut *mut Self, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSmsStatus: unsafe extern "system" fn(this: *mut *mut Self, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnSms {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8213, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSmsConfiguration {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceCenterAddress: unsafe extern "system" fn(this: *mut *mut Self, scaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceCenterAddress: usize,
    pub SetServiceCenterAddress: unsafe extern "system" fn(this: *mut *mut Self, scaddress: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub MaxMessageIndex: unsafe extern "system" fn(this: *mut *mut Self, index: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CdmaShortMsgSize: unsafe extern "system" fn(this: *mut *mut Self, shortmsgsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SmsFormat: unsafe extern "system" fn(this: *mut *mut Self, smsformat: *mut MBN_SMS_FORMAT) -> ::windows_sys::core::HRESULT,
    pub SetSmsFormat: unsafe extern "system" fn(this: *mut *mut Self, smsformat: MBN_SMS_FORMAT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnSmsConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8210, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSmsEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnSmsConfigurationChange: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnSetSmsConfigurationComplete: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnSmsSendComplete: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsReadComplete: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsReadComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsNewClass0Message: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsNewClass0Message: usize,
    pub OnSmsDeleteComplete: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnSmsStatusChange: unsafe extern "system" fn(this: *mut *mut Self, sms: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMbnSmsEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8214, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSmsReadMsgPdu {
    pub base__: ::windows_sys::core::IUnknown,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, index: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut MBN_MSG_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PduData: unsafe extern "system" fn(this: *mut *mut Self, pdudata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PduData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
impl ::windows_sys::core::Interface for IMbnSmsReadMsgPdu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8211, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSmsReadMsgTextCdma {
    pub base__: ::windows_sys::core::IUnknown,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, index: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut MBN_MSG_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, address: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Address: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Timestamp: usize,
    pub EncodingID: unsafe extern "system" fn(this: *mut *mut Self, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows_sys::core::HRESULT,
    pub LanguageID: unsafe extern "system" fn(this: *mut *mut Self, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows_sys::core::HRESULT,
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut *mut Self, sizeincharacters: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
impl ::windows_sys::core::Interface for IMbnSmsReadMsgTextCdma {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8212, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnSubscriberInformation {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriberID: unsafe extern "system" fn(this: *mut *mut Self, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriberID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimIccID: unsafe extern "system" fn(this: *mut *mut Self, simiccid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimIccID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TelephoneNumbers: usize,
}
impl ::windows_sys::core::Interface for IMbnSubscriberInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1168034883, data2: 48373, data3: 4572, data4: [168, 168, 0, 19, 33, 241, 64, 95] };
}
#[repr(C)]
pub struct IMbnVendorSpecificEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(this: *mut *mut Self, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetVendorSpecificComplete: unsafe extern "system" fn(this: *mut *mut Self, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetVendorSpecificComplete: usize,
}
impl ::windows_sys::core::Interface for IMbnVendorSpecificEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8218, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[repr(C)]
pub struct IMbnVendorSpecificOperation {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub SetVendorSpecific: unsafe extern "system" fn(this: *mut *mut Self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetVendorSpecific: usize,
}
impl ::windows_sys::core::Interface for IMbnVendorSpecificOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3703290550, data2: 8217, data3: 19387, data4: [170, 238, 51, 142, 54, 138, 246, 250] };
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_ACTIVATION_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_AUTH_PROTOCOL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_BAND_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = 256i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = 512i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = 2048i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = 4096i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = 8192i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = 16384i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = 32768i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = 65536i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = 131072i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = -2147483648i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CELLULAR_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_COMPRESSION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONNECTION_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: super::super::Foundation::BSTR,
    pub userName: super::super::Foundation::BSTR,
    pub password: super::super::Foundation::BSTR,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONTEXT_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = 100i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONTEXT_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CTRL_CAPS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DATA_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = 65536i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = 131072i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = 262144i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = 524288i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = 1048576i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = 2097152i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = 4194304i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = -2147483648i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: super::super::Foundation::BSTR,
    pub dataWriteSupported: i16,
    pub dataReadSupported: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DEVICE_SERVICES_INTERFACE_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DEVICE_SERVICE_SESSIONS_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: super::super::Foundation::BSTR,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: super::super::Foundation::BSTR,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: super::super::Foundation::BSTR,
    pub manufacturer: super::super::Foundation::BSTR,
    pub model: super::super::Foundation::BSTR,
    pub firmwareInfo: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_INTERFACE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_INTERFACE_CAPS_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 18i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_MSG_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl ::core::marker::Copy for MBN_PIN_INFO {}
impl ::core::clone::Clone for MBN_PIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER {
    pub providerID: super::super::Foundation::BSTR,
    pub providerState: u32,
    pub providerName: super::super::Foundation::BSTR,
    pub dataClass: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_PROVIDER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PROVIDER_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = 20i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PROVIDER_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_RADIO = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_OFF: MBN_RADIO = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_ON: MBN_RADIO = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_READY_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTER_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTER_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTRATION_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SIGNAL_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CAPS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CDMA_ENCODING = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CDMA_LANG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_FILTER {}
impl ::core::clone::Clone for MBN_SMS_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_FLAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_STATUS_FLAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_STATUS_INFO {}
impl ::core::clone::Clone for MBN_SMS_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_VOICE_CALL_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_VOICE_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = 3i32;
pub const MbnConnectionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597404, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
pub const MbnConnectionProfileManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597402, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
pub const MbnDeviceServicesManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 577362595, data2: 10911, data3: 16741, data4: [165, 1, 206, 0, 166, 247, 167, 91] };
pub const MbnInterfaceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597403, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type WWAEXT_SMS_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = 160i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl ::core::marker::Copy for __DummyPinType__ {}
impl ::core::clone::Clone for __DummyPinType__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
impl ::core::marker::Copy for __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __mbnapi_ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
