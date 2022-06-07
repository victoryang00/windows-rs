#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type CONFLICT_RESOLUTION_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type CONSTRAINT_CONFLICT_REASON = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type FILTERING_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type FILTER_COMBINATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = 0i32;
#[repr(C)]
pub struct IAsynchronousDataRetriever {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut *mut Self, pidparameters: *mut ID_PARAMETERS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut *mut Self, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevokeCallback: unsafe extern "system" fn(this: *mut *mut Self, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut *mut Self, ploadchangecontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsynchronousDataRetriever {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680677488, data2: 25066, data3: 19080, data4: [155, 228, 223, 86, 162, 124, 254, 242] };
}
#[repr(C)]
pub struct IChangeConflict {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResolveActionForChange: unsafe extern "system" fn(this: *mut *mut Self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub SetResolveActionForChange: unsafe extern "system" fn(this: *mut *mut Self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub GetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pchangeunit: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub SetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pchangeunit: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChangeConflict {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21938071, data2: 40736, data3: 20346, data4: [189, 212, 37, 151, 156, 119, 192, 2] };
}
#[repr(C)]
pub struct IChangeUnitException {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetItemId: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut *mut Self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChangeUnitException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215477884, data2: 65216, data3: 16417, data4: [153, 238, 240, 229, 52, 143, 42, 95] };
}
#[repr(C)]
pub struct IChangeUnitListFilterInfo {
    pub base__: ISyncFilterInfo,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitIdCount: unsafe extern "system" fn(this: *mut *mut Self, pdwchangeunitidcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut *mut Self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChangeUnitListFilterInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4068701809, data2: 3039, data3: 17402, data4: [181, 2, 35, 35, 117, 251, 80, 194] };
}
#[repr(C)]
pub struct IClockVector {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetClockVectorElements: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClockVectorElementCount: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClockVector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 347219786, data2: 34456, data3: 19654, data4: [147, 51, 248, 155, 209, 212, 123, 196] };
}
#[repr(C)]
pub struct IClockVectorElement {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetReplicaKey: unsafe extern "system" fn(this: *mut *mut Self, pdwreplicakey: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTickCount: unsafe extern "system" fn(this: *mut *mut Self, pulltickcount: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClockVectorElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3877388880, data2: 44536, data3: 18951, data4: [143, 174, 86, 105, 89, 105, 9, 193] };
}
#[repr(C)]
pub struct ICombinedFilterInfo {
    pub base__: ISyncFilterInfo,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut *mut Self, pdwfiltercount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut *mut Self, dwfilterindex: u32, ppifilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilterCombinationType: unsafe extern "system" fn(this: *mut *mut Self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICombinedFilterInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 301588081, data2: 10264, data3: 18297, data4: [178, 172, 66, 212, 80, 86, 95, 69] };
}
#[repr(C)]
pub struct IConstraintConflict {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDestinationProviderOriginalChange: unsafe extern "system" fn(this: *mut *mut Self, pporiginalchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut *mut Self, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDestinationProviderOriginalData: unsafe extern "system" fn(this: *mut *mut Self, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut *mut Self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub SetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut *mut Self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub GetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pchangeunit: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub SetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pchangeunit: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_sys::core::HRESULT,
    pub GetConstraintConflictReason: unsafe extern "system" fn(this: *mut *mut Self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_sys::core::HRESULT,
    pub IsTemporary: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConstraintConflict {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13774894, data2: 7416, data3: 18485, data4: [184, 95, 183, 202, 79, 121, 158, 10] };
}
#[repr(C)]
pub struct IConstructReplicaKeyMap {
    pub base__: ::windows_sys::core::IUnknown,
    pub FindOrAddReplica: unsafe extern "system" fn(this: *mut *mut Self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IConstructReplicaKeyMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3738241392, data2: 60549, data3: 16661, data4: [181, 44, 68, 5, 132, 86, 66, 165] };
}
#[repr(C)]
pub struct ICoreFragment {
    pub base__: ::windows_sys::core::IUnknown,
    pub NextColumn: unsafe extern "system" fn(this: *mut *mut Self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NextRange: unsafe extern "system" fn(this: *mut *mut Self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut *mut Self, pcolumncount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRangeCount: unsafe extern "system" fn(this: *mut *mut Self, prangecount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreFragment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1631267509, data2: 45828, data3: 18393, data4: [156, 49, 206, 108, 84, 64, 26, 21] };
}
#[repr(C)]
pub struct ICoreFragmentInspector {
    pub base__: ::windows_sys::core::IUnknown,
    pub NextCoreFragments: unsafe extern "system" fn(this: *mut *mut Self, requestedcount: u32, ppicorefragments: *mut *mut ::core::ffi::c_void, pfetchedcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreFragmentInspector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4160538109, data2: 44582, data3: 18041, data4: [186, 22, 150, 170, 197, 131, 193, 52] };
}
#[repr(C)]
pub struct ICustomFilterInfo {
    pub base__: ISyncFilterInfo,
    pub GetSyncFilter: unsafe extern "system" fn(this: *mut *mut Self, pisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICustomFilterInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 489905663, data2: 28552, data3: 20045, data4: [145, 168, 163, 243, 81, 207, 212, 115] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ID_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ID_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: super::super::Foundation::BOOL,
    pub cbIdSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ID_PARAMETER_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ID_PARAMETER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDataRetrieverCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub LoadChangeDataComplete: unsafe extern "system" fn(this: *mut *mut Self, punkdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LoadChangeDataError: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataRetrieverCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1907656251, data2: 63849, data3: 18038, data4: [187, 195, 61, 159, 220, 63, 178, 199] };
}
#[repr(C)]
pub struct IEnumChangeUnitExceptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32, ppchangeunitexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumChangeUnitExceptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 812967938, data2: 37657, data3: 17440, data4: [190, 33, 16, 34, 226, 226, 29, 168] };
}
#[repr(C)]
pub struct IEnumClockVector {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, csyncversions: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumClockVector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1381516507, data2: 10295, data3: 18329, data4: [158, 128, 129, 166, 110, 2, 34, 12] };
}
#[repr(C)]
pub struct IEnumFeedClockVector {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, csyncversions: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumFeedClockVector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1427076669, data2: 5226, data3: 18678, data4: [171, 235, 108, 136, 199, 247, 5, 20] };
}
#[repr(C)]
pub struct IEnumItemIds {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumItemIds {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1135230817, data2: 19246, data3: 19296, data4: [131, 223, 177, 16, 211, 225, 72, 241] };
}
#[repr(C)]
pub struct IEnumRangeExceptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32, pprangeexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumRangeExceptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 155468703, data2: 56753, data3: 16758, data4: [183, 3, 4, 111, 242, 42, 35, 134] };
}
#[repr(C)]
pub struct IEnumSingleItemExceptions {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32, ppsingleitemexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cexceptions: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSingleItemExceptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3848484892, data2: 6989, data3: 19558, data4: [151, 150, 200, 111, 172, 205, 205, 64] };
}
#[repr(C)]
pub struct IEnumSyncChangeUnits {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cchanges: u32, ppchangeunit: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cchanges: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSyncChangeUnits {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879441393, data2: 34563, data3: 19565, data4: [171, 26, 77, 188, 162, 207, 249, 127] };
}
#[repr(C)]
pub struct IEnumSyncChanges {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cchanges: u32, ppchange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cchanges: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSyncChanges {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1602666058, data2: 24184, data3: 20018, data4: [172, 28, 194, 79, 210, 35, 239, 133] };
}
#[repr(C)]
pub struct IEnumSyncProviderConfigUIInfos {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cfactories: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSyncProviderConfigUIInfos {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4139656706, data2: 6086, data3: 18008, data4: [162, 215, 104, 237, 51, 48, 246, 65] };
}
#[repr(C)]
pub struct IEnumSyncProviderInfos {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, cinstances: u32, ppsyncproviderinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, cinstances: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumSyncProviderInfos {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2689312848, data2: 24241, data3: 17933, data4: [169, 115, 57, 63, 203, 96, 138, 17] };
}
#[repr(C)]
pub struct IFeedClockVector {
    pub base__: IClockVector,
    pub GetUpdateCount: unsafe extern "system" fn(this: *mut *mut Self, pdwupdatecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNoConflictsSpecified: unsafe extern "system" fn(this: *mut *mut Self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNoConflictsSpecified: usize,
}
impl ::windows_sys::core::Interface for IFeedClockVector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2367527121, data2: 40888, data3: 20169, data4: [165, 83, 84, 221, 146, 78, 15, 103] };
}
#[repr(C)]
pub struct IFeedClockVectorElement {
    pub base__: IClockVectorElement,
    pub GetSyncTime: unsafe extern "system" fn(this: *mut *mut Self, psynctime: *mut SYNC_TIME) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pbflags: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFeedClockVectorElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2752202450, data2: 59771, data3: 16726, data4: [182, 218, 153, 31, 80, 27, 15, 5] };
}
#[repr(C)]
pub struct IFilterKeyMap {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AddFilter: unsafe extern "system" fn(this: *mut *mut Self, pisyncfilter: *mut ::core::ffi::c_void, pdwfilterkey: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(this: *mut *mut Self, dwfilterkey: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFilterKeyMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3390477906, data2: 1990, data3: 18184, data4: [163, 218, 110, 78, 186, 141, 34, 151] };
}
#[repr(C)]
pub struct IFilterRequestCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub RequestFilter: unsafe extern "system" fn(this: *mut *mut Self, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFilterRequestCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195687539, data2: 25440, data3: 17978, data4: [168, 161, 237, 229, 225, 161, 89, 77] };
}
#[repr(C)]
pub struct IFilterTrackingProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub SpecifyTrackedFilters: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddTrackedFilter: unsafe extern "system" fn(this: *mut *mut Self, pfilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFilterTrackingProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1949533120, data2: 64590, data3: 17850, data4: [173, 129, 217, 216, 76, 122, 36, 248] };
}
#[repr(C)]
pub struct IFilterTrackingRequestCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub RequestTrackedFilter: unsafe extern "system" fn(this: *mut *mut Self, pfilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFilterTrackingRequestCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1899800507, data2: 51288, data3: 18036, data4: [180, 182, 17, 34, 67, 101, 135, 169] };
}
#[repr(C)]
pub struct IFilterTrackingSyncChangeBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddFilterChange: unsafe extern "system" fn(this: *mut *mut Self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddFilterChange: usize,
    pub SetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFilterTrackingSyncChangeBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 693118112, data2: 28890, data3: 19544, data4: [136, 60, 206, 42, 251, 48, 141, 11] };
}
#[repr(C)]
pub struct IForgottenKnowledge {
    pub base__: ISyncKnowledge,
    pub ForgetToVersion: unsafe extern "system" fn(this: *mut *mut Self, pknowledge: *mut ::core::ffi::c_void, pversion: *const SYNC_VERSION) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IForgottenKnowledge {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1164840854, data2: 24630, data3: 17707, data4: [159, 157, 188, 196, 180, 168, 93, 178] };
}
#[repr(C)]
pub struct IKnowledgeSyncProvider {
    pub base__: ISyncProvider,
    pub BeginSession: unsafe extern "system" fn(this: *mut *mut Self, role: SYNC_PROVIDER_ROLE, psessionstate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSyncBatchParameters: unsafe extern "system" fn(this: *mut *mut Self, ppsyncknowledge: *mut *mut ::core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeBatch: unsafe extern "system" fn(this: *mut *mut Self, dwbatchsize: u32, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut *mut Self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessChangeBatch: unsafe extern "system" fn(this: *mut *mut Self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_sys::core::HRESULT,
    pub ProcessFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut *mut Self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_sys::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut *mut Self, psessionstate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnowledgeSyncProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1128483401, data2: 36260, data3: 18418, data4: [129, 114, 173, 123, 139, 2, 73, 120] };
}
#[repr(C)]
pub struct ILoadChangeContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSyncChange: unsafe extern "system" fn(this: *mut *mut Self, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecoverableErrorOnChange: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT, perrordata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecoverableErrorOnChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, hrerror: ::windows_sys::core::HRESULT, pchangeunit: *mut ::core::ffi::c_void, perrordata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoadChangeContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1151642314, data2: 60473, data3: 18133, data4: [181, 201, 214, 51, 192, 238, 103, 226] };
}
#[repr(C)]
pub struct IProviderConverter {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pisyncprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProviderConverter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2157671030, data2: 39119, data3: 18775, data4: [147, 165, 14, 189, 211, 221, 223, 253] };
}
#[repr(C)]
pub struct IRangeException {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetClosedRangeStart: unsafe extern "system" fn(this: *mut *mut Self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClosedRangeEnd: unsafe extern "system" fn(this: *mut *mut Self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRangeException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1974372215, data2: 26696, data3: 18935, data4: [149, 108, 163, 169, 47, 80, 150, 232] };
}
#[repr(C)]
pub struct IRecoverableError {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStage: unsafe extern "system" fn(this: *mut *mut Self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_sys::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut *mut Self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_sys::core::HRESULT,
    pub GetChangeWithRecoverableError: unsafe extern "system" fn(this: *mut *mut Self, ppchangewithrecoverableerror: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRecoverableErrorDataForChange: unsafe extern "system" fn(this: *mut *mut Self, phrerror: *mut ::windows_sys::core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRecoverableErrorDataForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pchangeunit: *mut ::core::ffi::c_void, phrerror: *mut ::windows_sys::core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRecoverableError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 257304040, data2: 2683, data3: 17902, data4: [150, 55, 28, 225, 54, 69, 144, 158] };
}
#[repr(C)]
pub struct IRecoverableErrorData {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pcszitemdisplayname: ::windows_sys::core::PCWSTR, pcszerrordescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetItemDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pszitemdisplayname: ::windows_sys::core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut *mut Self, pszerrordescription: ::windows_sys::core::PCWSTR, pccherrordescription: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRecoverableErrorData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011267082, data2: 19325, data3: 19501, data4: [151, 17, 59, 0, 209, 25, 177, 200] };
}
#[repr(C)]
pub struct IRegisteredSyncProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, pguidcontenttype: *const ::windows_sys::core::GUID, pcontextpropertystore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRegisteredSyncProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2436616054, data2: 18369, data3: 16565, data4: [168, 150, 94, 138, 156, 65, 76, 20] };
}
#[repr(C)]
pub struct IReplicaKeyMap {
    pub base__: ::windows_sys::core::IUnknown,
    pub LookupReplicaKey: unsafe extern "system" fn(this: *mut *mut Self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LookupReplicaId: unsafe extern "system" fn(this: *mut *mut Self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReplicaKeyMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 571077884, data2: 64784, data3: 20464, data4: [132, 168, 240, 161, 152, 46, 68, 14] };
}
#[repr(C)]
pub struct IRequestFilteredSync {
    pub base__: ::windows_sys::core::IUnknown,
    pub SpecifyFilter: unsafe extern "system" fn(this: *mut *mut Self, pcallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRequestFilteredSync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 771883396, data2: 27928, data3: 18087, data4: [163, 42, 218, 74, 235, 6, 105, 108] };
}
#[repr(C)]
pub struct ISingleItemException {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetItemId: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISingleItemException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2301606320, data2: 31829, data3: 18968, data4: [147, 22, 253, 244, 73, 86, 155, 100] };
}
#[repr(C)]
pub struct ISupportFilteredSync {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddFilter: unsafe extern "system" fn(this: *mut *mut Self, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISupportFilteredSync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1024626157, data2: 54613, data3: 19981, data4: [191, 75, 251, 33, 58, 138, 147, 2] };
}
#[repr(C)]
pub struct ISupportLastWriteTime {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetItemChangeTime: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitChangeTime: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISupportLastWriteTime {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3940516207, data2: 53437, data3: 17354, data4: [143, 64, 90, 205, 198, 192, 111, 122] };
}
#[repr(C)]
pub struct ISyncCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnProgress: unsafe extern "system" fn(this: *mut *mut Self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_sys::core::HRESULT,
    pub OnChange: unsafe extern "system" fn(this: *mut *mut Self, psyncchange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnConflict: unsafe extern "system" fn(this: *mut *mut Self, pconflict: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnFullEnumerationNeeded: unsafe extern "system" fn(this: *mut *mut Self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_sys::core::HRESULT,
    pub OnRecoverableError: unsafe extern "system" fn(this: *mut *mut Self, precoverableerror: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 93944191, data2: 24281, data3: 18524, data4: [174, 54, 12, 93, 27, 242, 231, 165] };
}
#[repr(C)]
pub struct ISyncCallback2 {
    pub base__: ISyncCallback,
    pub OnChangeApplied: unsafe extern "system" fn(this: *mut *mut Self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_sys::core::HRESULT,
    pub OnChangeFailed: unsafe extern "system" fn(this: *mut *mut Self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncCallback2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1204716719, data2: 29762, data3: 20141, data4: [134, 48, 18, 1, 94, 3, 10, 215] };
}
#[repr(C)]
pub struct ISyncChange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut *mut Self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRootItemId: unsafe extern "system" fn(this: *mut *mut Self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeVersion: unsafe extern "system" fn(this: *mut *mut Self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_sys::core::HRESULT,
    pub GetCreationVersion: unsafe extern "system" fn(this: *mut *mut Self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_sys::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetWorkEstimate: unsafe extern "system" fn(this: *mut *mut Self, pdwwork: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnits: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMadeWithKnowledge: unsafe extern "system" fn(this: *mut *mut Self, ppmadewithknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWorkEstimate: unsafe extern "system" fn(this: *mut *mut Self, dwwork: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2710907883, data2: 3947, data3: 18193, data4: [177, 54, 1, 218, 133, 185, 104, 166] };
}
#[repr(C)]
pub struct ISyncChangeBatch {
    pub base__: ISyncChangeBatchBase,
    pub BeginUnorderedGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndUnorderedGroup: unsafe extern "system" fn(this: *mut *mut Self, pmadewithknowledge: *mut ::core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndUnorderedGroup: usize,
    pub AddLoggedConflict: unsafe extern "system" fn(this: *mut *mut Self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1892044270, data2: 14351, data3: 19502, data4: [143, 112, 49, 197, 91, 213, 249, 179] };
}
#[repr(C)]
pub struct ISyncChangeBatch2 {
    pub base__: ISyncChangeBatch,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut *mut Self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddMergeTombstoneLoggedConflict: unsafe extern "system" fn(this: *mut *mut Self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatch2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 576670259, data2: 62958, data3: 19655, data4: [176, 57, 103, 162, 98, 180, 178, 172] };
}
#[repr(C)]
pub struct ISyncChangeBatchAdvanced {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut *mut Self, ppfilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertFullEnumerationChangeBatchToRegularChangeBatch: unsafe extern "system" fn(this: *mut *mut Self, ppchangebatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetUpperBoundItemId: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBatchLevelKnowledgeShouldBeApplied: unsafe extern "system" fn(this: *mut *mut Self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBatchLevelKnowledgeShouldBeApplied: usize,
}
impl ::windows_sys::core::Interface for ISyncChangeBatchAdvanced {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 253381013, data2: 52168, data3: 16925, data4: [181, 80, 93, 11, 235, 243, 233, 165] };
}
#[repr(C)]
pub struct ISyncChangeBatchBase {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetChangeEnumerator: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsLastBatch: unsafe extern "system" fn(this: *mut *mut Self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsLastBatch: usize,
    pub GetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut *mut Self, pdwworkforbatch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut *mut Self, pdwremainingworkforsession: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BeginOrderedGroup: unsafe extern "system" fn(this: *mut *mut Self, pblowerbound: *const u8) -> ::windows_sys::core::HRESULT,
    pub EndOrderedGroup: unsafe extern "system" fn(this: *mut *mut Self, pbupperbound: *const u8, pmadewithknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddItemMetadataToGroup: unsafe extern "system" fn(this: *mut *mut Self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut *mut Self, ppprerequisteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSourceForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, ppsourceforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetLastBatch: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut *mut Self, dwworkforbatch: u32) -> ::windows_sys::core::HRESULT,
    pub SetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut *mut Self, dwremainingworkforsession: u32) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatchBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1391912596, data2: 27249, data3: 17556, data4: [161, 132, 168, 49, 27, 245, 210, 39] };
}
#[repr(C)]
pub struct ISyncChangeBatchBase2 {
    pub base__: ISyncChangeBatchBase,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut *mut Self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatchBase2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1876646250, data2: 55125, data3: 17796, data4: [189, 12, 192, 194, 58, 84, 143, 191] };
}
#[repr(C)]
pub struct ISyncChangeBatchWithFilterKeyMap {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFilterKeyMap: unsafe extern "system" fn(this: *mut *mut Self, ppifilterkeymap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFilterKeyMap: unsafe extern "system" fn(this: *mut *mut Self, pifilterkeymap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, dwfilterkey: u32, pfilterforgottenknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatchWithFilterKeyMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3726929922, data2: 22125, data3: 17818, data4: [166, 237, 165, 170, 179, 69, 159, 183] };
}
#[repr(C)]
pub struct ISyncChangeBatchWithPrerequisite {
    pub base__: ISyncChangeBatchBase,
    pub SetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pprerequisiteknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBatchWithPrerequisite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 159323070, data2: 23442, data3: 16456, data4: [179, 242, 123, 66, 162, 81, 94, 7] };
}
#[repr(C)]
pub struct ISyncChangeBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddChangeUnitMetadata: unsafe extern "system" fn(this: *mut *mut Self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1458653041, data2: 34423, data3: 18511, data4: [161, 112, 227, 134, 228, 24, 166, 118] };
}
#[repr(C)]
pub struct ISyncChangeUnit {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetItemChange: unsafe extern "system" fn(this: *mut *mut Self, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut *mut Self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitVersion: unsafe extern "system" fn(this: *mut *mut Self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeUnit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1626200266, data2: 29505, data3: 19383, data4: [149, 206, 250, 182, 57, 75, 81, 203] };
}
#[repr(C)]
pub struct ISyncChangeWithFilterKeyMap {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut *mut Self, pdwfiltercount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilterChange: unsafe extern "system" fn(this: *mut *mut Self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilterChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut *mut Self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllChangeUnitsPresentFlag: usize,
    pub GetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeWithFilterKeyMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3219255040, data2: 59517, data3: 17149, data4: [164, 233, 36, 45, 112, 65, 74, 239] };
}
#[repr(C)]
pub struct ISyncChangeWithPrerequisite {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut *mut Self, ppprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut *mut Self, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncChangeWithPrerequisite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654484527, data2: 5513, data3: 18627, data4: [146, 228, 5, 236, 220, 180, 243, 247] };
}
#[repr(C)]
pub struct ISyncConstraintCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnConstraintConflict: unsafe extern "system" fn(this: *mut *mut Self, pconflict: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncConstraintCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2331214910, data2: 30131, data3: 17292, data4: [187, 81, 111, 2, 13, 112, 211, 203] };
}
#[repr(C)]
pub struct ISyncDataConverter {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConvertDataRetrieverFromProviderFormat: unsafe extern "system" fn(this: *mut *mut Self, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertDataRetrieverToProviderFormat: unsafe extern "system" fn(this: *mut *mut Self, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertDataFromProviderFormat: unsafe extern "system" fn(this: *mut *mut Self, pdatacontext: *mut ::core::ffi::c_void, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertDataToProviderFormat: unsafe extern "system" fn(this: *mut *mut Self, pdatacontext: *mut ::core::ffi::c_void, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncDataConverter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1130186849, data2: 26837, data3: 17578, data4: [160, 249, 114, 160, 176, 14, 249, 207] };
}
#[repr(C)]
pub struct ISyncFilter {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsIdentical: unsafe extern "system" fn(this: *mut *mut Self, psyncfilter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 142229269, data2: 4043, data3: 17601, data4: [150, 57, 83, 193, 78, 43, 85, 6] };
}
#[repr(C)]
pub struct ISyncFilterDeserializer {
    pub base__: ::windows_sys::core::IUnknown,
    pub DeserializeSyncFilter: unsafe extern "system" fn(this: *mut *mut Self, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFilterDeserializer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3025894002, data2: 58823, data3: 18110, data4: [156, 130, 119, 184, 177, 93, 171, 138] };
}
#[repr(C)]
pub struct ISyncFilterInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFilterInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2035198712, data2: 16174, data3: 18406, data4: [151, 40, 23, 230, 252, 249, 76, 183] };
}
#[repr(C)]
pub struct ISyncFilterInfo2 {
    pub base__: ISyncFilterInfo,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFilterInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 431199418, data2: 58320, data3: 18060, data4: [147, 77, 50, 25, 104, 178, 171, 52] };
}
#[repr(C)]
pub struct ISyncFullEnumerationChange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFullEnumerationChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2542133437, data2: 48639, data3: 16580, data4: [152, 197, 179, 75, 47, 25, 145, 179] };
}
#[repr(C)]
pub struct ISyncFullEnumerationChangeBatch {
    pub base__: ISyncChangeBatchBase,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut *mut Self, pplearnedknowledgeafterrecoverycomplete: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClosedLowerBoundItemId: unsafe extern "system" fn(this: *mut *mut Self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetClosedUpperBoundItemId: unsafe extern "system" fn(this: *mut *mut Self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFullEnumerationChangeBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016314749, data2: 20292, data3: 20130, data4: [179, 85, 69, 36, 113, 62, 59, 237] };
}
#[repr(C)]
pub struct ISyncFullEnumerationChangeBatch2 {
    pub base__: ISyncFullEnumerationChangeBatch,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut *mut Self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncFullEnumerationChangeBatch2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3764668916, data2: 41477, data3: 19301, data4: [151, 36, 1, 178, 33, 1, 238, 193] };
}
#[repr(C)]
pub struct ISyncKnowledge {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut *mut Self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Serialize: usize,
    pub SetLocalTickCount: unsafe extern "system" fn(this: *mut *mut Self, ulltickcount: u64) -> ::windows_sys::core::HRESULT,
    pub ContainsChange: unsafe extern "system" fn(this: *mut *mut Self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_sys::core::HRESULT,
    pub ContainsChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_sys::core::HRESULT,
    pub GetScopeVector: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetReplicaKeyMap: unsafe extern "system" fn(this: *mut *mut Self, ppreplicakeymap: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppclonedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConvertVersion: unsafe extern "system" fn(this: *mut *mut Self, pknowledgein: *mut ::core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_sys::core::HRESULT,
    pub MapRemoteToLocal: unsafe extern "system" fn(this: *mut *mut Self, premoteknowledge: *mut ::core::ffi::c_void, ppmappedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Union: unsafe extern "system" fn(this: *mut *mut Self, pknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProjectOntoItem: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProjectOntoChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProjectOntoRange: unsafe extern "system" fn(this: *mut *mut Self, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExcludeItem: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8) -> ::windows_sys::core::HRESULT,
    pub ExcludeChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_sys::core::HRESULT,
    pub ContainsKnowledge: unsafe extern "system" fn(this: *mut *mut Self, pknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindMinTickCountForReplica: unsafe extern "system" fn(this: *mut *mut Self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetRangeExceptions: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSingleItemExceptions: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChangeUnitExceptions: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindClockVectorForItem: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindClockVectorForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, pdwversion: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncKnowledge {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1633401683, data2: 51525, data3: 16899, data4: [191, 75, 44, 182, 89, 25, 160, 170] };
}
#[repr(C)]
pub struct ISyncKnowledge2 {
    pub base__: ISyncKnowledge,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut *mut Self, pidparameters: *mut ID_PARAMETERS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub ProjectOntoColumnSet: unsafe extern "system" fn(this: *mut *mut Self, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut *mut Self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetLowestUncontainedId: unsafe extern "system" fn(this: *mut *mut Self, pisyncknowledge: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInspector: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMinimumSupportedVersion: unsafe extern "system" fn(this: *mut *mut Self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_sys::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut *mut Self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ContainsKnowledgeForItem: unsafe extern "system" fn(this: *mut *mut Self, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_sys::core::HRESULT,
    pub ContainsKnowledgeForChangeUnit: unsafe extern "system" fn(this: *mut *mut Self, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_sys::core::HRESULT,
    pub ProjectOntoKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut *mut Self, pprerequisiteknowledge: *mut ::core::ffi::c_void, ptemplateknowledge: *mut ::core::ffi::c_void, ppprojectedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Complement: unsafe extern "system" fn(this: *mut *mut Self, psyncknowledge: *mut ::core::ffi::c_void, ppcomplementedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IntersectsWithKnowledge: unsafe extern "system" fn(this: *mut *mut Self, psyncknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetKnowledgeCookie: unsafe extern "system" fn(this: *mut *mut Self, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CompareToKnowledgeCookie: unsafe extern "system" fn(this: *mut *mut Self, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncKnowledge2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3976912320, data2: 15179, data3: 18081, data4: [154, 69, 69, 102, 29, 33, 20, 200] };
}
#[repr(C)]
pub struct ISyncMergeTombstoneChange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetWinnerItemId: unsafe extern "system" fn(this: *mut *mut Self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncMergeTombstoneChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1858479511, data2: 2307, data3: 18508, data4: [173, 97, 54, 214, 233, 56, 244, 123] };
}
#[repr(C)]
pub struct ISyncProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut *mut Self, pidparameters: *mut ID_PARAMETERS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
}
impl ::windows_sys::core::Interface for ISyncProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2405789782, data2: 11214, data3: 18967, data4: [140, 104, 199, 187, 120, 152, 181, 111] };
}
#[repr(C)]
pub struct ISyncProviderConfigUI {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, pguidcontenttype: *const ::windows_sys::core::GUID, pconfigurationproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetRegisteredProperties: unsafe extern "system" fn(this: *mut *mut Self, ppconfiguiproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetRegisteredProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateAndRegisterNewSyncProvider: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateAndRegisterNewSyncProvider: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ModifySyncProvider: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ModifySyncProvider: usize,
}
impl ::windows_sys::core::Interface for ISyncProviderConfigUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2064057846, data2: 52173, data3: 16497, data4: [171, 5, 59, 220, 54, 77, 74, 12] };
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct ISyncProviderConfigUIInfo {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub GetSyncProviderConfigUI: unsafe extern "system" fn(this: *mut *mut Self, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_sys::core::Interface for ISyncProviderConfigUIInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 557924782, data2: 13271, data3: 19853, data4: [142, 55, 242, 39, 232, 128, 206, 80] };
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct ISyncProviderInfo {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub GetSyncProvider: unsafe extern "system" fn(this: *mut *mut Self, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_sys::core::Interface for ISyncProviderInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 518075870, data2: 34980, data3: 17668, data4: [176, 208, 247, 146, 13, 126, 91, 166] };
}
#[repr(C)]
pub struct ISyncProviderRegistration {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateSyncProviderConfigUIRegistrationInstance: unsafe extern "system" fn(this: *mut *mut Self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateSyncProviderConfigUIRegistrationInstance: usize,
    pub UnregisterSyncProviderConfigUI: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EnumerateSyncProviderConfigUIs: unsafe extern "system" fn(this: *mut *mut Self, pguidcontenttype: *const ::windows_sys::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderRegistrationInstance: unsafe extern "system" fn(this: *mut *mut Self, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderRegistrationInstance: usize,
    pub UnregisterSyncProvider: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfoforProvider: unsafe extern "system" fn(this: *mut *mut Self, pguidproviderinstanceid: *const ::windows_sys::core::GUID, ppproviderconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfoforProvider: usize,
    pub EnumerateSyncProviders: unsafe extern "system" fn(this: *mut *mut Self, pguidcontenttype: *const ::windows_sys::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_sys::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderInfo: usize,
    pub GetSyncProviderFromInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfo: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfo: usize,
    pub GetSyncProviderConfigUIFromInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, dwclscontext: u32, ppconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSyncProviderState: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, pdwstateflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSyncProviderState: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *const ::windows_sys::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterForEvent: unsafe extern "system" fn(this: *mut *mut Self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterForEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RevokeEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RevokeEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetChange: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetChange: usize,
}
impl ::windows_sys::core::Interface for ISyncProviderRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3410335035, data2: 30244, data3: 18364, data4: [164, 114, 235, 140, 172, 107, 34, 46] };
}
#[repr(C)]
pub struct ISyncRegistrationChange {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetEvent: unsafe extern "system" fn(this: *mut *mut Self, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows_sys::core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pguidinstanceid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncRegistrationChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003518894, data2: 27433, data3: 17332, data4: [158, 112, 227, 174, 51, 187, 44, 59] };
}
#[repr(C)]
pub struct ISyncSessionExtendedErrorInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSyncProviderWithError: unsafe extern "system" fn(this: *mut *mut Self, ppproviderwitherror: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncSessionExtendedErrorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 845965328, data2: 30986, data3: 16539, data4: [183, 65, 105, 153, 56, 135, 97, 235] };
}
#[repr(C)]
pub struct ISyncSessionState {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCanceled: usize,
    pub GetInfoForChangeApplication: unsafe extern "system" fn(this: *mut *mut Self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LoadInfoFromChangeApplication: unsafe extern "system" fn(this: *mut *mut Self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_sys::core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeStart: unsafe extern "system" fn(this: *mut *mut Self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeEnd: unsafe extern "system" fn(this: *mut *mut Self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetForgottenKnowledgeRecoveryRange: unsafe extern "system" fn(this: *mut *mut Self, prange: *const SYNC_RANGE) -> ::windows_sys::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut *mut Self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncSessionState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3098099966, data2: 40705, data3: 18491, data4: [148, 52, 195, 125, 54, 18, 37, 217] };
}
#[repr(C)]
pub struct ISyncSessionState2 {
    pub base__: ISyncSessionState,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderWithError: unsafe extern "system" fn(this: *mut *mut Self, fself: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderWithError: usize,
    pub GetSessionErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, phrsessionerror: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISyncSessionState2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654457763, data2: 40504, data3: 19553, data4: [156, 163, 255, 232, 16, 180, 92, 162] };
}
#[repr(C)]
pub struct ISynchronousDataRetriever {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut *mut Self, pidparameters: *mut ID_PARAMETERS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut *mut Self, ploadchangecontext: *mut ::core::ffi::c_void, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISynchronousDataRetriever {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2602758825, data2: 42189, data3: 17992, data4: [157, 142, 58, 81, 13, 77, 160, 75] };
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type KNOWLEDGE_COOKIE_COMPARISON_RESULT = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 1430988010, data2: 59619, data3: 17850, data4: [147, 82, 223, 181, 97, 225, 113, 228] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2216140385, data2: 24822, data3: 19484, data4: [136, 237, 241, 197, 49, 179, 43, 218] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_CONSTRAINT_RESOLVE_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = 5i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: super::super::Foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYNC_FILTER_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYNC_FILTER_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_FULL_ENUMERATION_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_PROGRESS_STAGE = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_PROVIDER_ROLE = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl ::core::marker::Copy for SYNC_RANGE {}
impl ::core::clone::Clone for SYNC_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_REGISTRATION_EVENT = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_RESOLVE_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_LAST: SYNC_RESOLVE_ACTION = 5i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_SERIALIZATION_VERSION = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = 4i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = 5i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl ::core::marker::Copy for SYNC_SESSION_STATISTICS {}
impl ::core::clone::Clone for SYNC_SESSION_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub type SYNC_STATISTICS = i32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for SYNC_TIME {}
impl ::core::clone::Clone for SYNC_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl ::core::marker::Copy for SYNC_VERSION {}
impl ::core::clone::Clone for SYNC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_sys::core::GUID,
    pub clsidConfigUI: ::windows_sys::core::GUID,
    pub guidContentType: ::windows_sys::core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SyncProviderConfigUIConfiguration {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SyncProviderConfigUIConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows_sys::core::GUID,
    pub clsidProvider: ::windows_sys::core::GUID,
    pub guidConfigUIInstanceId: ::windows_sys::core::GUID,
    pub guidContentType: ::windows_sys::core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl ::core::marker::Copy for SyncProviderConfiguration {}
impl ::core::clone::Clone for SyncProviderConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SyncProviderRegistration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4163587825, data2: 37801, data3: 19934, data4: [128, 21, 247, 149, 10, 26, 110, 49] };
