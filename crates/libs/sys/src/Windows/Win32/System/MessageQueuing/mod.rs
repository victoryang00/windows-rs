#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type FOREIGN_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = 2i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineIdOfMachineName: unsafe extern "system" fn(this: *mut *mut Self, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineIdOfMachineName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication2 {
    pub base__: IMSMQApplication,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterCertificate: unsafe extern "system" fn(this: *mut *mut Self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineNameOfMachineId: unsafe extern "system" fn(this: *mut *mut Self, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineNameOfMachineId: usize,
    pub MSMQVersionMajor: unsafe extern "system" fn(this: *mut *mut Self, psmsmqversionmajor: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MSMQVersionMinor: unsafe extern "system" fn(this: *mut *mut Self, psmsmqversionminor: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MSMQVersionBuild: unsafe extern "system" fn(this: *mut *mut Self, psmsmqversionbuild: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsDsEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfisdsenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication3 {
    pub base__: IMSMQApplication2,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ActiveQueues: unsafe extern "system" fn(this: *mut *mut Self, pvactivequeues: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ActiveQueues: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrivateQueues: unsafe extern "system" fn(this: *mut *mut Self, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrivateQueues: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DirectoryServiceServer: unsafe extern "system" fn(this: *mut *mut Self, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DirectoryServiceServer: usize,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, pfisconnected: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInAllQueues: unsafe extern "system" fn(this: *mut *mut Self, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInAllQueues: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMachine: unsafe extern "system" fn(this: *mut *mut Self, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMachine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Machine: unsafe extern "system" fn(this: *mut *mut Self, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Machine: usize,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Tidy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCollection {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser2 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser3 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQDestination {
    pub base__: super::Com::IDispatch,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, pfisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IADs: unsafe extern "system" fn(this: *mut *mut Self, ppiads: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IADs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_IADs: unsafe extern "system" fn(this: *mut *mut Self, piads: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_IADs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut *mut Self, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetADsPath: unsafe extern "system" fn(this: *mut *mut Self, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetADsPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut *mut Self, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut *mut Self, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destinations: unsafe extern "system" fn(this: *mut *mut Self, ppdestinations: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Destinations: unsafe extern "system" fn(this: *mut *mut Self, pdestinations: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent2 {
    pub base__: IMSMQEvent,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent3 {
    pub base__: IMSMQEvent2,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQManagement {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Init: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Machine: unsafe extern "system" fn(this: *mut *mut Self, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Machine: usize,
    pub MessageCount: unsafe extern "system" fn(this: *mut *mut Self, plmessagecount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ForeignStatus: unsafe extern "system" fn(this: *mut *mut Self, plforeignstatus: *mut i32) -> ::windows_sys::core::HRESULT,
    pub QueueType: unsafe extern "system" fn(this: *mut *mut Self, plqueuetype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut *mut Self, pfislocal: *mut i16) -> ::windows_sys::core::HRESULT,
    pub TransactionalStatus: unsafe extern "system" fn(this: *mut *mut Self, pltransactionalstatus: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInQueue: unsafe extern "system" fn(this: *mut *mut Self, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInQueue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQMessage {
    pub base__: super::Com::IDispatch,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, plclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut *mut Self, plauthlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut *mut Self, lauthlevel: i32) -> ::windows_sys::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut *mut Self, pldelivery: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut *mut Self, ldelivery: i32) -> ::windows_sys::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut *mut Self, pltrace: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut *mut Self, ltrace: i32) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, plpriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, lpriority: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut *mut Self, lappspecific: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut *mut Self, pcbbody: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pvarbody: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut *mut Self, plack: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut *mut Self, lack: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreachqueue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreachqueue: i32) -> ::windows_sys::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreceive: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreceive: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plhashalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lhashalg: i32) -> ::windows_sys::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plencryptalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lencryptalg: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut *mut Self, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut *mut Self, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut *mut Self, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut *mut Self, plsenderidtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut *mut Self, lsenderidtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut *mut Self, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQMessage2 {
    pub base__: super::Com::IDispatch,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, plclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut *mut Self, plauthlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut *mut Self, lauthlevel: i32) -> ::windows_sys::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut *mut Self, pldelivery: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut *mut Self, ldelivery: i32) -> ::windows_sys::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut *mut Self, pltrace: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut *mut Self, ltrace: i32) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, plpriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, lpriority: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut *mut Self, lappspecific: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut *mut Self, pcbbody: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pvarbody: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut *mut Self, plack: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut *mut Self, lack: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreachqueue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreachqueue: i32) -> ::windows_sys::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreceive: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreceive: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plhashalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lhashalg: i32) -> ::windows_sys::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plencryptalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lencryptalg: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut *mut Self, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut *mut Self, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut *mut Self, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut *mut Self, plsenderidtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut *mut Self, lsenderidtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut *mut Self, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut *mut Self, plsenderversion: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut *mut Self, pvarextension: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut *mut Self, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut *mut Self, pvarsignature: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut *mut Self, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, plauthprovtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, lauthprovtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut *mut Self, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut *mut Self, plmsgclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut *mut Self, lmsgclass: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, pvarxactid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pisfirstinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pislastinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, psreceivedauthenticationlevel: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQMessage3 {
    pub base__: super::Com::IDispatch,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, plclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut *mut Self, plauthlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut *mut Self, lauthlevel: i32) -> ::windows_sys::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut *mut Self, pldelivery: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut *mut Self, ldelivery: i32) -> ::windows_sys::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut *mut Self, pltrace: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut *mut Self, ltrace: i32) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, plpriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, lpriority: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut *mut Self, lappspecific: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut *mut Self, pcbbody: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pvarbody: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut *mut Self, plack: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut *mut Self, lack: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreachqueue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreachqueue: i32) -> ::windows_sys::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreceive: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreceive: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plhashalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lhashalg: i32) -> ::windows_sys::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plencryptalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lencryptalg: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut *mut Self, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut *mut Self, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut *mut Self, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut *mut Self, plsenderidtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut *mut Self, lsenderidtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut *mut Self, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut *mut Self, plsenderversion: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut *mut Self, pvarextension: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut *mut Self, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut *mut Self, pvarsignature: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut *mut Self, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, plauthprovtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, lauthprovtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut *mut Self, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut *mut Self, plmsgclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut *mut Self, lmsgclass: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, pvarxactid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pisfirstinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pislastinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, psreceivedauthenticationlevel: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut *mut Self, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut *mut Self, pdestresponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupId: unsafe extern "system" fn(this: *mut *mut Self, pvarlookupid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut *mut Self, pisfirstinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut *mut Self, pislastinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut *mut Self, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SoapEnvelope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut *mut Self, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompoundMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut *mut Self, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapHeader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapBody: unsafe extern "system" fn(this: *mut *mut Self, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapBody: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQMessage4 {
    pub base__: super::Com::IDispatch,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, plclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut *mut Self, plauthlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut *mut Self, lauthlevel: i32) -> ::windows_sys::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut *mut Self, pldelivery: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut *mut Self, ldelivery: i32) -> ::windows_sys::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut *mut Self, pltrace: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut *mut Self, ltrace: i32) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, plpriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, lpriority: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut *mut Self, plappspecific: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut *mut Self, lappspecific: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut *mut Self, pcbbody: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, pvarbody: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, pvarmsgid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut *mut Self, plack: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut *mut Self, lack: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreachqueue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreachqueue: i32) -> ::windows_sys::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, plmaxtimetoreceive: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut *mut Self, lmaxtimetoreceive: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plhashalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lhashalg: i32) -> ::windows_sys::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, plencryptalg: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, lencryptalg: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut *mut Self, pvarsenttime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut *mut Self, plarrivedtime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvarsendercert: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut *mut Self, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut *mut Self, pvarsenderid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut *mut Self, plsenderidtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut *mut Self, lsenderidtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut *mut Self, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut *mut Self, plsenderversion: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut *mut Self, pvarextension: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut *mut Self, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut *mut Self, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut *mut Self, pvarsignature: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut *mut Self, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, plauthprovtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut *mut Self, lauthprovtype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut *mut Self, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut *mut Self, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut *mut Self, plmsgclass: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut *mut Self, lmsgclass: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, pvarxactid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pisfirstinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pislastinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, psreceivedauthenticationlevel: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut *mut Self, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut *mut Self, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut *mut Self, pdestresponse: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupId: unsafe extern "system" fn(this: *mut *mut Self, pvarlookupid: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut *mut Self, pisauthenticated: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut *mut Self, pisfirstinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut *mut Self, pislastinxact: *mut i16) -> ::windows_sys::core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut *mut Self, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SoapEnvelope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut *mut Self, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompoundMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut *mut Self, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapHeader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapBody: unsafe extern "system" fn(this: *mut *mut Self, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapBody: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQOutgoingQueueManagement {
    pub base__: IMSMQManagement,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, plstate: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NextHops: unsafe extern "system" fn(this: *mut *mut Self, pvnexthops: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NextHops: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EodGetSendInfo: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EodGetSendInfo: usize,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EodResend: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQPrivateDestination {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, pvarhandle: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetHandle: unsafe extern "system" fn(this: *mut *mut Self, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetHandle: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQPrivateEvent {
    pub base__: super::Com::IDispatch,
    pub Hwnd: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedEvent: unsafe extern "system" fn(this: *mut *mut Self, pq: *mut ::core::ffi::c_void, msgcursor: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedErrorEvent: unsafe extern "system" fn(this: *mut *mut Self, pq: *mut ::core::ffi::c_void, hrstatus: ::windows_sys::core::HRESULT, msgcursor: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedErrorEvent: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery2 {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery3 {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery4 {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut *mut Self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue {
    pub base__: super::Com::IDispatch,
    pub Access: unsafe extern "system" fn(this: *mut *mut Self, placcess: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut *mut Self, plsharemode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, plhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut *mut Self, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue2 {
    pub base__: super::Com::IDispatch,
    pub Access: unsafe extern "system" fn(this: *mut *mut Self, placcess: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut *mut Self, plsharemode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, plhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut *mut Self, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue3 {
    pub base__: super::Com::IDispatch,
    pub Access: unsafe extern "system" fn(this: *mut *mut Self, placcess: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut *mut Self, plsharemode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, plhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut *mut Self, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle2: unsafe extern "system" fn(this: *mut *mut Self, pvarhandle: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue4 {
    pub base__: super::Com::IDispatch,
    pub Access: unsafe extern "system" fn(this: *mut *mut Self, placcess: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut *mut Self, plsharemode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut *mut Self, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut *mut Self, plhandle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut *mut Self, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle2: unsafe extern "system" fn(this: *mut *mut Self, pvarhandle: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut *mut Self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut *mut Self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut *mut Self, pisopen: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupIdAllowPeek: unsafe extern "system" fn(this: *mut *mut Self, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupIdAllowPeek: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut *mut Self, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut *mut Self, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut *mut Self, plquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut *mut Self, lquota: i32) -> ::windows_sys::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut *mut Self, plbasepriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut *mut Self, lbasepriority: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut *mut Self, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut *mut Self, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut *mut Self, plauthenticate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, lauthenticate: i32) -> ::windows_sys::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut *mut Self, pljournalquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut *mut Self, ljournalquota: i32) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo2 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut *mut Self, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut *mut Self, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut *mut Self, plquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut *mut Self, lquota: i32) -> ::windows_sys::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut *mut Self, plbasepriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut *mut Self, lbasepriority: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut *mut Self, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut *mut Self, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut *mut Self, plauthenticate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, lauthenticate: i32) -> ::windows_sys::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut *mut Self, pljournalquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut *mut Self, ljournalquota: i32) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo3 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut *mut Self, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut *mut Self, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut *mut Self, plquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut *mut Self, lquota: i32) -> ::windows_sys::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut *mut Self, plbasepriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut *mut Self, lbasepriority: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut *mut Self, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut *mut Self, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut *mut Self, plauthenticate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, lauthenticate: i32) -> ::windows_sys::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut *mut Self, pljournalquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut *mut Self, ljournalquota: i32) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MulticastAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut *mut Self, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo4 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut *mut Self, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut *mut Self, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut *mut Self, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut *mut Self, plprivlevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut *mut Self, lprivlevel: i32) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, pljournal: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut *mut Self, ljournal: i32) -> ::windows_sys::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut *mut Self, plquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut *mut Self, lquota: i32) -> ::windows_sys::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut *mut Self, plbasepriority: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut *mut Self, lbasepriority: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut *mut Self, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut *mut Self, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut *mut Self, plauthenticate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, lauthenticate: i32) -> ::windows_sys::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut *mut Self, pljournalquota: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut *mut Self, ljournalquota: i32) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut *mut Self, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, pvarsecurity: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut *mut Self, pistransactional: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut *mut Self, pisworldreadable: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MulticastAddress: unsafe extern "system" fn(this: *mut *mut Self, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut *mut Self, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos {
    pub base__: super::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos2 {
    pub base__: super::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos3 {
    pub base__: super::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos4 {
    pub base__: super::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueManagement {
    pub base__: IMSMQManagement,
    pub JournalMessageCount: unsafe extern "system" fn(this: *mut *mut Self, pljournalmessagecount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInJournal: unsafe extern "system" fn(this: *mut *mut Self, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInJournal: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EodGetReceiveInfo: unsafe extern "system" fn(this: *mut *mut Self, pvcollection: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EodGetReceiveInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction {
    pub base__: super::Com::IDispatch,
    pub Transaction: unsafe extern "system" fn(this: *mut *mut Self, pltransaction: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Commit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Abort: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction2 {
    pub base__: IMSMQTransaction,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitNew: unsafe extern "system" fn(this: *mut *mut Self, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitNew: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction3 {
    pub base__: IMSMQTransaction2,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ITransaction: unsafe extern "system" fn(this: *mut *mut Self, pvaritransaction: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ITransaction: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser2 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser3 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const LONG_LIVED: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_CONNECT: &str = "CONNECT";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_DISCONNECT: &str = "DISCONNECT";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_TIDY: &str = "TIDY";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_CORRECT_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_FOREIGN_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_INCORRECT_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_LOCAL_LOCATION: &str = "LOCAL";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_REMOTE_LOCATION: &str = "REMOTE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_CONNECTED: &str = "CONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_DISCONNECTED: &str = "DISCONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_DISCONNECTING: &str = "DISCONNECTING";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_LOCAL: &str = "LOCAL CONNECTION";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_LOCKED: &str = "LOCKED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: &str = "NEED VALIDATION";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_NONACTIVE: &str = "INACTIVE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_ONHOLD: &str = "ONHOLD";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_WAITING: &str = "WAITING";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_CONNECTOR: &str = "CONNECTOR";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_MACHINE: &str = "MACHINE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_MULTICAST: &str = "MULTICAST";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_PRIVATE: &str = "PRIVATE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_PUBLIC: &str = "PUBLIC";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_UNKNOWN_TYPE: &str = "UNKNOWN";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MO_MACHINE_TOKEN: &str = "MACHINE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MO_QUEUE_TOKEN: &str = "QUEUE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQACCESS = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_RECEIVE_ACCESS: MQACCESS = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_SEND_ACCESS: MQACCESS = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PEEK_ACCESS: MQACCESS = 32i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ADMIN_ACCESS: MQACCESS = 128i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQAUTHENTICATE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQCALG = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD2: MQCALG = 32769i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD4: MQCALG = 32770i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD5: MQCALG = 32771i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SHA: MQCALG = 32772i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SHA1: MQCALG = 32772i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MAC: MQCALG = 32773i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RSA_SIGN: MQCALG = 9216i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_DSS_SIGN: MQCALG = 8704i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RSA_KEYX: MQCALG = 41984i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_DES: MQCALG = 26113i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RC2: MQCALG = 26114i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RC4: MQCALG = 26625i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SEAL: MQCALG = 26626i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQCERT_REGISTER = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQDEFAULT = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_PRIORITY: MQDEFAULT = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_DELIVERY: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_JOURNAL: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_QUOTA: MQDEFAULT = -1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = -1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQERROR = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR: MQERROR = -1072824319i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTY: MQERROR = -1072824318i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = -1072824317i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = -1072824316i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = -1072824315i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = -1072824314i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = -1072824313i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = -1072824312i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = -1072824311i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = -1072824309i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = -1072824307i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = -1072824304i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = -1072824303i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_DS: MQERROR = -1072824301i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = -1072824300i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = -1072824296i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = -1072824295i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = -1072824294i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = -1072824293i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = -1072824292i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = -1072824291i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = -1072824290i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = -1072824289i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = -1072824288i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = -1072824287i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = -1072824286i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = -1072824285i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = -1072824284i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = -1072824283i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = -1072824282i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = -1072824281i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = -1072824280i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = -1072824278i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = -1072824277i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = -1072824276i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = -1072824275i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = -1072824274i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = -1072824273i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = -1072824272i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = -1072824271i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = -1072824269i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = -1072824267i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = -1072824266i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = -1072824265i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = -1072824264i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = -1072824263i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = -1072824262i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = -1072824261i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = -1072824260i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = -1072824259i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = -1072824258i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = -1072824257i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = -1072824256i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = -1072824255i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_IS_FULL: MQERROR = -1072824254i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_ERROR: MQERROR = -1072824253i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_OWNER: MQERROR = -1072824252i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = -1072824251i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = -1072824250i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = -1072824248i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = -1072824247i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = -1072824246i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = -1072824245i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DTC_CONNECT: MQERROR = -1072824244i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = -1072824242i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = -1072824240i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = -1072824239i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = -1072824235i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_STALE_HANDLE: MQERROR = -1072824234i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = -1072824232i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = -1072824230i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = -1072824229i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = -1072824228i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = -1072824227i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = -1072824226i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = -1072824225i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = -1072824224i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = -1072824223i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = -1072824222i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = -1072824221i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = -1072824220i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = -1072824219i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = -1072824218i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = -1072824217i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = -1072824216i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = -1072824215i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = -1072824214i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = -1072824213i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = -1072824212i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = -1072824211i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = -1072824210i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = -1072824209i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = -1072824209i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = -1072824208i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = -1072824208i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = -1072824207i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = -1072824206i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = -1072824205i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = -1072824204i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = -1072824203i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = -1072824202i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = -1072824201i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = -1072824200i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = -1072824199i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = -1072824198i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = -1072824197i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = -1072824196i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = -1072824195i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = -1072824194i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = -1072824193i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = -1072824192i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = -1072824191i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = -1072824190i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = -1072824189i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = -1072824188i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = -1072824187i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = -1072824186i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = -1072824185i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = -1072824184i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = -1072824183i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = -1072824182i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = -1072824181i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = -1072824180i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = -1072824179i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_GC_NEEDED: MQERROR = -1072824178i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = -1072824177i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = -1072824176i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = -1072824175i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = -1072824174i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = -1072824173i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = -1072824172i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = -1072824171i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = -1072824170i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQJOURNAL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_JOURNAL_NONE: MQJOURNAL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_JOURNAL: MQJOURNAL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMAX = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_Q_NAME_LEN: MQMAX = 124i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = 124i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGACKNOWLEDGEMENT = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = 8i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = 5i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = 12i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = 14i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGAUTHENTICATION = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = 5i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = 9i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGAUTHLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = 8i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGCLASS = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = 16384i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = 32768i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = 32769i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = 32770i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = 32771i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = 32772i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = 32773i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = 32774i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = 32775i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = 32776i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = 32777i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = 32778i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = 32779i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = 32780i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = 49152i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = 49153i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = 49154i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = 49155i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGCURSOR = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_FIRST: MQMSGCURSOR = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CURRENT: MQMSGCURSOR = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NEXT: MQMSGCURSOR = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGDELIVERY = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGIDSIZE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = 20i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = 20i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = 20i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGJOURNAL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_JOURNAL: MQMSGJOURNAL = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGMAX = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = 249i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGPRIVLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGSENDERIDTYPE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQMSGTRACE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_TRACE_NONE: MQMSGTRACE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQPRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MIN_PRIORITY: MQPRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_PRIORITY: MQPRIORITY = 7i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQPRIVLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQSHARE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_DENY_NONE: MQSHARE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQTRANSACTION = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_NO_TRANSACTION: MQTRANSACTION = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XA_TRANSACTION: MQTRANSACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQTRANSACTIONAL = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type MQWARNING = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_PROPERTY: MQWARNING = 1074659329i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = 1074659330i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = 1074659331i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = 1074659332i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = 1074659333i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = 1074659334i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = 1074659337i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = 1074659338i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = 1074659339i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows_sys::core::HRESULT = -1072824164i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows_sys::core::HRESULT = -1072824165i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows_sys::core::HRESULT = -1072824167i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows_sys::core::HRESULT = -1072824166i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MOVE_ACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_OK: ::windows_sys::core::HRESULT = 0i32;
pub const MSMQApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183622, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146827313, data2: 12044, data3: 17384, data4: [146, 78, 230, 5, 44, 220, 73, 63] };
pub const MSMQCoordinatedTransactionDispenser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183618, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQDestination: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3953748760, data2: 8552, data3: 4563, data4: [137, 140, 0, 224, 44, 7, 79, 107] };
pub const MSMQEvent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183610, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 969840382, data2: 62661, data3: 17540, data4: [161, 67, 76, 45, 93, 50, 66, 41] };
pub const MSMQMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183605, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQOutgoingQueueManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 25706524, data2: 9338, data3: 20461, data4: [153, 198, 191, 20, 17, 157, 112, 85] };
pub const MSMQQuery: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183603, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183609, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183612, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueInfos: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183614, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 867618942, data2: 62077, data3: 17146, data4: [178, 215, 191, 130, 225, 30, 147, 116] };
pub const MSMQTransaction: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183616, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQTransactionDispenser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183620, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MSMQ_CONNECTED: &str = "CONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MSMQ_DISCONNECTED: &str = "DISCONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PREQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRGT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRLT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRNE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DELIVERY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_EXTENSION: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_HASH_ALG: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_JOURNAL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LABEL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LOOKUPID: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MSGID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_NAME: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENTTIME: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SIGNATURE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TRACE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_VERSION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACTID: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_BASE: u32 = 5800u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_VERSION: u32 = 5801u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_BASE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_CONNECTION: u32 = 204u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_PATHNAME: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_SITE_ID: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_BASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_INSTANCE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_JOURNAL: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_LABEL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PATHNAME: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_QUOTA: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_TYPE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUERY_SORTASCEND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUERY_SORTDESCEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_EOD_RESEND: &str = "EOD_RESEND";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_PAUSE: &str = "PAUSE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_RESUME: &str = "RESUME";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type QUEUE_STATE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type QUEUE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type RELOPS = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_NOP: RELOPS = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_EQ: RELOPS = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_NEQ: RELOPS = 2i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_LT: RELOPS = 3i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_GT: RELOPS = 4i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_LE: RELOPS = 5i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_GE: RELOPS = 6i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub type XACT_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = 2i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _DMSMQEventEvents {
    pub base__: super::Com::IDispatch,
}
