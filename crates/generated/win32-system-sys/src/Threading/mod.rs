#[link(name = "windows")]
extern "system" {
    pub fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    pub fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    pub fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor: *mut ::win32_foundation_sys::HANDLE, integritylabel: ::win32_foundation_sys::PSID) -> ::win32_foundation_sys::BOOL;
    pub fn AddSIDToBoundaryDescriptor(boundarydescriptor: *mut ::win32_foundation_sys::HANDLE, requiredsid: ::win32_foundation_sys::PSID) -> ::win32_foundation_sys::BOOL;
    pub fn AttachThreadInput(idattach: u32, idattachto: u32, fattach: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn AvQuerySystemResponsiveness(avrthandle: ::win32_foundation_sys::HANDLE, systemresponsivenessvalue: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn AvRevertMmThreadCharacteristics(avrthandle: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtCreateThreadOrderingGroup(context: *mut ::win32_foundation_sys::HANDLE, period: *const i64, threadorderingguid: *mut ::windows_core_sys::GUID, timeout: *const i64) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtCreateThreadOrderingGroupExA(context: *mut ::win32_foundation_sys::HANDLE, period: *const i64, threadorderingguid: *mut ::windows_core_sys::GUID, timeout: *const i64, taskname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtCreateThreadOrderingGroupExW(context: *mut ::win32_foundation_sys::HANDLE, period: *const i64, threadorderingguid: *mut ::windows_core_sys::GUID, timeout: *const i64, taskname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtDeleteThreadOrderingGroup(context: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtJoinThreadOrderingGroup(context: *mut ::win32_foundation_sys::HANDLE, threadorderingguid: *const ::windows_core_sys::GUID, before: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtLeaveThreadOrderingGroup(context: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn AvRtWaitOnThreadOrderingGroup(context: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn AvSetMmMaxThreadCharacteristicsA(firsttask: ::windows_core_sys::PCSTR, secondtask: ::windows_core_sys::PCSTR, taskindex: *mut u32) -> ::win32_foundation_sys::HANDLE;
    pub fn AvSetMmMaxThreadCharacteristicsW(firsttask: ::windows_core_sys::PCWSTR, secondtask: ::windows_core_sys::PCWSTR, taskindex: *mut u32) -> ::win32_foundation_sys::HANDLE;
    pub fn AvSetMmThreadCharacteristicsA(taskname: ::windows_core_sys::PCSTR, taskindex: *mut u32) -> ::win32_foundation_sys::HANDLE;
    pub fn AvSetMmThreadCharacteristicsW(taskname: ::windows_core_sys::PCWSTR, taskindex: *mut u32) -> ::win32_foundation_sys::HANDLE;
    pub fn AvSetMmThreadPriority(avrthandle: ::win32_foundation_sys::HANDLE, priority: AVRT_PRIORITY) -> ::win32_foundation_sys::BOOL;
    pub fn CallbackMayRunLong(pci: *mut TP_CALLBACK_INSTANCE) -> ::win32_foundation_sys::BOOL;
    pub fn CancelThreadpoolIo(pio: *mut TP_IO);
    pub fn CancelWaitableTimer(htimer: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn ChangeTimerQueueTimer(timerqueue: ::win32_foundation_sys::HANDLE, timer: ::win32_foundation_sys::HANDLE, duetime: u32, period: u32) -> ::win32_foundation_sys::BOOL;
    pub fn ClosePrivateNamespace(handle: NamespaceHandle, flags: u32) -> ::win32_foundation_sys::BOOLEAN;
    pub fn CloseThreadpool(ptpp: PTP_POOL);
    pub fn CloseThreadpoolCleanupGroup(ptpcg: isize);
    pub fn CloseThreadpoolCleanupGroupMembers(ptpcg: isize, fcancelpendingcallbacks: ::win32_foundation_sys::BOOL, pvcleanupcontext: *mut ::core::ffi::c_void);
    pub fn CloseThreadpoolIo(pio: *mut TP_IO);
    pub fn CloseThreadpoolTimer(pti: *mut TP_TIMER);
    pub fn CloseThreadpoolWait(pwa: *mut TP_WAIT);
    pub fn CloseThreadpoolWork(pwk: *mut TP_WORK);
    pub fn ConvertFiberToThread() -> ::win32_foundation_sys::BOOL;
    pub fn ConvertThreadToFiber(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn ConvertThreadToFiberEx(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void;
    pub fn CreateBoundaryDescriptorA(name: ::windows_core_sys::PCSTR, flags: u32) -> BoundaryDescriptorHandle;
    pub fn CreateBoundaryDescriptorW(name: ::windows_core_sys::PCWSTR, flags: u32) -> BoundaryDescriptorHandle;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateEventA(lpeventattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, bmanualreset: ::win32_foundation_sys::BOOL, binitialstate: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateEventExA(lpeventattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpname: ::windows_core_sys::PCSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateEventExW(lpeventattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpname: ::windows_core_sys::PCWSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateEventW(lpeventattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, bmanualreset: ::win32_foundation_sys::BOOL, binitialstate: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMutexA(lpmutexattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binitialowner: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMutexExA(lpmutexattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpname: ::windows_core_sys::PCSTR, dwflags: u32, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMutexExW(lpmutexattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpname: ::windows_core_sys::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMutexW(lpmutexattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binitialowner: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreatePrivateNamespaceA(lpprivatenamespaceattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows_core_sys::PCSTR) -> NamespaceHandle;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreatePrivateNamespaceW(lpprivatenamespaceattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows_core_sys::PCWSTR) -> NamespaceHandle;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateProcessA(lpapplicationname: ::windows_core_sys::PCSTR, lpcommandline: ::windows_core_sys::PSTR, lpprocessattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binherithandles: ::win32_foundation_sys::BOOL, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCSTR, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateProcessAsUserA(htoken: ::win32_foundation_sys::HANDLE, lpapplicationname: ::windows_core_sys::PCSTR, lpcommandline: ::windows_core_sys::PSTR, lpprocessattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binherithandles: ::win32_foundation_sys::BOOL, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCSTR, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateProcessAsUserW(htoken: ::win32_foundation_sys::HANDLE, lpapplicationname: ::windows_core_sys::PCWSTR, lpcommandline: ::windows_core_sys::PWSTR, lpprocessattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binherithandles: ::win32_foundation_sys::BOOL, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateProcessW(lpapplicationname: ::windows_core_sys::PCWSTR, lpcommandline: ::windows_core_sys::PWSTR, lpprocessattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, binherithandles: ::win32_foundation_sys::BOOL, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn CreateProcessWithLogonW(lpusername: ::windows_core_sys::PCWSTR, lpdomain: ::windows_core_sys::PCWSTR, lppassword: ::windows_core_sys::PCWSTR, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: ::windows_core_sys::PCWSTR, lpcommandline: ::windows_core_sys::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn CreateProcessWithTokenW(htoken: ::win32_foundation_sys::HANDLE, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: ::windows_core_sys::PCWSTR, lpcommandline: ::windows_core_sys::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows_core_sys::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateRemoteThread(hprocess: ::win32_foundation_sys::HANDLE, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateRemoteThreadEx(hprocess: ::win32_foundation_sys::HANDLE, lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid: *mut u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateSemaphoreA(lpsemaphoreattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateSemaphoreExA(lpsemaphoreattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows_core_sys::PCSTR, dwflags: u32, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateSemaphoreExW(lpsemaphoreattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows_core_sys::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateSemaphoreW(lpsemaphoreattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateThread(lpthreadattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> ::win32_foundation_sys::HANDLE;
    pub fn CreateThreadpool(reserved: *mut ::core::ffi::c_void) -> PTP_POOL;
    pub fn CreateThreadpoolCleanupGroup() -> isize;
    pub fn CreateThreadpoolIo(fl: ::win32_foundation_sys::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO;
    pub fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER;
    pub fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT;
    pub fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK;
    pub fn CreateTimerQueue() -> ::win32_foundation_sys::HANDLE;
    pub fn CreateTimerQueueTimer(phnewtimer: *mut ::win32_foundation_sys::HANDLE, timerqueue: ::win32_foundation_sys::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateWaitableTimerExW(lptimerattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, lptimername: ::windows_core_sys::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateWaitableTimerW(lptimerattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES, bmanualreset: ::win32_foundation_sys::BOOL, lptimername: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn DeleteBoundaryDescriptor(boundarydescriptor: BoundaryDescriptorHandle);
    #[cfg(feature = "win32-system-sys")]
    pub fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    pub fn DeleteFiber(lpfiber: *const ::core::ffi::c_void);
    pub fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
    pub fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteTimerQueue(timerqueue: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteTimerQueueEx(timerqueue: ::win32_foundation_sys::HANDLE, completionevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteTimerQueueTimer(timerqueue: ::win32_foundation_sys::HANDLE, timer: ::win32_foundation_sys::HANDLE, completionevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn DisassociateCurrentThreadFromCallback(pci: *mut TP_CALLBACK_INSTANCE);
    #[cfg(feature = "win32-system-sys")]
    pub fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    pub fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> ::win32_foundation_sys::BOOL;
    pub fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn ExitProcess(uexitcode: u32) -> !;
    pub fn ExitThread(dwexitcode: u32) -> !;
    pub fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32;
    pub fn FlsFree(dwflsindex: u32) -> ::win32_foundation_sys::BOOL;
    pub fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void;
    pub fn FlsSetValue(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn FlushProcessWriteBuffers();
    pub fn FreeLibraryWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mod: ::win32_foundation_sys::HINSTANCE);
    pub fn GetActiveProcessorCount(groupnumber: u16) -> u32;
    pub fn GetActiveProcessorGroupCount() -> u16;
    pub fn GetCurrentProcess() -> ::win32_foundation_sys::HANDLE;
    pub fn GetCurrentProcessId() -> u32;
    pub fn GetCurrentProcessorNumber() -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
    pub fn GetCurrentThread() -> ::win32_foundation_sys::HANDLE;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize);
    pub fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void;
    pub fn GetExitCodeProcess(hprocess: ::win32_foundation_sys::HANDLE, lpexitcode: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetExitCodeThread(hthread: ::win32_foundation_sys::HANDLE, lpexitcode: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetGuiResources(hprocess: ::win32_foundation_sys::HANDLE, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32;
    pub fn GetMachineTypeAttributes(machine: u16, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> ::windows_core_sys::HRESULT;
    pub fn GetMaximumProcessorCount(groupnumber: u16) -> u32;
    pub fn GetMaximumProcessorGroupCount() -> u16;
    pub fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaNodeNumberFromHandle(hfile: ::win32_foundation_sys::HANDLE, nodenumber: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: *mut super::SystemInformation::GROUP_AFFINITY, processormaskcount: u16, requiredmaskcount: *mut u16) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetPriorityClass(hprocess: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn GetProcessAffinityMask(hprocess: ::win32_foundation_sys::HANDLE, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessDEPPolicy(hprocess: ::win32_foundation_sys::HANDLE, lpflags: *mut u32, lppermanent: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetProcessDefaultCpuSetMasks(process: ::win32_foundation_sys::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessDefaultCpuSets(process: ::win32_foundation_sys::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessGroupAffinity(hprocess: ::win32_foundation_sys::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessHandleCount(hprocess: ::win32_foundation_sys::HANDLE, pdwhandlecount: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessId(process: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn GetProcessIdOfThread(thread: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn GetProcessInformation(hprocess: ::win32_foundation_sys::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessIoCounters(hprocess: ::win32_foundation_sys::HANDLE, lpiocounters: *mut IO_COUNTERS) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessMitigationPolicy(hprocess: ::win32_foundation_sys::HANDLE, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessPriorityBoost(hprocess: ::win32_foundation_sys::HANDLE, pdisablepriorityboost: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessTimes(hprocess: ::win32_foundation_sys::HANDLE, lpcreationtime: *mut ::win32_foundation_sys::FILETIME, lpexittime: *mut ::win32_foundation_sys::FILETIME, lpkerneltime: *mut ::win32_foundation_sys::FILETIME, lpusertime: *mut ::win32_foundation_sys::FILETIME) -> ::win32_foundation_sys::BOOL;
    pub fn GetProcessVersion(processid: u32) -> u32;
    pub fn GetProcessWorkingSetSize(hprocess: ::win32_foundation_sys::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA);
    pub fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW);
    pub fn GetSystemTimes(lpidletime: *mut ::win32_foundation_sys::FILETIME, lpkerneltime: *mut ::win32_foundation_sys::FILETIME, lpusertime: *mut ::win32_foundation_sys::FILETIME) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadDescription(hthread: ::win32_foundation_sys::HANDLE, ppszthreaddescription: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetThreadGroupAffinity(hthread: ::win32_foundation_sys::HANDLE, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadIOPendingFlag(hthread: ::win32_foundation_sys::HANDLE, lpioispending: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadId(thread: ::win32_foundation_sys::HANDLE) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetThreadIdealProcessorEx(hthread: ::win32_foundation_sys::HANDLE, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadInformation(hthread: ::win32_foundation_sys::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadPriority(hthread: ::win32_foundation_sys::HANDLE) -> i32;
    pub fn GetThreadPriorityBoost(hthread: ::win32_foundation_sys::HANDLE, pdisablepriorityboost: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetThreadSelectedCpuSetMasks(thread: ::win32_foundation_sys::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadSelectedCpuSets(thread: ::win32_foundation_sys::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetThreadTimes(hthread: ::win32_foundation_sys::HANDLE, lpcreationtime: *mut ::win32_foundation_sys::FILETIME, lpexittime: *mut ::win32_foundation_sys::FILETIME, lpkerneltime: *mut ::win32_foundation_sys::FILETIME, lpusertime: *mut ::win32_foundation_sys::FILETIME) -> ::win32_foundation_sys::BOOL;
    pub fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn GetUmsSystemThreadInformation(threadhandle: ::win32_foundation_sys::HANDLE, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut ::win32_foundation_sys::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn InitOnceInitialize(initonce: *mut RTL_RUN_ONCE);
    pub fn InitializeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[cfg(feature = "win32-system-sys")]
    pub fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[cfg(feature = "win32-system-sys")]
    pub fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER);
    pub fn InitializeSRWLock(srwlock: *mut RTL_SRWLOCK);
    pub fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "win32-system-sys")]
    pub fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "win32-system-sys")]
    pub fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "win32-system-sys")]
    pub fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY;
    pub fn IsImmersiveProcess(hprocess: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn IsProcessCritical(hprocess: ::win32_foundation_sys::HANDLE, critical: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> ::win32_foundation_sys::BOOL;
    pub fn IsThreadAFiber() -> ::win32_foundation_sys::BOOL;
    pub fn IsThreadpoolTimerSet(pti: *mut TP_TIMER) -> ::win32_foundation_sys::BOOL;
    pub fn IsWow64Process(hprocess: ::win32_foundation_sys::HANDLE, wow64process: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn IsWow64Process2(hprocess: ::win32_foundation_sys::HANDLE, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[cfg(feature = "win32-system-sys")]
    pub fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION);
    pub fn NtQueryInformationProcess(processhandle: ::win32_foundation_sys::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> ::win32_foundation_sys::NTSTATUS;
    pub fn NtQueryInformationThread(threadhandle: ::win32_foundation_sys::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> ::win32_foundation_sys::NTSTATUS;
    pub fn NtSetInformationThread(threadhandle: ::win32_foundation_sys::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> ::win32_foundation_sys::NTSTATUS;
    pub fn OpenEventA(dwdesiredaccess: u32, binherithandle: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn OpenEventW(dwdesiredaccess: u32, binherithandle: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn OpenMutexW(dwdesiredaccess: u32, binherithandle: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn OpenPrivateNamespaceA(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows_core_sys::PCSTR) -> NamespaceHandle;
    pub fn OpenPrivateNamespaceW(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows_core_sys::PCWSTR) -> NamespaceHandle;
    pub fn OpenProcess(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: ::win32_foundation_sys::BOOL, dwprocessid: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn OpenProcessToken(processhandle: ::win32_foundation_sys::HANDLE, desiredaccess: ::win32_security_sys::TOKEN_ACCESS_MASK, tokenhandle: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn OpenSemaphoreW(dwdesiredaccess: u32, binherithandle: ::win32_foundation_sys::BOOL, lpname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn OpenThread(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: ::win32_foundation_sys::BOOL, dwthreadid: u32) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn OpenThreadToken(threadhandle: ::win32_foundation_sys::HANDLE, desiredaccess: ::win32_security_sys::TOKEN_ACCESS_MASK, openasself: ::win32_foundation_sys::BOOL, tokenhandle: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn OpenWaitableTimerW(dwdesiredaccess: u32, binherithandle: ::win32_foundation_sys::BOOL, lptimername: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::HANDLE;
    pub fn PulseEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
    pub fn QueryFullProcessImageNameA(hprocess: ::win32_foundation_sys::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows_core_sys::PSTR, lpdwsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn QueryFullProcessImageNameW(hprocess: ::win32_foundation_sys::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows_core_sys::PWSTR, lpdwsize: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn QueryProcessAffinityUpdateMode(hprocess: ::win32_foundation_sys::HANDLE, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn QueryProtectedPolicy(policyguid: *const ::windows_core_sys::GUID, policyvalue: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn QueryThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn QueueUserAPC(pfnapc: ::win32_foundation_sys::PAPCFUNC, hthread: ::win32_foundation_sys::HANDLE, dwdata: usize) -> u32;
    pub fn QueueUserAPC2(apcroutine: ::win32_foundation_sys::PAPCFUNC, thread: ::win32_foundation_sys::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn RegisterWaitForSingleObject(phnewwaitobject: *mut ::win32_foundation_sys::HANDLE, hobject: ::win32_foundation_sys::HANDLE, callback: WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn ReleaseMutex(hmutex: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn ReleaseMutexWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mut: ::win32_foundation_sys::HANDLE);
    pub fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    pub fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    pub fn ReleaseSemaphore(hsemaphore: ::win32_foundation_sys::HANDLE, lreleasecount: i32, lppreviouscount: *mut i32) -> ::win32_foundation_sys::BOOL;
    pub fn ReleaseSemaphoreWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, sem: ::win32_foundation_sys::HANDLE, crel: u32);
    pub fn ResetEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn ResumeThread(hthread: ::win32_foundation_sys::HANDLE) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32;
    pub fn SetEvent(hevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn SetEventWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, evt: ::win32_foundation_sys::HANDLE);
    pub fn SetPriorityClass(hprocess: ::win32_foundation_sys::HANDLE, dwpriorityclass: PROCESS_CREATION_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessAffinityMask(hprocess: ::win32_foundation_sys::HANDLE, dwprocessaffinitymask: usize) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessAffinityUpdateMode(hprocess: ::win32_foundation_sys::HANDLE, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetProcessDefaultCpuSetMasks(process: ::win32_foundation_sys::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessDefaultCpuSets(process: ::win32_foundation_sys::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessDynamicEHContinuationTargets(process: ::win32_foundation_sys::HANDLE, numberoftargets: u16, targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessDynamicEnforcedCetCompatibleRanges(process: ::win32_foundation_sys::HANDLE, numberofranges: u16, ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessInformation(hprocess: ::win32_foundation_sys::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessPriorityBoost(hprocess: ::win32_foundation_sys::HANDLE, bdisablepriorityboost: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessRestrictionExemption(fenableexemption: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetProcessWorkingSetSize(hprocess: ::win32_foundation_sys::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> ::win32_foundation_sys::BOOL;
    pub fn SetProtectedPolicy(policyguid: *const ::windows_core_sys::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadAffinityMask(hthread: ::win32_foundation_sys::HANDLE, dwthreadaffinitymask: usize) -> usize;
    pub fn SetThreadDescription(hthread: ::win32_foundation_sys::HANDLE, lpthreaddescription: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetThreadGroupAffinity(hthread: ::win32_foundation_sys::HANDLE, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadIdealProcessor(hthread: ::win32_foundation_sys::HANDLE, dwidealprocessor: u32) -> u32;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetThreadIdealProcessorEx(hthread: ::win32_foundation_sys::HANDLE, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadInformation(hthread: ::win32_foundation_sys::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadPriority(hthread: ::win32_foundation_sys::HANDLE, npriority: THREAD_PRIORITY) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadPriorityBoost(hthread: ::win32_foundation_sys::HANDLE, bdisablepriorityboost: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn SetThreadSelectedCpuSetMasks(thread: ::win32_foundation_sys::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadSelectedCpuSets(thread: ::win32_foundation_sys::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadToken(thread: *const ::win32_foundation_sys::HANDLE, token: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *const TP_POOL_STACK_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdmost: u32);
    pub fn SetThreadpoolThreadMinimum(ptpp: PTP_POOL, cthrdmic: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadpoolTimer(pti: *mut TP_TIMER, pftduetime: *const ::win32_foundation_sys::FILETIME, msperiod: u32, mswindowlength: u32);
    pub fn SetThreadpoolTimerEx(pti: *mut TP_TIMER, pftduetime: *const ::win32_foundation_sys::FILETIME, msperiod: u32, mswindowlength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetThreadpoolWait(pwa: *mut TP_WAIT, h: ::win32_foundation_sys::HANDLE, pfttimeout: *const ::win32_foundation_sys::FILETIME);
    pub fn SetThreadpoolWaitEx(pwa: *mut TP_WAIT, h: ::win32_foundation_sys::HANDLE, pfttimeout: *const ::win32_foundation_sys::FILETIME, reserved: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn SetTimerQueueTimer(timerqueue: ::win32_foundation_sys::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::HANDLE;
    pub fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetWaitableTimer(htimer: ::win32_foundation_sys::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn SetWaitableTimerEx(htimer: ::win32_foundation_sys::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> ::win32_foundation_sys::BOOL;
    pub fn Sleep(dwmilliseconds: u32);
    #[cfg(feature = "win32-system-sys")]
    pub fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> ::win32_foundation_sys::BOOL;
    pub fn SleepEx(dwmilliseconds: u32, balertable: ::win32_foundation_sys::BOOL) -> u32;
    pub fn StartThreadpoolIo(pio: *mut TP_IO);
    pub fn SubmitThreadpoolWork(pwk: *mut TP_WORK);
    pub fn SuspendThread(hthread: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void);
    pub fn SwitchToThread() -> ::win32_foundation_sys::BOOL;
    pub fn TerminateProcess(hprocess: ::win32_foundation_sys::HANDLE, uexitcode: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TerminateThread(hthread: ::win32_foundation_sys::HANDLE, dwexitcode: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TlsAlloc() -> u32;
    pub fn TlsFree(dwtlsindex: u32) -> ::win32_foundation_sys::BOOL;
    pub fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void;
    pub fn TlsSetValue(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> ::win32_foundation_sys::BOOLEAN;
    pub fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> ::win32_foundation_sys::BOOLEAN;
    #[cfg(feature = "win32-system-sys")]
    pub fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> ::win32_foundation_sys::BOOL;
    pub fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> ::win32_foundation_sys::BOOL;
    pub fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterWait(waithandle: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterWaitEx(waithandle: ::win32_foundation_sys::HANDLE, completionevent: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::BOOL;
    pub fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> ::win32_foundation_sys::BOOL;
    pub fn WaitForInputIdle(hprocess: ::win32_foundation_sys::HANDLE, dwmilliseconds: u32) -> u32;
    pub fn WaitForMultipleObjects(ncount: u32, lphandles: *const ::win32_foundation_sys::HANDLE, bwaitall: ::win32_foundation_sys::BOOL, dwmilliseconds: u32) -> u32;
    pub fn WaitForMultipleObjectsEx(ncount: u32, lphandles: *const ::win32_foundation_sys::HANDLE, bwaitall: ::win32_foundation_sys::BOOL, dwmilliseconds: u32, balertable: ::win32_foundation_sys::BOOL) -> u32;
    pub fn WaitForSingleObject(hhandle: ::win32_foundation_sys::HANDLE, dwmilliseconds: u32) -> u32;
    pub fn WaitForSingleObjectEx(hhandle: ::win32_foundation_sys::HANDLE, dwmilliseconds: u32, balertable: ::win32_foundation_sys::BOOL) -> u32;
    pub fn WaitForThreadpoolIoCallbacks(pio: *mut TP_IO, fcancelpendingcallbacks: ::win32_foundation_sys::BOOL);
    pub fn WaitForThreadpoolTimerCallbacks(pti: *mut TP_TIMER, fcancelpendingcallbacks: ::win32_foundation_sys::BOOL);
    pub fn WaitForThreadpoolWaitCallbacks(pwa: *mut TP_WAIT, fcancelpendingcallbacks: ::win32_foundation_sys::BOOL);
    pub fn WaitForThreadpoolWorkCallbacks(pwk: *mut TP_WORK, fcancelpendingcallbacks: ::win32_foundation_sys::BOOL);
    pub fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> ::win32_foundation_sys::BOOL;
    pub fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    pub fn WakeByAddressAll(address: *const ::core::ffi::c_void);
    pub fn WakeByAddressSingle(address: *const ::core::ffi::c_void);
    pub fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    pub fn WinExec(lpcmdline: ::windows_core_sys::PCSTR, ucmdshow: u32) -> u32;
    pub fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16;
    pub fn Wow64SuspendThread(hthread: ::win32_foundation_sys::HANDLE) -> u32;
}
#[repr(C)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AVRT_PRIORITY = i32;
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = -2i32;
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = -1i32;
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = 0i32;
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = 1i32;
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = 2i32;
pub type BoundaryDescriptorHandle = isize;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub type CREATE_EVENT = u32;
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = 2u32;
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = 1u32;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub type CREATE_PROCESS_LOGON_FLAGS = u32;
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = 1u32;
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = 2u32;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
pub type GET_GUI_RESOURCES_FLAGS = u32;
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = 0u32;
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 2u32;
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = 1u32;
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 4u32;
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[repr(C)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LPFIBER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpfiberparameter: *mut ::core::ffi::c_void)>;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut ::core::ffi::c_void;
pub type LPTHREAD_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
pub type MACHINE_ATTRIBUTES = u32;
pub const UserEnabled: MACHINE_ATTRIBUTES = 1u32;
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2u32;
pub const Wow64Container: MACHINE_ATTRIBUTES = 4u32;
pub type MEMORY_PRIORITY = u32;
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = 1u32;
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = 2u32;
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = 3u32;
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = 4u32;
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = 5u32;
#[repr(C)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
pub type NamespaceHandle = isize;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for PEB {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for PEB_LDR_DATA {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFLS_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(lpflsdata: *const ::core::ffi::c_void)>;
pub type PINIT_ONCE_FN = ::core::option::Option<unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL>;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub type POWER_REQUEST_CONTEXT_FLAGS = u32;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = 2u32;
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = 1u32;
pub type PPS_POST_PROCESS_INIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
pub type PROCESSINFOCLASS = i32;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0i32;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7i32;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26i32;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27i32;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29i32;
pub type PROCESSOR_FEATURE_ID = u32;
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = 25u32;
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 24u32;
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = 26u32;
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 27u32;
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = 18u32;
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 7u32;
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = 16u32;
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = 2u32;
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = 14u32;
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = 15u32;
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = 23u32;
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = 1u32;
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = 0u32;
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 3u32;
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = 12u32;
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = 9u32;
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 8u32;
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = 22u32;
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = 20u32;
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 13u32;
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = 21u32;
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 6u32;
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 10u32;
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = 17u32;
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 29u32;
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 30u32;
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 31u32;
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 34u32;
pub type PROCESS_ACCESS_RIGHTS = u32;
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = 1u32;
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = 2u32;
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = 4u32;
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = 8u32;
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = 16u32;
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = 32u32;
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = 64u32;
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = 128u32;
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = 256u32;
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = 512u32;
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = 1024u32;
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = 2048u32;
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 4096u32;
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 8192u32;
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = 2097151u32;
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = 65536u32;
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = 131072u32;
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = 262144u32;
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = 524288u32;
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = 1048576u32;
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = 983040u32;
pub type PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = u32;
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 0u32;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 1u32;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut ::core::ffi::c_void,
    pub PebBaseAddress: *mut PEB,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut ::core::ffi::c_void,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_CREATION_FLAGS = u32;
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = 1u32;
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = 2u32;
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = 4u32;
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = 8u32;
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = 16u32;
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32u32;
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 256u32;
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = 512u32;
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = 1024u32;
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = 2048u32;
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = 4096u32;
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = 8192u32;
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 16384u32;
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32768u32;
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = 65536u32;
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = 131072u32;
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = 262144u32;
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = 524288u32;
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = 1048576u32;
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = 2097152u32;
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = 4194304u32;
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = 16777216u32;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = 33554432u32;
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = 67108864u32;
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = 134217728u32;
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = 268435456u32;
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = 536870912u32;
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = 1073741824u32;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = 2147483648u32;
pub type PROCESS_DEP_FLAGS = u32;
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = 1u32;
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = 2u32;
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = 0u32;
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_INFORMATION {
    pub hProcess: ::win32_foundation_sys::HANDLE,
    pub hThread: ::win32_foundation_sys::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
impl ::core::marker::Copy for PROCESS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_INFORMATION_CLASS = i32;
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = 0i32;
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = 1i32;
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = 2i32;
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = 3i32;
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = 4i32;
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = 5i32;
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = 6i32;
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = 7i32;
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = 8i32;
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = 9i32;
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = 10i32;
#[repr(C)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0i32;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1i32;
pub type PROCESS_MITIGATION_POLICY = i32;
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = 0i32;
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = 1i32;
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = 2i32;
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = 3i32;
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = 4i32;
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = 5i32;
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = 6i32;
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = 7i32;
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = 8i32;
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = 9i32;
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = 10i32;
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = 11i32;
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = 12i32;
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = 13i32;
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = 14i32;
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = 15i32;
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = 16i32;
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = 17i32;
pub type PROCESS_NAME_FORMAT = u32;
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = 0u32;
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = 1u32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[repr(C)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PROCESS_PROTECTION_LEVEL = u32;
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = 0u32;
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = 1u32;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = 2u32;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = 3u32;
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = 4u32;
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = 5u32;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = 6u32;
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = 7u32;
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = 8u32;
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = 4294967294u32;
#[repr(C)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
pub type PROC_THREAD_ATTRIBUTE_NUM = u32;
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = 0u32;
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = 2u32;
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = 3u32;
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = 4u32;
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = 5u32;
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = 6u32;
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = 7u32;
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = 9u32;
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = 11u32;
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = 13u32;
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = 14u32;
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = 15u32;
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = 16u32;
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = 17u32;
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = 18u32;
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = 22u32;
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = 24u32;
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = 25u32;
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = 26u32;
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = 27u32;
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
#[cfg(feature = "win32-system-sys")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = ::core::option::Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void)>;
pub type PTIMERAPCROUTINE = ::core::option::Option<unsafe extern "system" fn(lpargtocompletionroutine: *const ::core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(objectcontext: *mut ::core::ffi::c_void, cleanupcontext: *mut ::core::ffi::c_void)>;
pub type PTP_POOL = isize;
pub type PTP_SIMPLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void)>;
pub type PTP_TIMER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, timer: *mut TP_TIMER)>;
pub type PTP_WAIT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, wait: *mut TP_WAIT, waitresult: u32)>;
pub type PTP_WIN32_IO_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, overlapped: *mut ::core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut TP_IO)>;
pub type PTP_WORK_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, work: *mut TP_WORK)>;
pub type QUEUE_USER_APC_FLAGS = i32;
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = 0i32;
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = 1i32;
#[repr(C)]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
impl ::core::marker::Copy for REASON_CONTEXT {}
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: ::win32_foundation_sys::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: ::win32_foundation_sys::HANDLE,
    pub LockSemaphore: ::win32_foundation_sys::HANDLE,
    pub SpinCount: usize,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RTL_UMS_THREAD_INFO_CLASS = i32;
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = 0i32;
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = 1i32;
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = 2i32;
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = 3i32;
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = 4i32;
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = 5i32;
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = 6i32;
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = 7i32;
#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::core::ffi::c_void; 10],
    pub ImagePathName: ::win32_foundation_sys::UNICODE_STRING,
    pub CommandLine: ::win32_foundation_sys::UNICODE_STRING,
}
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: ::windows_core_sys::PSTR,
    pub lpDesktop: ::windows_core_sys::PSTR,
    pub lpTitle: ::windows_core_sys::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: ::win32_foundation_sys::HANDLE,
    pub hStdOutput: ::win32_foundation_sys::HANDLE,
    pub hStdError: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for STARTUPINFOA {}
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl ::core::marker::Copy for STARTUPINFOEXA {}
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl ::core::marker::Copy for STARTUPINFOEXW {}
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: ::windows_core_sys::PWSTR,
    pub lpDesktop: ::windows_core_sys::PWSTR,
    pub lpTitle: ::windows_core_sys::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: ::win32_foundation_sys::HANDLE,
    pub hStdOutput: ::win32_foundation_sys::HANDLE,
    pub hStdError: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for STARTUPINFOW {}
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STARTUPINFOW_FLAGS = u32;
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = 64u32;
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = 128u32;
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = 8192u32;
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = 32u32;
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = 4096u32;
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = 2048u32;
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = 32768u32;
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = 8u32;
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = 16u32;
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = 512u32;
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = 4u32;
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = 1u32;
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = 2u32;
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = 256u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub type THREADINFOCLASS = i32;
pub const ThreadIsIoPending: THREADINFOCLASS = 16i32;
pub const ThreadNameInformation: THREADINFOCLASS = 38i32;
pub type THREAD_ACCESS_RIGHTS = u32;
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = 1u32;
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = 2u32;
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = 8u32;
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = 16u32;
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = 32u32;
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = 64u32;
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = 128u32;
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = 256u32;
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = 512u32;
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 1024u32;
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 2048u32;
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = 4096u32;
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = 2097151u32;
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = 65536u32;
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = 131072u32;
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = 262144u32;
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = 524288u32;
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = 1048576u32;
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = 983040u32;
pub type THREAD_CREATION_FLAGS = u32;
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = 0u32;
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = 4u32;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = 65536u32;
pub type THREAD_INFORMATION_CLASS = i32;
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = 0i32;
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = 1i32;
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = 2i32;
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = 3i32;
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = 4i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[repr(C)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
pub type THREAD_PRIORITY = i32;
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = 65536i32;
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = 131072i32;
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = 1i32;
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = -1i32;
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = 2i32;
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = -15i32;
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = 0i32;
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = 15i32;
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: isize,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut ::core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
#[repr(C)]
pub union TP_CALLBACK_ENVIRON_V3_1 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_1_0,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2i32;
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = 3i32;
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = 3i32;
#[repr(C)]
pub struct TP_IO(pub u8);
#[repr(C)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_TIMER(pub u8);
#[repr(C)]
pub struct TP_WAIT(pub u8);
#[repr(C)]
pub struct TP_WORK(pub u8);
pub type TimerQueueHandle = isize;
#[repr(C)]
#[cfg(feature = "win32-system-sys")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut ::core::ffi::c_void,
}
#[cfg(feature = "win32-system-sys")]
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "win32-system-sys")]
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WAITORTIMERCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: ::win32_foundation_sys::BOOLEAN)>;
pub const WAIT_ABANDONED: u32 = 128u32;
pub const WAIT_ABANDONED_0: u32 = 128u32;
pub const WAIT_IO_COMPLETION: u32 = 192u32;
pub const WAIT_OBJECT_0: u32 = 0u32;
pub type WORKER_THREAD_FLAGS = u32;
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = 0u32;
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = 1u32;
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = 128u32;
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = 4u32;
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = 16u32;
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = 8u32;
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = 256u32;
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = 32u32;
