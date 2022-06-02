#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn BackupPerfRegistryToFileW(szfilename: ::windows_sys::core::PCWSTR, szcommentstring: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn InstallPerfDllA(szcomputername: ::windows_sys::core::PCSTR, lpinifile: ::windows_sys::core::PCSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn InstallPerfDllW(szcomputername: ::windows_sys::core::PCWSTR, lpinifile: ::windows_sys::core::PCWSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsA(lpcommandline: ::windows_sys::core::PCSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsW(lpcommandline: ::windows_sys::core::PCWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhAddCounterA(hquery: isize, szfullcounterpath: ::windows_sys::core::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhAddCounterW(hquery: isize, szfullcounterpath: ::windows_sys::core::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: ::windows_sys::core::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: ::windows_sys::core::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCloseQuery(hquery: isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCollectQueryData(hquery: isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhConnectMachineA(szmachinename: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhConnectMachineW(szmachinename: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCreateSQLTablesA(szdatasource: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhCreateSQLTablesW(szdatasource: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumLogSetNamesA(szdatasource: ::windows_sys::core::PCSTR, mszdatasetnamelist: ::windows_sys::core::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumLogSetNamesW(szdatasource: ::windows_sys::core::PCWSTR, mszdatasetnamelist: ::windows_sys::core::PWSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumMachinesA(szdatasource: ::windows_sys::core::PCSTR, mszmachinelist: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumMachinesW(szdatasource: ::windows_sys::core::PCWSTR, mszmachinelist: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumObjectItemsA(szdatasource: ::windows_sys::core::PCSTR, szmachinename: ::windows_sys::core::PCSTR, szobjectname: ::windows_sys::core::PCSTR, mszcounterlist: ::windows_sys::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_sys::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: ::windows_sys::core::PCSTR, szobjectname: ::windows_sys::core::PCSTR, mszcounterlist: ::windows_sys::core::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_sys::core::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, mszcounterlist: ::windows_sys::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_sys::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhEnumObjectItemsW(szdatasource: ::windows_sys::core::PCWSTR, szmachinename: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, mszcounterlist: ::windows_sys::core::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_sys::core::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsA(szdatasource: ::windows_sys::core::PCSTR, szmachinename: ::windows_sys::core::PCSTR, mszobjectlist: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: ::windows_sys::core::PCSTR, mszobjectlist: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: ::windows_sys::core::PCWSTR, mszobjectlist: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsW(szdatasource: ::windows_sys::core::PCWSTR, szmachinename: ::windows_sys::core::PCWSTR, mszobjectlist: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandCounterPathA(szwildcardpath: ::windows_sys::core::PCSTR, mszexpandedpathlist: ::windows_sys::core::PSTR, pcchpathlistlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandCounterPathW(szwildcardpath: ::windows_sys::core::PCWSTR, mszexpandedpathlist: ::windows_sys::core::PWSTR, pcchpathlistlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandWildCardPathA(szdatasource: ::windows_sys::core::PCSTR, szwildcardpath: ::windows_sys::core::PCSTR, mszexpandedpathlist: ::windows_sys::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: ::windows_sys::core::PCSTR, mszexpandedpathlist: ::windows_sys::core::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: ::windows_sys::core::PCWSTR, mszexpandedpathlist: ::windows_sys::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhExpandWildCardPathW(szdatasource: ::windows_sys::core::PCWSTR, szwildcardpath: ::windows_sys::core::PCWSTR, mszexpandedpathlist: ::windows_sys::core::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDataSourceTimeRangeA(szdatasource: ::windows_sys::core::PCSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDataSourceTimeRangeW(szdatasource: ::windows_sys::core::PCWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfCounterA(szdatasource: ::windows_sys::core::PCSTR, szmachinename: ::windows_sys::core::PCSTR, szobjectname: ::windows_sys::core::PCSTR, szdefaultcountername: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: ::windows_sys::core::PCSTR, szobjectname: ::windows_sys::core::PCSTR, szdefaultcountername: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, szdefaultcountername: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfCounterW(szdatasource: ::windows_sys::core::PCWSTR, szmachinename: ::windows_sys::core::PCWSTR, szobjectname: ::windows_sys::core::PCWSTR, szdefaultcountername: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfObjectA(szdatasource: ::windows_sys::core::PCSTR, szmachinename: ::windows_sys::core::PCSTR, szdefaultobjectname: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: ::windows_sys::core::PCSTR, szdefaultobjectname: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: ::windows_sys::core::PCWSTR, szdefaultobjectname: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDefaultPerfObjectW(szdatasource: ::windows_sys::core::PCWSTR, szmachinename: ::windows_sys::core::PCWSTR, szdefaultobjectname: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows_sys::core::GUID, prunid: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhLookupPerfIndexByNameA(szmachinename: ::windows_sys::core::PCSTR, sznamebuffer: ::windows_sys::core::PCSTR, pdwindex: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhLookupPerfIndexByNameW(szmachinename: ::windows_sys::core::PCWSTR, sznamebuffer: ::windows_sys::core::PCWSTR, pdwindex: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhLookupPerfNameByIndexA(szmachinename: ::windows_sys::core::PCSTR, dwnameindex: u32, sznamebuffer: ::windows_sys::core::PSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhLookupPerfNameByIndexW(szmachinename: ::windows_sys::core::PCWSTR, dwnameindex: u32, sznamebuffer: ::windows_sys::core::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: ::windows_sys::core::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: ::windows_sys::core::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhOpenLogA(szlogfilename: ::windows_sys::core::PCSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows_sys::core::PCSTR, phlog: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhOpenLogW(szlogfilename: ::windows_sys::core::PCWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows_sys::core::PCWSTR, phlog: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhOpenQueryA(szdatasource: ::windows_sys::core::PCSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhOpenQueryW(szdatasource: ::windows_sys::core::PCWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhParseCounterPathA(szfullpathbuffer: ::windows_sys::core::PCSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhParseCounterPathW(szfullpathbuffer: ::windows_sys::core::PCWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhParseInstanceNameA(szinstancestring: ::windows_sys::core::PCSTR, szinstancename: ::windows_sys::core::PSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows_sys::core::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhParseInstanceNameW(szinstancestring: ::windows_sys::core::PCWSTR, szinstancename: ::windows_sys::core::PWSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows_sys::core::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhRemoveCounter(hcounter: isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows_sys::core::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows_sys::core::PWSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhUpdateLogA(hlog: isize, szuserstring: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhUpdateLogW(hlog: isize, szuserstring: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhValidatePathA(szfullpathbuffer: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhValidatePathW(szfullpathbuffer: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhVerifySQLDBA(szdatasource: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PdhVerifySQLDBW(szdatasource: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows_sys::core::GUID, name: ::windows_sys::core::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfEnumerateCounterSet(szmachine: ::windows_sys::core::PCWSTR, pcountersetids: *mut ::windows_sys::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfEnumerateCounterSetInstances(szmachine: ::windows_sys::core::PCWSTR, pcountersetid: *const ::windows_sys::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfOpenQueryHandle(szmachine: ::windows_sys::core::PCWSTR, phquery: *mut PerfQueryHandle) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfQueryCounterSetRegistrationInfo(szmachine: ::windows_sys::core::PCWSTR, pcountersetid: *const ::windows_sys::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows_sys::core::GUID, name: ::windows_sys::core::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfStartProvider(providerguid: *const ::windows_sys::core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut PerfProviderHandle) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfStartProviderEx(providerguid: *const ::windows_sys::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn RestorePerfRegistryFromFileW(szfilename: ::windows_sys::core::PCWSTR, szlangid: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn SetServiceAsTrustedA(szreserved: ::windows_sys::core::PCSTR, szservicename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn SetServiceAsTrustedW(szreserved: ::windows_sys::core::PCWSTR, szservicename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsA(lpcommandline: ::windows_sys::core::PCSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsW(lpcommandline: ::windows_sys::core::PCWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn UpdatePerfNameFilesA(sznewctrfilepath: ::windows_sys::core::PCSTR, sznewhlpfilepath: ::windows_sys::core::PCSTR, szlanguageid: ::windows_sys::core::PCSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance\"`*"]
    pub fn UpdatePerfNameFilesW(sznewctrfilepath: ::windows_sys::core::PCWSTR, sznewhlpfilepath: ::windows_sys::core::PCWSTR, szlanguageid: ::windows_sys::core::PCWSTR, dwflags: usize) -> u32;
}
pub const AppearPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3835118057, data2: 37800, data3: 19121, data4: [142, 150, 191, 68, 130, 40, 46, 156] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type AutoPathFormat = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaNone: AutoPathFormat = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPattern: AutoPathFormat = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaComputer: AutoPathFormat = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonthDayHour: AutoPathFormat = 256i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSerialNumber: AutoPathFormat = 512i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearDayOfYear: AutoPathFormat = 1024i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonth: AutoPathFormat = 2048i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonthDay: AutoPathFormat = 4096i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaYearMonthDayHour: AutoPathFormat = 8192i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonthDayHourMinute: AutoPathFormat = 16384i32;
pub const BootTraceSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946872, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const BootTraceSessionCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946873, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type ClockType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTimeStamp: ClockType = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPerformance: ClockType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSystem: ClockType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCycle: ClockType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type CommitMode = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateNew: CommitMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaModify: CommitMode = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateOrModify: CommitMode = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaUpdateRunningInstance: CommitMode = 16i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlushTrace: CommitMode = 32i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaValidateOnly: CommitMode = 4096i32;
pub const CounterItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3302152416, data2: 53725, data3: 4558, data4: [148, 15, 0, 128, 41, 0, 67, 72] };
pub const CounterItem2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125739618, data2: 49951, data3: 19683, data4: [160, 46, 121, 239, 224, 246, 165, 37] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type CounterPathCallBack = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3482617185, data2: 60904, data3: 4558, data4: [148, 30, 0, 128, 41, 0, 67, 71] };
pub const Counters: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2997905106, data2: 10924, data3: 4559, data4: [148, 47, 0, 128, 41, 0, 67, 71] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DICounterItem {
    pub base__: super::Com::IDispatch,
}
pub const DIID_DICounterItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3230420978, data2: 3630, data3: 4559, data4: [148, 44, 0, 128, 41, 0, 67, 71] };
pub const DIID_DILogFileItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2366193660, data2: 63351, data3: 18711, data4: [130, 209, 131, 63, 188, 84, 197, 143] };
pub const DIID_DISystemMonitor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 332873089, data2: 49966, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
pub const DIID_DISystemMonitorEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2224527664, data2: 19123, data3: 4559, data4: [148, 58, 0, 128, 41, 0, 67, 71] };
pub const DIID_DISystemMonitorInternal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 424587842, data2: 49964, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DILogFileItem {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitor {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitorEvents {
    pub base__: super::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DISystemMonitorInternal {
    pub base__: super::Com::IDispatch,
}
pub const DataCollectorSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946849, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const DataCollectorSetCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946853, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type DataCollectorSetStatus = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaStopped: DataCollectorSetStatus = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunning: DataCollectorSetStatus = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCompiling: DataCollectorSetStatus = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPending: DataCollectorSetStatus = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaUndefined: DataCollectorSetStatus = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type DataCollectorType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaPerformanceCounter: DataCollectorType = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTrace: DataCollectorType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaConfiguration: DataCollectorType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaAlert: DataCollectorType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaApiTrace: DataCollectorType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type DataManagerSteps = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateReport: DataManagerSteps = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunRules: DataManagerSteps = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateHtml: DataManagerSteps = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFolderActions: DataManagerSteps = 8i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaResourceFreeing: DataManagerSteps = 16i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type DataSourceTypeConstants = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonNullDataSource: DataSourceTypeConstants = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonCurrentActivity: DataSourceTypeConstants = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonLogFiles: DataSourceTypeConstants = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonSqlLog: DataSourceTypeConstants = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type DisplayTypeConstants = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonLineGraph: DisplayTypeConstants = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonHistogram: DisplayTypeConstants = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonReport: DisplayTypeConstants = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonChartArea: DisplayTypeConstants = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonChartStackedArea: DisplayTypeConstants = 5i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type FileFormat = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCommaSeparated: FileFormat = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTabSeparated: FileFormat = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSql: FileFormat = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBinary: FileFormat = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type FolderActionSteps = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaCreateCab: FolderActionSteps = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteData: FolderActionSteps = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSendCab: FolderActionSteps = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteCab: FolderActionSteps = 8i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteReport: FolderActionSteps = 16i32;
pub const GeneralPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3286619090, data2: 6659, data3: 4559, data4: [148, 45, 0, 128, 41, 0, 67, 71] };
pub const GraphPropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3286619091, data2: 6659, data3: 4559, data4: [148, 45, 0, 128, 41, 0, 67, 71] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlertDataCollector {
    pub base__: IDataCollector,
    #[cfg(feature = "Win32_System_Com")]
    pub AlertThresholds: unsafe extern "system" fn(this: *mut *mut Self, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlertThresholds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAlertThresholds: unsafe extern "system" fn(this: *mut *mut Self, alerts: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAlertThresholds: usize,
    pub EventLog: unsafe extern "system" fn(this: *mut *mut Self, log: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEventLog: unsafe extern "system" fn(this: *mut *mut Self, log: i16) -> ::windows_sys::core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(this: *mut *mut Self, interval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(this: *mut *mut Self, interval: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, task: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut *mut Self, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    pub TaskRunAsSelf: unsafe extern "system" fn(this: *mut *mut Self, runasself: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(this: *mut *mut Self, runasself: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskArguments: unsafe extern "system" fn(this: *mut *mut Self, task: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskArguments: unsafe extern "system" fn(this: *mut *mut Self, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskUserTextArguments: unsafe extern "system" fn(this: *mut *mut Self, task: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskUserTextArguments: unsafe extern "system" fn(this: *mut *mut Self, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TriggerDataCollectorSet: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TriggerDataCollectorSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTriggerDataCollectorSet: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTriggerDataCollectorSet: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IApiTracingDataCollector {
    pub base__: IDataCollector,
    pub LogApiNamesOnly: unsafe extern "system" fn(this: *mut *mut Self, logapinames: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogApiNamesOnly: unsafe extern "system" fn(this: *mut *mut Self, logapinames: i16) -> ::windows_sys::core::HRESULT,
    pub LogApisRecursively: unsafe extern "system" fn(this: *mut *mut Self, logrecursively: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogApisRecursively: unsafe extern "system" fn(this: *mut *mut Self, logrecursively: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExePath: unsafe extern "system" fn(this: *mut *mut Self, exepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExePath: unsafe extern "system" fn(this: *mut *mut Self, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFilePath: unsafe extern "system" fn(this: *mut *mut Self, logfilepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFilePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFilePath: unsafe extern "system" fn(this: *mut *mut Self, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFilePath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeModules: unsafe extern "system" fn(this: *mut *mut Self, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeModules: unsafe extern "system" fn(this: *mut *mut Self, includemodules: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeModules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncludeApis: unsafe extern "system" fn(this: *mut *mut Self, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIncludeApis: unsafe extern "system" fn(this: *mut *mut Self, includeapis: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIncludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeApis: unsafe extern "system" fn(this: *mut *mut Self, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeApis: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeApis: unsafe extern "system" fn(this: *mut *mut Self, excludeapis: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeApis: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IConfigurationDataCollector {
    pub base__: IDataCollector,
    pub FileMaxCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFileMaxCount: unsafe extern "system" fn(this: *mut *mut Self, count: u32) -> ::windows_sys::core::HRESULT,
    pub FileMaxRecursiveDepth: unsafe extern "system" fn(this: *mut *mut Self, depth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFileMaxRecursiveDepth: unsafe extern "system" fn(this: *mut *mut Self, depth: u32) -> ::windows_sys::core::HRESULT,
    pub FileMaxTotalSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFileMaxTotalSize: unsafe extern "system" fn(this: *mut *mut Self, size: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Files: unsafe extern "system" fn(this: *mut *mut Self, files: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Files: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFiles: unsafe extern "system" fn(this: *mut *mut Self, files: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ManagementQueries: unsafe extern "system" fn(this: *mut *mut Self, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ManagementQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetManagementQueries: unsafe extern "system" fn(this: *mut *mut Self, queries: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetManagementQueries: usize,
    pub QueryNetworkAdapters: unsafe extern "system" fn(this: *mut *mut Self, network: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetQueryNetworkAdapters: unsafe extern "system" fn(this: *mut *mut Self, network: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistryKeys: unsafe extern "system" fn(this: *mut *mut Self, query: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistryKeys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistryKeys: unsafe extern "system" fn(this: *mut *mut Self, query: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistryKeys: usize,
    pub RegistryMaxRecursiveDepth: unsafe extern "system" fn(this: *mut *mut Self, depth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetRegistryMaxRecursiveDepth: unsafe extern "system" fn(this: *mut *mut Self, depth: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SystemStateFile: unsafe extern "system" fn(this: *mut *mut Self, filename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SystemStateFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSystemStateFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSystemStateFile: usize,
}
#[repr(C)]
pub struct ICounterItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pdblvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, iwidth: i32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(this: *mut *mut Self, ilinestyle: i32) -> ::windows_sys::core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, iscale: i32) -> ::windows_sys::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, status: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut *mut Self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICounterItem2 {
    pub base__: ICounterItem,
    pub SetSelected: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDataAt: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDataAt: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICounters {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plong: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollector {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectorSet: unsafe extern "system" fn(this: *mut *mut Self, group: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectorSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDataCollectorSet: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDataCollectorSet: usize,
    pub DataCollectorType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut DataCollectorType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileName: usize,
    pub FileNameFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut AutoPathFormat) -> ::windows_sys::core::HRESULT,
    pub SetFileNameFormat: unsafe extern "system" fn(this: *mut *mut Self, format: AutoPathFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileNameFormatPattern: unsafe extern "system" fn(this: *mut *mut Self, pattern: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileNameFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileNameFormatPattern: unsafe extern "system" fn(this: *mut *mut Self, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileNameFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LatestOutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLatestOutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLatestOutputLocation: usize,
    pub LogAppend: unsafe extern "system" fn(this: *mut *mut Self, append: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogAppend: unsafe extern "system" fn(this: *mut *mut Self, append: i16) -> ::windows_sys::core::HRESULT,
    pub LogCircular: unsafe extern "system" fn(this: *mut *mut Self, circular: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogCircular: unsafe extern "system" fn(this: *mut *mut Self, circular: i16) -> ::windows_sys::core::HRESULT,
    pub LogOverwrite: unsafe extern "system" fn(this: *mut *mut Self, overwrite: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogOverwrite: unsafe extern "system" fn(this: *mut *mut Self, overwrite: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutputLocation: usize,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, index: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut *mut Self, xml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetXml: unsafe extern "system" fn(this: *mut *mut Self, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetXml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateOutputLocation: unsafe extern "system" fn(this: *mut *mut Self, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateOutputLocation: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, collector: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, collectors: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(this: *mut *mut Self, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut *mut ::core::ffi::c_void, pcollector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDataCollectorFromXml: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDataCollector: unsafe extern "system" fn(this: *mut *mut Self, r#type: DataCollectorType, collector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDataCollector: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorSet {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub DataCollectors: unsafe extern "system" fn(this: *mut *mut Self, collectors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataCollectors: usize,
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, seconds: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, seconds: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DescriptionUnresolved: unsafe extern "system" fn(this: *mut *mut Self, descr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DescriptionUnresolved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayNameUnresolved: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayNameUnresolved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Keywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keywords: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetKeywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetKeywords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LatestOutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLatestOutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLatestOutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OutputLocation: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OutputLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RootPath: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RootPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRootPath: unsafe extern "system" fn(this: *mut *mut Self, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRootPath: usize,
    pub Segment: unsafe extern "system" fn(this: *mut *mut Self, segment: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSegment: unsafe extern "system" fn(this: *mut *mut Self, segment: i16) -> ::windows_sys::core::HRESULT,
    pub SegmentMaxDuration: unsafe extern "system" fn(this: *mut *mut Self, seconds: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSegmentMaxDuration: unsafe extern "system" fn(this: *mut *mut Self, seconds: u32) -> ::windows_sys::core::HRESULT,
    pub SegmentMaxSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSegmentMaxSize: unsafe extern "system" fn(this: *mut *mut Self, size: u32) -> ::windows_sys::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, index: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSerialNumber: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Server: unsafe extern "system" fn(this: *mut *mut Self, server: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Server: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, status: *mut DataCollectorSetStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Subdirectory: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subdirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubdirectory: unsafe extern "system" fn(this: *mut *mut Self, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubdirectory: usize,
    pub SubdirectoryFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut AutoPathFormat) -> ::windows_sys::core::HRESULT,
    pub SetSubdirectoryFormat: unsafe extern "system" fn(this: *mut *mut Self, format: AutoPathFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SubdirectoryFormatPattern: unsafe extern "system" fn(this: *mut *mut Self, pattern: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubdirectoryFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubdirectoryFormatPattern: unsafe extern "system" fn(this: *mut *mut Self, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubdirectoryFormatPattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, task: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut *mut Self, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    pub TaskRunAsSelf: unsafe extern "system" fn(this: *mut *mut Self, runasself: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(this: *mut *mut Self, runasself: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskArguments: unsafe extern "system" fn(this: *mut *mut Self, task: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskArguments: unsafe extern "system" fn(this: *mut *mut Self, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskUserTextArguments: unsafe extern "system" fn(this: *mut *mut Self, usertext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskUserTextArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTaskUserTextArguments: unsafe extern "system" fn(this: *mut *mut Self, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTaskUserTextArguments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Schedules: unsafe extern "system" fn(this: *mut *mut Self, ppschedules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Schedules: usize,
    pub SchedulesEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSchedulesEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut *mut Self, user: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut *mut Self, xml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, pbstrsecurity: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Security: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurity: usize,
    pub StopOnCompletion: unsafe extern "system" fn(this: *mut *mut Self, stop: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetStopOnCompletion: unsafe extern "system" fn(this: *mut *mut Self, stop: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DataManager: unsafe extern "system" fn(this: *mut *mut Self, datamanager: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataManager: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut *mut Self, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Commit: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, synchronous: i16) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self, synchronous: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetXml: unsafe extern "system" fn(this: *mut *mut Self, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetXml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataCollectorSetCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, set: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, sets: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataCollectorSets: unsafe extern "system" fn(this: *mut *mut Self, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataCollectorSets: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDataManager {
    pub base__: super::Com::IDispatch,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenabled: i16) -> ::windows_sys::core::HRESULT,
    pub CheckBeforeRunning: unsafe extern "system" fn(this: *mut *mut Self, pfcheck: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetCheckBeforeRunning: unsafe extern "system" fn(this: *mut *mut Self, fcheck: i16) -> ::windows_sys::core::HRESULT,
    pub MinFreeDisk: unsafe extern "system" fn(this: *mut *mut Self, minfreedisk: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinFreeDisk: unsafe extern "system" fn(this: *mut *mut Self, minfreedisk: u32) -> ::windows_sys::core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(this: *mut *mut Self, pulmaxsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxSize: unsafe extern "system" fn(this: *mut *mut Self, ulmaxsize: u32) -> ::windows_sys::core::HRESULT,
    pub MaxFolderCount: unsafe extern "system" fn(this: *mut *mut Self, pulmaxfoldercount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxFolderCount: unsafe extern "system" fn(this: *mut *mut Self, ulmaxfoldercount: u32) -> ::windows_sys::core::HRESULT,
    pub ResourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, ppolicy: *mut ResourcePolicy) -> ::windows_sys::core::HRESULT,
    pub SetResourcePolicy: unsafe extern "system" fn(this: *mut *mut Self, policy: ResourcePolicy) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FolderActions: unsafe extern "system" fn(this: *mut *mut Self, actions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FolderActions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportSchema: unsafe extern "system" fn(this: *mut *mut Self, reportschema: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportSchema: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReportSchema: unsafe extern "system" fn(this: *mut *mut Self, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReportSchema: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReportFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReportFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RuleTargetFileName: unsafe extern "system" fn(this: *mut *mut Self, filename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RuleTargetFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRuleTargetFileName: unsafe extern "system" fn(this: *mut *mut Self, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRuleTargetFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventsFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventsFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventsFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventsFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Rules: unsafe extern "system" fn(this: *mut *mut Self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Rules: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRules: unsafe extern "system" fn(this: *mut *mut Self, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRules: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Run: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Extract: unsafe extern "system" fn(this: *mut *mut Self, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Extract: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFolderAction {
    pub base__: super::Com::IDispatch,
    pub Age: unsafe extern "system" fn(this: *mut *mut Self, pulage: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAge: unsafe extern "system" fn(this: *mut *mut Self, ulage: u32) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, pulage: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, ulage: u32) -> ::windows_sys::core::HRESULT,
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, steps: *mut FolderActionSteps) -> ::windows_sys::core::HRESULT,
    pub SetActions: unsafe extern "system" fn(this: *mut *mut Self, steps: FolderActionSteps) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendCabTo: unsafe extern "system" fn(this: *mut *mut Self, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendCabTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSendCabTo: unsafe extern "system" fn(this: *mut *mut Self, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSendCabTo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFolderActionCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, r#enum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, action: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, actions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFolderAction: unsafe extern "system" fn(this: *mut *mut Self, folderaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFolderAction: usize,
}
#[repr(C)]
pub struct ILogFileItem {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILogFiles {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plong: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPerformanceCounterDataCollector {
    pub base__: IDataCollector,
    #[cfg(feature = "Win32_Foundation")]
    pub DataSourceName: unsafe extern "system" fn(this: *mut *mut Self, dsn: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DataSourceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDataSourceName: unsafe extern "system" fn(this: *mut *mut Self, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDataSourceName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PerformanceCounters: unsafe extern "system" fn(this: *mut *mut Self, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PerformanceCounters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPerformanceCounters: unsafe extern "system" fn(this: *mut *mut Self, counters: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPerformanceCounters: usize,
    pub LogFileFormat: unsafe extern "system" fn(this: *mut *mut Self, format: *mut FileFormat) -> ::windows_sys::core::HRESULT,
    pub SetLogFileFormat: unsafe extern "system" fn(this: *mut *mut Self, format: FileFormat) -> ::windows_sys::core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(this: *mut *mut Self, interval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(this: *mut *mut Self, interval: u32) -> ::windows_sys::core::HRESULT,
    pub SegmentMaxRecords: unsafe extern "system" fn(this: *mut *mut Self, records: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSegmentMaxRecords: unsafe extern "system" fn(this: *mut *mut Self, records: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISchedule {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartDate: unsafe extern "system" fn(this: *mut *mut Self, start: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetStartDate: unsafe extern "system" fn(this: *mut *mut Self, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetStartDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EndDate: unsafe extern "system" fn(this: *mut *mut Self, end: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EndDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEndDate: unsafe extern "system" fn(this: *mut *mut Self, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEndDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, start: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetStartTime: usize,
    pub Days: unsafe extern "system" fn(this: *mut *mut Self, days: *mut WeekDays) -> ::windows_sys::core::HRESULT,
    pub SetDays: unsafe extern "system" fn(this: *mut *mut Self, days: WeekDays) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IScheduleCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ienum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pschedule: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, pschedules: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSchedule: unsafe extern "system" fn(this: *mut *mut Self, schedule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSchedule: usize,
}
#[repr(C)]
pub struct ISystemMonitor {
    pub base__: ::windows_sys::core::IUnknown,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, iappearance: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut *mut Self, iappearance: i32) -> ::windows_sys::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut *mut Self, iborderstyle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut *mut Self, iborderstyle: i32) -> ::windows_sys::core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(this: *mut *mut Self, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut *mut Self, pfont: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(this: *mut *mut Self, ppicounters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, fvalue: f32) -> ::windows_sys::core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, pfvalue: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut *mut Self, edisplaytype: DisplayTypeConstants) -> ::windows_sys::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut *mut Self, pedisplaytype: *mut DisplayTypeConstants) -> ::windows_sys::core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGraphTitle: unsafe extern "system" fn(this: *mut *mut Self, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GraphTitle: unsafe extern "system" fn(this: *mut *mut Self, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetYAxisLabel: unsafe extern "system" fn(this: *mut *mut Self, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetYAxisLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub YAxisLabel: unsafe extern "system" fn(this: *mut *mut Self, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    YAxisLabel: usize,
    pub CollectSample: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Counter: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, ppicounter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCounter: unsafe extern "system" fn(this: *mut *mut Self, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCounter: usize,
    pub DeleteCounter: unsafe extern "system" fn(this: *mut *mut Self, pctr: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFileName: unsafe extern "system" fn(this: *mut *mut Self, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFileName: unsafe extern "system" fn(this: *mut *mut Self, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFileName: usize,
    pub SetLogViewStart: unsafe extern "system" fn(this: *mut *mut Self, starttime: f64) -> ::windows_sys::core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(this: *mut *mut Self, starttime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(this: *mut *mut Self, stoptime: f64) -> ::windows_sys::core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(this: *mut *mut Self, stoptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GridColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub Highlight: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(this: *mut *mut Self, ereportvaluetype: ReportValueTypeConstants) -> ::windows_sys::core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(this: *mut *mut Self, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows_sys::core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(this: *mut *mut Self, ppilogfiles: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(this: *mut *mut Self, edatasourcetype: DataSourceTypeConstants) -> ::windows_sys::core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(this: *mut *mut Self, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlDsnName: unsafe extern "system" fn(this: *mut *mut Self, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlDsnName: unsafe extern "system" fn(this: *mut *mut Self, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlLogSetName: unsafe extern "system" fn(this: *mut *mut Self, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlLogSetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlLogSetName: unsafe extern "system" fn(this: *mut *mut Self, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlLogSetName: usize,
}
#[repr(C)]
pub struct ISystemMonitor2 {
    pub base__: ISystemMonitor,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(this: *mut *mut Self, bscroll: i16) -> ::windows_sys::core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(this: *mut *mut Self, pbscroll: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(this: *mut *mut Self, inewcount: i32) -> ::windows_sys::core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(this: *mut *mut Self, pidatapointcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(this: *mut *mut Self, bselectedcountersonly: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAs: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Relog: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Relog: usize,
    pub ClearData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(this: *mut *mut Self, starttime: f64, stoptime: f64) -> ::windows_sys::core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(this: *mut *mut Self, starttime: *mut f64, stoptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(this: *mut *mut Self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadSettings: unsafe extern "system" fn(this: *mut *mut Self, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadSettings: usize,
}
#[repr(C)]
pub struct ISystemMonitorEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCounterSelected: unsafe extern "system" fn(this: *mut *mut Self, index: i32),
    pub OnCounterAdded: unsafe extern "system" fn(this: *mut *mut Self, index: i32),
    pub OnCounterDeleted: unsafe extern "system" fn(this: *mut *mut Self, index: i32),
    pub OnSampleCollected: unsafe extern "system" fn(this: *mut *mut Self),
    pub OnDblClick: unsafe extern "system" fn(this: *mut *mut Self, index: i32),
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITraceDataCollector {
    pub base__: IDataCollector,
    pub BufferSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut *mut Self, size: u32) -> ::windows_sys::core::HRESULT,
    pub BuffersLost: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBuffersLost: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub BuffersWritten: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBuffersWritten: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub ClockType: unsafe extern "system" fn(this: *mut *mut Self, clock: *mut ClockType) -> ::windows_sys::core::HRESULT,
    pub SetClockType: unsafe extern "system" fn(this: *mut *mut Self, clock: ClockType) -> ::windows_sys::core::HRESULT,
    pub EventsLost: unsafe extern "system" fn(this: *mut *mut Self, events: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetEventsLost: unsafe extern "system" fn(this: *mut *mut Self, events: u32) -> ::windows_sys::core::HRESULT,
    pub ExtendedModes: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetExtendedModes: unsafe extern "system" fn(this: *mut *mut Self, mode: u32) -> ::windows_sys::core::HRESULT,
    pub FlushTimer: unsafe extern "system" fn(this: *mut *mut Self, seconds: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFlushTimer: unsafe extern "system" fn(this: *mut *mut Self, seconds: u32) -> ::windows_sys::core::HRESULT,
    pub FreeBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFreeBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, guid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut *mut Self, guid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IsKernelTrace: unsafe extern "system" fn(this: *mut *mut Self, kernel: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MaximumBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaximumBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub MinimumBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinimumBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub NumberOfBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNumberOfBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub PreallocateFile: unsafe extern "system" fn(this: *mut *mut Self, allocate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPreallocateFile: unsafe extern "system" fn(this: *mut *mut Self, allocate: i16) -> ::windows_sys::core::HRESULT,
    pub ProcessMode: unsafe extern "system" fn(this: *mut *mut Self, process: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetProcessMode: unsafe extern "system" fn(this: *mut *mut Self, process: i16) -> ::windows_sys::core::HRESULT,
    pub RealTimeBuffersLost: unsafe extern "system" fn(this: *mut *mut Self, buffers: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetRealTimeBuffersLost: unsafe extern "system" fn(this: *mut *mut Self, buffers: u32) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, id: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetSessionId: unsafe extern "system" fn(this: *mut *mut Self, id: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSessionName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSessionName: usize,
    pub SessionThreadId: unsafe extern "system" fn(this: *mut *mut Self, tid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSessionThreadId: unsafe extern "system" fn(this: *mut *mut Self, tid: u32) -> ::windows_sys::core::HRESULT,
    pub StreamMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut StreamMode) -> ::windows_sys::core::HRESULT,
    pub SetStreamMode: unsafe extern "system" fn(this: *mut *mut Self, mode: StreamMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TraceDataProviders: unsafe extern "system" fn(this: *mut *mut Self, providers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TraceDataProviders: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITraceDataProvider {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, guid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut *mut Self, guid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Level: unsafe extern "system" fn(this: *mut *mut Self, pplevel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Level: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAny: unsafe extern "system" fn(this: *mut *mut Self, ppkeywords: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAny: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KeywordsAll: unsafe extern "system" fn(this: *mut *mut Self, ppkeywords: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KeywordsAll: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub FilterEnabled: unsafe extern "system" fn(this: *mut *mut Self, filterenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFilterEnabled: unsafe extern "system" fn(this: *mut *mut Self, filterenabled: i16) -> ::windows_sys::core::HRESULT,
    pub FilterType: unsafe extern "system" fn(this: *mut *mut Self, pultype: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetFilterType: unsafe extern "system" fn(this: *mut *mut Self, ultype: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilterData: unsafe extern "system" fn(this: *mut *mut Self, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilterData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFilterData: unsafe extern "system" fn(this: *mut *mut Self, pdata: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFilterData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resolve: unsafe extern "system" fn(this: *mut *mut Self, pfrom: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resolve: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurity: unsafe extern "system" fn(this: *mut *mut Self, securityinfo: u32, sddl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredProcesses: unsafe extern "system" fn(this: *mut *mut Self, processes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredProcesses: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITraceDataProviderCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, providers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTraceDataProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTraceDataProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTraceDataProviders: unsafe extern "system" fn(this: *mut *mut Self, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTraceDataProviders: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(this: *mut *mut Self, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTraceDataProvidersByProcess: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IValueMap {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ValueMapType) -> ::windows_sys::core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(this: *mut *mut Self, r#type: ValueMapType) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, map: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValueMapItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValueMapItem: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IValueMapItem {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, key: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Key: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKey: unsafe extern "system" fn(this: *mut *mut Self, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ValueMapType) -> ::windows_sys::core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(this: *mut *mut Self, r#type: ValueMapType) -> ::windows_sys::core::HRESULT,
}
pub const LIBID_SystemMonitor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 460799554, data2: 9481, data3: 4559, data4: [148, 47, 0, 128, 41, 0, 67, 71] };
pub const LegacyDataCollectorSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946854, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyDataCollectorSetCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946855, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyTraceSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946856, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyTraceSessionCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946857, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LogFileItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 384588776, data2: 57235, data3: 16951, data4: [148, 228, 158, 233, 24, 17, 29, 113] };
pub const LogFiles: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657840637, data2: 63161, data3: 20249, data4: [165, 217, 226, 208, 104, 88, 75, 197] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const MAX_COUNTER_PATH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: ::windows_sys::core::PSTR,
    pub szReturnPathBuffer: ::windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: ::windows_sys::core::PWSTR,
    pub szReturnPathBuffer: ::windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows_sys::core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: ::windows_sys::core::PSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: ::windows_sys::core::PSTR,
    pub szObjectName: ::windows_sys::core::PSTR,
    pub szInstanceName: ::windows_sys::core::PSTR,
    pub szParentInstance: ::windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows_sys::core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: ::windows_sys::core::PWSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: ::windows_sys::core::PWSTR,
    pub szObjectName: ::windows_sys::core::PWSTR,
    pub szInstanceName: ::windows_sys::core::PWSTR,
    pub szParentInstance: ::windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: ::windows_sys::core::PSTR,
    pub szObjectName: ::windows_sys::core::PSTR,
    pub szInstanceName: ::windows_sys::core::PSTR,
    pub szParentInstance: ::windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: ::windows_sys::core::PWSTR,
    pub szObjectName: ::windows_sys::core::PWSTR,
    pub szInstanceName: ::windows_sys::core::PWSTR,
    pub szParentInstance: ::windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: ::windows_sys::core::PSTR,
    pub ObjectGUID: ::windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: ::windows_sys::core::PWSTR,
    pub ObjectGUID: ::windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_DLL_VERSION = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = 1280u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_VERSION: PDH_DLL_VERSION = 1283u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_FMT = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_DOUBLE: PDH_FMT = 512u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_LARGE: PDH_FMT = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FMT_LONG: PDH_FMT = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: ::windows_sys::core::PCSTR,
    pub WideStringValue: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_0 {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: ::windows_sys::core::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: ::windows_sys::core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_LOG = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_READ_ACCESS: PDH_LOG = 65536u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = 131072u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = 262144u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows_sys::core::PSTR,
    pub szDefaultDir: ::windows_sys::core::PSTR,
    pub szBaseFileName: ::windows_sys::core::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows_sys::core::PSTR,
    pub PdlCounterList: ::windows_sys::core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows_sys::core::PWSTR,
    pub szDefaultDir: ::windows_sys::core::PWSTR,
    pub szBaseFileName: ::windows_sys::core::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows_sys::core::PWSTR,
    pub PdlCounterList: ::windows_sys::core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_LOG_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MAX_SCALE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MIN_SCALE: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_MORE_DATA: i32 = -2147481646i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_DATA: i32 = -2147481643i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_PATH_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: ::windows_sys::core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: ::windows_sys::core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl ::core::marker::Copy for PDH_RAW_LOG_RECORD {}
impl ::core::clone::Clone for PDH_RAW_LOG_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_RETRY: i32 = -2147481644i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PDH_SELECT_DATA_SOURCE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_STATISTICS {}
impl ::core::clone::Clone for PDH_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl ::core::marker::Copy for PDH_TIME_INFO {}
impl ::core::clone::Clone for PDH_TIME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERFLIBREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ADD_COUNTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_INSTANCE: &str = "_Total";
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COLLECT_END: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COLLECT_START: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: ::windows_sys::core::GUID,
    pub ProviderGuid: ::windows_sys::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: ::windows_sys::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INSTANCE {}
impl ::core::clone::Clone for PERF_COUNTERSET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: ::windows_sys::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_COUNTER_AGGREGATE_FUNC = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_BLOCK {}
impl ::core::clone::Clone for PERF_COUNTER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DATA {}
impl ::core::clone::Clone for PERF_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(target_arch = "x86")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: ::windows_sys::core::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: ::windows_sys::core::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: ::windows_sys::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTIFIER {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: ::windows_sys::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTITY {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERF_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DATA_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DELTA_BASE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_DETAIL = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = 100u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = 200u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = 300u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = 400u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_DEFINITION {}
impl ::core::clone::Clone for PERF_INSTANCE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_HEADER {}
impl ::core::clone::Clone for PERF_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PERF_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pbuffer: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_MULTI_COUNTERS {}
impl ::core::clone::Clone for PERF_MULTI_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl ::core::marker::Copy for PERF_MULTI_INSTANCES {}
impl ::core::clone::Clone for PERF_MULTI_INSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NO_INSTANCES: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_NUMBER_HEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
#[cfg(target_arch = "x86")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: ::windows_sys::core::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: ::windows_sys::core::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PERF_PROVIDER_CONTEXT {}
impl ::core::clone::Clone for PERF_PROVIDER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_DWORD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_LARGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_STRING_BUFFER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_BUFFER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for PERF_STRING_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TEXT_ASCII: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TEXT_UNICODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TIMER_100NS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TIMER_TICK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_TEXT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_TYPE_ZERO: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_WILDCARD_INSTANCE: &str = "*";
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PLA_CABEXTRACT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filename: ::windows_sys::core::PCWSTR, context: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_CLOSE_PROC = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_COLLECT_PROC = ::core::option::Option<unsafe extern "system" fn(pvaluename: ::windows_sys::core::PCWSTR, ppdata: *mut *mut ::core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PM_OPEN_PROC = ::core::option::Option<unsafe extern "system" fn(pcontext: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PerfCounterDataType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_COUNTERSET: PerfCounterDataType = 6i32;
pub type PerfProviderHandle = isize;
pub type PerfQueryHandle = isize;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type PerfRegInfoType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = 5i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = 6i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = 7i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = 8i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = 9i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = 10i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type REAL_TIME_DATA_SOURCE_ID_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type ReportValueTypeConstants = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDefaultValue: ReportValueTypeConstants = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonCurrentValue: ReportValueTypeConstants = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonAverage: ReportValueTypeConstants = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonMinimum: ReportValueTypeConstants = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonMaximum: ReportValueTypeConstants = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type ResourcePolicy = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteLargest: ResourcePolicy = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaDeleteOldest: ResourcePolicy = 1i32;
pub const S_PDH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 81159000, data2: 50337, data3: 16795, data4: [128, 35, 35, 183, 57, 2, 222, 44] };
pub const ServerDataCollectorSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946865, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const ServerDataCollectorSetCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946866, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SourcePropPage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 217262753, data2: 30065, data3: 4560, data4: [147, 196, 0, 170, 0, 163, 221, 234] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type StreamMode = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFile: StreamMode = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRealTime: StreamMode = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBoth: StreamMode = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaBuffering: StreamMode = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type SysmonBatchReason = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchNone: SysmonBatchReason = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddFiles: SysmonBatchReason = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddCounters: SysmonBatchReason = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type SysmonDataType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataAvg: SysmonDataType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataMin: SysmonDataType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataMax: SysmonDataType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataTime: SysmonDataType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonDataCount: SysmonDataType = 5i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type SysmonFileType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileHtml: SysmonFileType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileReport: SysmonFileType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileCsv: SysmonFileType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileTsv: SysmonFileType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileBlg: SysmonFileType = 5i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileRetiredBlg: SysmonFileType = 6i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const sysmonFileGif: SysmonFileType = 7i32;
pub const SystemDataCollectorSet: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946886, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SystemDataCollectorSetCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946887, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SystemMonitor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3302152416, data2: 53725, data3: 4558, data4: [148, 15, 0, 128, 41, 0, 67, 71] };
pub const SystemMonitor2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2133874572, data2: 24376, data3: 17938, data4: [172, 254, 110, 208, 76, 123, 122, 248] };
pub const TraceDataProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946835, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceDataProviderCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946833, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946844, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceSessionCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58946864, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type ValueMapType = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaIndex: ValueMapType = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlag: ValueMapType = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFlagArray: ValueMapType = 3i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaValidation: ValueMapType = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub type WeekDays = i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaRunOnce: WeekDays = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSunday: WeekDays = 1i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaMonday: WeekDays = 2i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaTuesday: WeekDays = 4i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaWednesday: WeekDays = 8i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaThursday: WeekDays = 16i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaFriday: WeekDays = 32i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaSaturday: WeekDays = 64i32;
#[doc = "*Required features: `\"Win32_System_Performance\"`*"]
pub const plaEveryday: WeekDays = 127i32;
#[repr(C)]
pub struct _ICounterItemUnion {
    pub base__: ::windows_sys::core::IUnknown,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pdblvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, iwidth: i32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(this: *mut *mut Self, ilinestyle: i32) -> ::windows_sys::core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, iscale: i32) -> ::windows_sys::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, status: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut *mut Self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDataAt: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDataAt: usize,
}
#[repr(C)]
pub struct _ISystemMonitorUnion {
    pub base__: ::windows_sys::core::IUnknown,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, iappearance: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut *mut Self, iappearance: i32) -> ::windows_sys::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut *mut Self, iborderstyle: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut *mut Self, iborderstyle: i32) -> ::windows_sys::core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(this: *mut *mut Self, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut *mut Self, pfont: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(this: *mut *mut Self, ppicounters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, fvalue: f32) -> ::windows_sys::core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, pfvalue: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut *mut Self, edisplaytype: DisplayTypeConstants) -> ::windows_sys::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut *mut Self, pedisplaytype: *mut DisplayTypeConstants) -> ::windows_sys::core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGraphTitle: unsafe extern "system" fn(this: *mut *mut Self, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GraphTitle: unsafe extern "system" fn(this: *mut *mut Self, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GraphTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetYAxisLabel: unsafe extern "system" fn(this: *mut *mut Self, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetYAxisLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub YAxisLabel: unsafe extern "system" fn(this: *mut *mut Self, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    YAxisLabel: usize,
    pub CollectSample: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Counter: unsafe extern "system" fn(this: *mut *mut Self, iindex: i32, ppicounter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCounter: unsafe extern "system" fn(this: *mut *mut Self, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCounter: usize,
    pub DeleteCounter: unsafe extern "system" fn(this: *mut *mut Self, pctr: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogFileName: unsafe extern "system" fn(this: *mut *mut Self, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogFileName: unsafe extern "system" fn(this: *mut *mut Self, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogFileName: usize,
    pub SetLogViewStart: unsafe extern "system" fn(this: *mut *mut Self, starttime: f64) -> ::windows_sys::core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(this: *mut *mut Self, starttime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(this: *mut *mut Self, stoptime: f64) -> ::windows_sys::core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(this: *mut *mut Self, stoptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GridColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(this: *mut *mut Self, color: u32) -> ::windows_sys::core::HRESULT,
    pub Highlight: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(this: *mut *mut Self, ereportvaluetype: ReportValueTypeConstants) -> ::windows_sys::core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(this: *mut *mut Self, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows_sys::core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(this: *mut *mut Self, ivalue: i32) -> ::windows_sys::core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(this: *mut *mut Self, pivalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(this: *mut *mut Self, ppilogfiles: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(this: *mut *mut Self, edatasourcetype: DataSourceTypeConstants) -> ::windows_sys::core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(this: *mut *mut Self, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlDsnName: unsafe extern "system" fn(this: *mut *mut Self, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlDsnName: unsafe extern "system" fn(this: *mut *mut Self, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlDsnName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSqlLogSetName: unsafe extern "system" fn(this: *mut *mut Self, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSqlLogSetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SqlLogSetName: unsafe extern "system" fn(this: *mut *mut Self, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SqlLogSetName: usize,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(this: *mut *mut Self, bstate: i16) -> ::windows_sys::core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(this: *mut *mut Self, pbstate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(this: *mut *mut Self, bscroll: i16) -> ::windows_sys::core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(this: *mut *mut Self, pbscroll: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(this: *mut *mut Self, inewcount: i32) -> ::windows_sys::core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(this: *mut *mut Self, pidatapointcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(this: *mut *mut Self, bselectedcountersonly: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAs: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Relog: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Relog: usize,
    pub ClearData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(this: *mut *mut Self, starttime: f64, stoptime: f64) -> ::windows_sys::core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(this: *mut *mut Self, starttime: *mut f64, stoptime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(this: *mut *mut Self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadSettings: unsafe extern "system" fn(this: *mut *mut Self, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadSettings: usize,
}
