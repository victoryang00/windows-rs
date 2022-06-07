#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManager(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerC(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerExA(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerExW(i_pwszhost: ::windows_sys::core::PCWSTR, i_pwsztmname: ::windows_sys::core::PCWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type APPLICATIONTYPE = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type AUTHENTICATION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl ::core::marker::Copy for BOID {}
impl ::core::clone::Clone for BOID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_MSDtcTransaction: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 972609387, data2: 2344, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
pub const CLSID_MSDtcTransactionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1528343393, data2: 2333, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER = ::core::option::Option<unsafe extern "system" fn(pszhost: ::windows_sys::core::PCSTR, psztmname: ::windows_sys::core::PCSTR, rid: *const ::windows_sys::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = ::core::option::Option<unsafe extern "system" fn(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = ::core::option::Option<unsafe extern "system" fn(i_pwszhost: ::windows_sys::core::PCWSTR, i_pwsztmname: ::windows_sys::core::PCWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_INSTALL_CLIENT = ::core::option::Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_STATUS_ = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTING: DTC_STATUS_ = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTED: DTC_STATUS_ = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = 6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = 7i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_FAILED: DTC_STATUS_ = 9i32;
#[repr(C)]
pub struct IDtcLuConfigure {
    pub base__: ::windows_sys::core::IUnknown,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, puclupair: *const u8, cblupair: u32) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, puclupair: *const u8, cblupair: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuConfigure {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789536, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecovery {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IDtcLuRecovery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2888534738, data2: 55024, data3: 4560, data4: [179, 134, 0, 160, 201, 8, 51, 101] };
}
#[repr(C)]
pub struct IDtcLuRecoveryFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789538, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtc {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWork: unsafe extern "system" fn(this: *mut *mut Self, pwork: *mut _DtcLu_LocalRecovery_Work, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryInitiatedByDtc {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789540, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork {
    pub base__: ::windows_sys::core::IUnknown,
    pub HandleCheckLuStatus: unsafe extern "system" fn(this: *mut *mut Self, lrecoveryseqnum: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryInitiatedByDtcStatusWork {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789542, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLogNameSizes: unsafe extern "system" fn(this: *mut *mut Self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut *mut Self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(this: *mut *mut Self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_sys::core::HRESULT,
    pub HandleTheirXlnResponse: unsafe extern "system" fn(this: *mut *mut Self, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut _DtcLu_Xln_Confirmation) -> ::windows_sys::core::HRESULT,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(this: *mut *mut Self, error: _DtcLu_Xln_Error) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckForCompareStates: unsafe extern "system" fn(this: *mut *mut Self, fcomparestates: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckForCompareStates: usize,
    pub GetOurTransIdSize: unsafe extern "system" fn(this: *mut *mut Self, pcbourtransid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOurCompareStates: unsafe extern "system" fn(this: *mut *mut Self, pourtransid: *mut u8, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_sys::core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(this: *mut *mut Self, comparestate: _DtcLu_CompareState, pconfirmation: *mut _DtcLu_CompareStates_Confirmation) -> ::windows_sys::core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut *mut Self, error: _DtcLu_CompareStates_Error) -> ::windows_sys::core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetRecoverySeqNum: unsafe extern "system" fn(this: *mut *mut Self, plrecoveryseqnum: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(this: *mut *mut Self, lnewrecoveryseqnum: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryInitiatedByDtcTransWork {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789541, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByLu {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(this: *mut *mut Self, ppwork: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryInitiatedByLu {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789544, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRecoveryInitiatedByLuWork {
    pub base__: ::windows_sys::core::IUnknown,
    pub HandleTheirXln: unsafe extern "system" fn(this: *mut *mut Self, lrecoveryseqnum: i32, xln: _DtcLu_Xln, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut _DtcLu_Xln_Response) -> ::windows_sys::core::HRESULT,
    pub GetOurLogNameSize: unsafe extern "system" fn(this: *mut *mut Self, pcbourlogname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOurXln: unsafe extern "system" fn(this: *mut *mut Self, pxln: *mut _DtcLu_Xln, pourlogname: *mut u8, pdwprotocol: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HandleConfirmationOfOurXln: unsafe extern "system" fn(this: *mut *mut Self, confirmation: _DtcLu_Xln_Confirmation) -> ::windows_sys::core::HRESULT,
    pub HandleTheirCompareStates: unsafe extern "system" fn(this: *mut *mut Self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: _DtcLu_CompareState, presponse: *mut _DtcLu_CompareStates_Response, pcomparestate: *mut _DtcLu_CompareState) -> ::windows_sys::core::HRESULT,
    pub HandleConfirmationOfOurCompareStates: unsafe extern "system" fn(this: *mut *mut Self, confirmation: _DtcLu_CompareStates_Confirmation) -> ::windows_sys::core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(this: *mut *mut Self, error: _DtcLu_CompareStates_Error) -> ::windows_sys::core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRecoveryInitiatedByLuWork {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2888534737, data2: 55024, data3: 4560, data4: [179, 134, 0, 160, 201, 8, 51, 101] };
}
#[repr(C)]
pub struct IDtcLuRmEnlistment {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut *mut Self, fconversationlost: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRmEnlistment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789545, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRmEnlistmentFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, puclupair: *mut u8, cblupair: u32, pitransaction: *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut ::core::ffi::c_void, pprmenlistment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRmEnlistmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789553, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuRmEnlistmentSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub AckUnplug: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuRmEnlistmentSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789552, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuSubordinateDtc {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Unplug: unsafe extern "system" fn(this: *mut *mut Self, fconversationlost: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unplug: usize,
    pub BackedOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Prepare: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuSubordinateDtc {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789555, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuSubordinateDtcFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut ::core::ffi::c_void, ppsubordinatedtc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuSubordinateDtcFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789557, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcLuSubordinateDtcSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub AckUnplug: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TmDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BackOut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Committed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Forget: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcLuSubordinateDtcSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1093789556, data2: 6890, data3: 4560, data4: [148, 75, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAnyNetworkAccess: unsafe extern "system" fn(this: *mut *mut Self, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAnyNetworkAccess: unsafe extern "system" fn(this: *mut *mut Self, banynetworkaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAnyNetworkAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut *mut Self, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(this: *mut *mut Self, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkAdministrationAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut *mut Self, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(this: *mut *mut Self, bnetworktransactionaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTransactionAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkClientAccess: unsafe extern "system" fn(this: *mut *mut Self, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkClientAccess: unsafe extern "system" fn(this: *mut *mut Self, bnetworkclientaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkClientAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkTIPAccess: unsafe extern "system" fn(this: *mut *mut Self, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkTIPAccess: unsafe extern "system" fn(this: *mut *mut Self, bnetworktipaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkTIPAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXAAccess: unsafe extern "system" fn(this: *mut *mut Self, pbxaaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXAAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXAAccess: unsafe extern "system" fn(this: *mut *mut Self, bxaaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXAAccess: usize,
    pub RestartDtcService: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcNetworkAccessConfig {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2543305053, data2: 42024, data3: 17041, data4: [135, 182, 9, 149, 3, 26, 103, 141] };
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig2 {
    pub base__: IDtcNetworkAccessConfig,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkInboundAccess: unsafe extern "system" fn(this: *mut *mut Self, pbinbound: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut *mut Self, pboutbound: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNetworkOutboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkInboundAccess: unsafe extern "system" fn(this: *mut *mut Self, binbound: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkInboundAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(this: *mut *mut Self, boutbound: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNetworkOutboundAccess: usize,
    pub GetAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, pauthlevel: *mut AUTHENTICATION_LEVEL) -> ::windows_sys::core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, authlevel: AUTHENTICATION_LEVEL) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcNetworkAccessConfig2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2812936507, data2: 60285, data3: 20290, data4: [180, 28, 178, 222, 192, 154, 224, 52] };
}
#[repr(C)]
pub struct IDtcNetworkAccessConfig3 {
    pub base__: IDtcNetworkAccessConfig2,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLUAccess: unsafe extern "system" fn(this: *mut *mut Self, pbluaccess: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLUAccess: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLUAccess: unsafe extern "system" fn(this: *mut *mut Self, bluaccess: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLUAccess: usize,
}
impl ::windows_sys::core::Interface for IDtcNetworkAccessConfig3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1994700019, data2: 11429, data3: 18027, data4: [137, 213, 253, 33, 142, 231, 91, 73] };
}
#[repr(C)]
pub struct IDtcToXaHelper {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, i_fdorecovery: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut *mut Self, pitransaction: *mut ::core::ffi::c_void, pguidbqual: *const ::windows_sys::core::GUID, pxid: *mut xid_t) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
}
impl ::windows_sys::core::Interface for IDtcToXaHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2844136977, data2: 12362, data3: 4561, data4: [152, 19, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcToXaHelperFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pszdsn: ::windows_sys::core::PCSTR, pszclientdllname: ::windows_sys::core::PCSTR, pguidrm: *mut ::windows_sys::core::GUID, ppxahelper: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcToXaHelperFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2844136976, data2: 12362, data3: 4561, data4: [152, 19, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IDtcToXaHelperSinglePipe {
    pub base__: ::windows_sys::core::IUnknown,
    pub XARMCreate: unsafe extern "system" fn(this: *mut *mut Self, pszdsn: ::windows_sys::core::PCSTR, pszclientdll: ::windows_sys::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConvertTridToXID: unsafe extern "system" fn(this: *mut *mut Self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConvertTridToXID: usize,
    pub EnlistWithRM: unsafe extern "system" fn(this: *mut *mut Self, dwrmcookie: u32, i_pitransaction: *mut ::core::ffi::c_void, i_pitransres: *mut ::core::ffi::c_void, o_ppitransenslitment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseRMCookie: unsafe extern "system" fn(this: *mut *mut Self, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseRMCookie: usize,
}
impl ::windows_sys::core::Interface for IDtcToXaHelperSinglePipe {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1206733169, data2: 21427, data3: 4561, data4: [187, 185, 0, 192, 79, 214, 88, 246] };
}
#[repr(C)]
pub struct IDtcToXaMapper {
    pub base__: ::windows_sys::core::IUnknown,
    pub RequestNewResourceManager: unsafe extern "system" fn(this: *mut *mut Self, pszdsn: ::windows_sys::core::PCSTR, pszclientdllname: ::windows_sys::core::PCSTR, pdwrmcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateTridToXid: unsafe extern "system" fn(this: *mut *mut Self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut xid_t) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateTridToXid: usize,
    pub EnlistResourceManager: unsafe extern "system" fn(this: *mut *mut Self, dwrmcookie: u32, pdwitransaction: *const u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseResourceManager: unsafe extern "system" fn(this: *mut *mut Self, dwrmcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDtcToXaMapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1694477280, data2: 31977, data3: 4560, data4: [140, 230, 0, 192, 79, 220, 135, 126] };
}
#[repr(C)]
pub struct IGetDispenser {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDispenser: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetDispenser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3258762096, data2: 34799, data3: 4558, data4: [128, 129, 0, 128, 199, 88, 82, 126] };
}
#[repr(C)]
pub struct IKernelTransaction {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandle: unsafe extern "system" fn(this: *mut *mut Self, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandle: usize,
}
impl ::windows_sys::core::Interface for IKernelTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2034399787, data2: 63637, data3: 16608, data4: [190, 121, 181, 125, 200, 46, 210, 49] };
}
#[repr(C)]
pub struct ILastResourceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub TransactionCommitted: unsafe extern "system" fn(this: *mut *mut Self, pprepinfo: *const u8, cbprepinfo: u32) -> ::windows_sys::core::HRESULT,
    pub RecoveryDone: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILastResourceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1301695188, data2: 23347, data3: 4563, data4: [138, 145, 0, 192, 79, 121, 235, 109] };
}
#[repr(C)]
pub struct IPrepareInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut *mut Self, pcbprepinfo: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut *mut Self, pprepinfo: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrepareInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2160574416, data2: 34798, data3: 4558, data4: [128, 129, 0, 128, 199, 88, 82, 126] };
}
#[repr(C)]
pub struct IPrepareInfo2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPrepareInfoSize: unsafe extern "system" fn(this: *mut *mut Self, pcbprepinfo: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPrepareInfo: unsafe extern "system" fn(this: *mut *mut Self, cbprepareinfo: u32, pprepinfo: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrepareInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1605051719, data2: 38777, data3: 4561, data4: [184, 134, 0, 192, 79, 185, 97, 138] };
}
#[repr(C)]
pub struct IRMHelper {
    pub base__: ::windows_sys::core::IUnknown,
    pub RMCount: unsafe extern "system" fn(this: *mut *mut Self, dwctotalnumberofrms: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RMInfo: unsafe extern "system" fn(this: *mut *mut Self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: ::windows_sys::core::PCSTR, pszclosestring: ::windows_sys::core::PCSTR, guidrmrecovery: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RMInfo: usize,
}
impl ::windows_sys::core::Interface for IRMHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885233873, data2: 62781, data3: 4559, data4: [166, 13, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IResourceManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub Enlist: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void, pres: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reenlist: unsafe extern "system" fn(this: *mut *mut Self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_sys::core::HRESULT,
    pub ReenlistmentComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDistributedTransactionManager: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 326376737, data2: 34795, data3: 4558, data4: [128, 129, 0, 128, 199, 88, 82, 126] };
}
#[repr(C)]
pub struct IResourceManager2 {
    pub base__: IResourceManager,
    #[cfg(feature = "Win32_Foundation")]
    pub Enlist2: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void, presasync: *mut ::core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut xid_t, ppenlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enlist2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Reenlist2: unsafe extern "system" fn(this: *mut *mut Self, pxid: *const xid_t, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Reenlist2: usize,
}
impl ::windows_sys::core::Interface for IResourceManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3510027930, data2: 63305, data3: 4561, data4: [143, 71, 0, 192, 79, 142, 229, 125] };
}
#[repr(C)]
pub struct IResourceManagerFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pguidrm: *const ::windows_sys::core::GUID, pszrmname: ::windows_sys::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, ppresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManagerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 326376736, data2: 34795, data3: 4558, data4: [128, 129, 0, 128, 199, 88, 82, 126] };
}
#[repr(C)]
pub struct IResourceManagerFactory2 {
    pub base__: IResourceManagerFactory,
    pub CreateEx: unsafe extern "system" fn(this: *mut *mut Self, pguidrm: *const ::windows_sys::core::GUID, pszrmname: ::windows_sys::core::PCSTR, piresmgrsink: *mut ::core::ffi::c_void, riidrequested: *const ::windows_sys::core::GUID, ppvresmgr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManagerFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1798741025, data2: 64466, data3: 4561, data4: [143, 71, 0, 192, 79, 142, 229, 125] };
}
#[repr(C)]
pub struct IResourceManagerRejoinable {
    pub base__: IResourceManager2,
    pub Rejoin: unsafe extern "system" fn(this: *mut *mut Self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManagerRejoinable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1869473312, data2: 46559, data3: 20286, data4: [156, 250, 200, 174, 189, 5, 23, 43] };
}
#[repr(C)]
pub struct IResourceManagerSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub TMDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IResourceManagerSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 223752577, data2: 57083, data3: 4558, data4: [174, 209, 0, 170, 0, 81, 226, 196] };
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type ISOFLAG = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = 12i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = 10i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = 15i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_READONLY: ISOFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type ISOLATIONLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = -1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = 65536i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = 1048576i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = 1048576i32;
#[repr(C)]
pub struct ITipHelper {
    pub base__: ::windows_sys::core::IUnknown,
    pub Pull: unsafe extern "system" fn(this: *mut *mut Self, i_psztxurl: *const u8, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PullAsync: unsafe extern "system" fn(this: *mut *mut Self, i_psztxurl: *const u8, i_ptippullsink: *mut ::core::ffi::c_void, o_ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalTmUrl: unsafe extern "system" fn(this: *mut *mut Self, o_ppszlocaltmurl: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITipHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399471313, data2: 47813, data3: 4561, data4: [177, 191, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct ITipPullSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub PullComplete: unsafe extern "system" fn(this: *mut *mut Self, i_hrpull: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITipPullSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399471314, data2: 47813, data3: 4561, data4: [177, 191, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct ITipTransaction {
    pub base__: ::windows_sys::core::IUnknown,
    pub Push: unsafe extern "system" fn(this: *mut *mut Self, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(this: *mut *mut Self, o_ppszlocaltxurl: *mut ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITipTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399471312, data2: 47813, data3: 4561, data4: [177, 191, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct ITmNodeName {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetNodeNameSize: unsafe extern "system" fn(this: *mut *mut Self, pcbnodenamesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(this: *mut *mut Self, cbnodenamebuffersize: u32, pnodenamebuffer: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITmNodeName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 807882632, data2: 28388, data3: 18254, data4: [155, 149, 120, 7, 188, 158, 248, 207] };
}
#[repr(C)]
pub struct ITransaction {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Commit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Abort: usize,
    pub GetTransactionInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut XACTTRANSINFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 263278724, data2: 44865, data3: 4558, data4: [189, 43, 32, 76, 79, 79, 80, 32] };
}
#[repr(C)]
pub struct ITransaction2 {
    pub base__: ITransactionCloner,
    pub GetTransactionInfo2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut XACTTRANSINFO) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransaction2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 872551752, data2: 101, data3: 4563, data4: [186, 193, 0, 192, 79, 121, 123, 226] };
}
#[repr(C)]
pub struct ITransactionCloner {
    pub base__: ITransaction,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(this: *mut *mut Self, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionCloner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 40200528, data2: 8530, data3: 4560, data4: [148, 76, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct ITransactionDispenser {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOptionsObject: unsafe extern "system" fn(this: *mut *mut Self, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(this: *mut *mut Self, punkouter: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut ::core::ffi::c_void, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionDispenser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 980081121, data2: 9145, data3: 4559, data4: [173, 96, 0, 170, 0, 167, 76, 205] };
}
#[repr(C)]
pub struct ITransactionEnlistmentAsync {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRequestDone: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, pmk: *mut ::core::ffi::c_void, pboidreason: *const BOID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionEnlistmentAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 263278721, data2: 44865, data3: 4558, data4: [189, 43, 32, 76, 79, 79, 80, 32] };
}
#[repr(C)]
pub struct ITransactionExport {
    pub base__: ::windows_sys::core::IUnknown,
    pub Export: unsafe extern "system" fn(this: *mut *mut Self, punktransaction: *mut ::core::ffi::c_void, pcbtransactioncookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTransactionCookie: unsafe extern "system" fn(this: *mut *mut Self, punktransaction: *mut ::core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionExport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21101989, data2: 36800, data3: 4558, data4: [189, 24, 32, 76, 79, 79, 80, 32] };
}
#[repr(C)]
pub struct ITransactionExportFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRemoteClassId: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionExportFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3788479315, data2: 34629, data3: 4558, data4: [169, 186, 0, 170, 0, 108, 55, 6] };
}
#[repr(C)]
pub struct ITransactionImport {
    pub base__: ::windows_sys::core::IUnknown,
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const ::windows_sys::core::GUID, ppvtransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionImport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3788479322, data2: 34629, data3: 4558, data4: [169, 186, 0, 170, 0, 108, 55, 6] };
}
#[repr(C)]
pub struct ITransactionImportWhereabouts {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWhereaboutsSize: unsafe extern "system" fn(this: *mut *mut Self, pcbwhereabouts: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetWhereabouts: unsafe extern "system" fn(this: *mut *mut Self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionImportWhereabouts {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21101988, data2: 36800, data3: 4558, data4: [189, 24, 32, 76, 79, 79, 80, 32] };
}
#[repr(C)]
pub struct ITransactionLastEnlistmentAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub TransactionOutcome: unsafe extern "system" fn(this: *mut *mut Self, xactstat: XACTSTAT, pboidreason: *const BOID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionLastEnlistmentAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3358315827, data2: 23344, data3: 4563, data4: [138, 145, 0, 192, 79, 121, 235, 109] };
}
#[repr(C)]
pub struct ITransactionLastResourceAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub DelegateCommit: unsafe extern "system" fn(this: *mut *mut Self, grfrm: u32) -> ::windows_sys::core::HRESULT,
    pub ForgetRequest: unsafe extern "system" fn(this: *mut *mut Self, pnewuow: *const BOID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionLastResourceAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3358315826, data2: 23344, data3: 4563, data4: [138, 145, 0, 192, 79, 121, 235, 109] };
}
#[repr(C)]
pub struct ITransactionOptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetOptions: unsafe extern "system" fn(this: *mut *mut Self, poptions: *const XACTOPT) -> ::windows_sys::core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut *mut Self, poptions: *mut XACTOPT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 980081120, data2: 9145, data3: 4559, data4: [173, 96, 0, 170, 0, 167, 76, 205] };
}
#[repr(C)]
pub struct ITransactionOutcomeEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Committed: unsafe extern "system" fn(this: *mut *mut Self, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Committed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Aborted: unsafe extern "system" fn(this: *mut *mut Self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Aborted: usize,
    pub HeuristicDecision: unsafe extern "system" fn(this: *mut *mut Self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Indoubt: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionOutcomeEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 980081122, data2: 9145, data3: 4559, data4: [173, 96, 0, 170, 0, 167, 76, 205] };
}
#[repr(C)]
pub struct ITransactionPhase0EnlistmentAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut *mut Self, ppitransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionPhase0EnlistmentAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195491041, data2: 43348, data3: 4561, data4: [143, 136, 0, 96, 8, 149, 231, 213] };
}
#[repr(C)]
pub struct ITransactionPhase0Factory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pphase0notify: *mut ::core::ffi::c_void, ppphase0enlistment: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionPhase0Factory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195491040, data2: 43348, data3: 4561, data4: [143, 136, 0, 96, 8, 149, 231, 213] };
}
#[repr(C)]
pub struct ITransactionPhase0NotifyAsync {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Phase0Request: unsafe extern "system" fn(this: *mut *mut Self, fabortinghint: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Phase0Request: usize,
    pub EnlistCompleted: unsafe extern "system" fn(this: *mut *mut Self, status: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionPhase0NotifyAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4010285065, data2: 3190, data3: 4562, data4: [135, 166, 0, 192, 79, 153, 15, 52] };
}
#[repr(C)]
pub struct ITransactionReceiver {
    pub base__: ::windows_sys::core::IUnknown,
    pub UnmarshalPropagationToken: unsafe extern "system" fn(this: *mut *mut Self, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReturnTokenSize: unsafe extern "system" fn(this: *mut *mut Self, pcbreturntoken: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MarshalReturnToken: unsafe extern "system" fn(this: *mut *mut Self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionReceiver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1496399363, data2: 45932, data3: 4559, data4: [165, 57, 0, 170, 0, 104, 135, 195] };
}
#[repr(C)]
pub struct ITransactionReceiverFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, ppreceiver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionReceiverFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1496399362, data2: 45932, data3: 4559, data4: [165, 57, 0, 170, 0, 104, 135, 195] };
}
#[repr(C)]
pub struct ITransactionResource {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut *mut Self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut *mut Self, grfrm: u32, pnewuow: *const BOID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut *mut Self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3999266739, data2: 17778, data3: 4560, data4: [148, 82, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct ITransactionResourceAsync {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareRequest: unsafe extern "system" fn(this: *mut *mut Self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareRequest: usize,
    pub CommitRequest: unsafe extern "system" fn(this: *mut *mut Self, grfrm: u32, pnewuow: *const BOID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AbortRequest: unsafe extern "system" fn(this: *mut *mut Self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionResourceAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1776906736, data2: 9166, data3: 4559, data4: [173, 96, 0, 170, 0, 167, 76, 205] };
}
#[repr(C)]
pub struct ITransactionTransmitter {
    pub base__: ::windows_sys::core::IUnknown,
    pub Set: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPropagationTokenSize: unsafe extern "system" fn(this: *mut *mut Self, pcbtoken: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MarshalPropagationToken: unsafe extern "system" fn(this: *mut *mut Self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnmarshalReturnToken: unsafe extern "system" fn(this: *mut *mut Self, cbreturntoken: u32, rgbreturntoken: *const u8) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionTransmitter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1496399361, data2: 45932, data3: 4559, data4: [165, 57, 0, 170, 0, 104, 135, 195] };
}
#[repr(C)]
pub struct ITransactionTransmitterFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, pptransmitter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionTransmitterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1496399360, data2: 45932, data3: 4559, data4: [165, 57, 0, 170, 0, 104, 135, 195] };
}
#[repr(C)]
pub struct ITransactionVoterBallotAsync2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub VoteRequestDone: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT, pboidreason: *const BOID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionVoterBallotAsync2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1412642668, data2: 16717, data3: 4563, data4: [178, 6, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct ITransactionVoterFactory2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void, pvoternotify: *mut ::core::ffi::c_void, ppvoterballot: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionVoterFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1412642666, data2: 16717, data3: 4563, data4: [178, 6, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct ITransactionVoterNotifyAsync2 {
    pub base__: ITransactionOutcomeEvents,
    pub VoteRequest: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITransactionVoterNotifyAsync2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1412642667, data2: 16717, data3: 4563, data4: [178, 6, 0, 192, 79, 194, 243, 239] };
}
#[repr(C)]
pub struct IXAConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, clsidhelperdll: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXAConfig {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3366380449, data2: 39564, data3: 4559, data4: [163, 8, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IXAObtainRMInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub ObtainRMInfo: unsafe extern "system" fn(this: *mut *mut Self, pirmhelper: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXAObtainRMInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885233874, data2: 62781, data3: 4559, data4: [166, 13, 0, 160, 201, 5, 65, 110] };
}
#[repr(C)]
pub struct IXATransLookup {
    pub base__: ::windows_sys::core::IUnknown,
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXATransLookup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4088525105, data2: 61146, data3: 4558, data4: [174, 212, 0, 170, 0, 81, 226, 196] };
}
#[repr(C)]
pub struct IXATransLookup2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, pxid: *const xid_t, pptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lookup: usize,
}
impl ::windows_sys::core::Interface for IXATransLookup2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3206102149, data2: 3354, data3: 17040, data4: [184, 143, 210, 203, 136, 115, 209, 231] };
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXBQUALSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXGTRIDSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const RMNAMESZ: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMENDRSCAN: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_INVAL: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_PROTO: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_TMERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMFAIL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMJOIN: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMIGRATE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMULTIPLE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOFLAGS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOMIGRATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOWAIT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMONEPHASE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMREGISTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMRESUME: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSTARTRSCAN: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUCCESS: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUSPEND: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMUSEASYNC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_JOIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_RESUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type TX_MISC_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = 40i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTCONST = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTHEURISTIC = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl ::core::marker::Copy for XACTOPT {}
impl ::core::clone::Clone for XACTOPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTRM = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_NOREADONLYPREPARES: XACTRM = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTSTAT = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NONE: XACTSTAT = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENNORMAL: XACTSTAT = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENREFUSED: XACTSTAT = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARING: XACTSTAT = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARED: XACTSTAT = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = 32i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTING: XACTSTAT = 64i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = 128i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTING: XACTSTAT = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTED: XACTSTAT = 512i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTED: XACTSTAT = 1024i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = 2048i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = 8192i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = 16384i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = 32768i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = 65536i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_INDOUBT: XACTSTAT = 131072i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_CLOSED: XACTSTAT = 262144i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPEN: XACTSTAT = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NOTPREPARED: XACTSTAT = 524227i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ALL: XACTSTAT = 524287i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XACTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTTC = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_NONE: XACTTC = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASEONE: XACTTC = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASETWO: XACTTC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC: XACTTC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC_PHASEONE: XACTTC = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC: XACTTC = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl ::core::marker::Copy for XACTTRANSINFO {}
impl ::core::clone::Clone for XACTTRANSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACT_DTC_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = -2147168000i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = -2147167999i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = -2147167998i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = -2147167997i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167996i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = -2147167995i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = -2147167994i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = -2147167993i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = -2147167992i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = -2147167991i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = -2147167990i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = -2147167989i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = -2147167988i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = -2147167987i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = -2147167986i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = -2147167985i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = -2147167984i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = -2147167983i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167982i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = -2147167981i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = 315648i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = 315649i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = 65535i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_ASYNC: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_DUPID: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_INVAL: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_NOTA: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_OUTSIDE: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_PROTO: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMERR: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMFAIL: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_CLOSE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_COMPLETE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC: u32 = 4478019u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURCOM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURHAZ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURMIX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURRB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_NOMIGRATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_OPEN_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBBASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBCOMMFAIL: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBDEADLOCK: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBEND: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBINTEGRITY: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBOTHER: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBPROTO: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBROLLBACK: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTIMEOUT: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTRANSIENT: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RDONLY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RETRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_SWITCH_F_DTC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XIDDATASIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareState = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = 6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Confirmation = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Error = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Response = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_LocalRecovery_Work = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_COLD: _DtcLu_Xln = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_WARM: _DtcLu_Xln = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Confirmation = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Error = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Response = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl ::core::marker::Copy for _ProxyConfigParams {}
impl ::core::clone::Clone for _ProxyConfigParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xa_switch_t {
    pub name: [super::super::Foundation::CHAR; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: isize,
    pub xa_close_entry: isize,
    pub xa_start_entry: isize,
    pub xa_end_entry: isize,
    pub xa_rollback_entry: isize,
    pub xa_prepare_entry: isize,
    pub xa_commit_entry: isize,
    pub xa_recover_entry: isize,
    pub xa_forget_entry: isize,
    pub xa_complete_entry: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xa_switch_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xid_t {
    fn clone(&self) -> Self {
        *self
    }
}
