#[link(name = "windows")]
extern "system" {
    pub fn HcsAttachLayerStorageFilter(layerpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows_core_sys::HRESULT;
    pub fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
    pub fn HcsCloseOperation(operation: HCS_OPERATION);
    pub fn HcsCloseProcess(process: HCS_PROCESS);
    pub fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Security")]
    pub fn HcsCreateComputeSystem(id: ::windows_core_sys::PCWSTR, configuration: ::windows_core_sys::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const ::win32_security_sys::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows_core_sys::HRESULT;
    pub fn HcsCreateComputeSystemInNamespace(idnamespace: ::windows_core_sys::PCWSTR, id: ::windows_core_sys::PCWSTR, configuration: ::windows_core_sys::PCWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows_core_sys::HRESULT;
    pub fn HcsCreateEmptyGuestStateFile(gueststatefilepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION;
    #[cfg(feature = "Win32_Security")]
    pub fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: ::windows_core_sys::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const ::win32_security_sys::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows_core_sys::HRESULT;
    pub fn HcsDestroyLayer(layerpath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsDetachLayerStorageFilter(layerpath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsEnumerateComputeSystems(query: ::windows_core_sys::PCWSTR, operation: HCS_OPERATION) -> ::windows_core_sys::HRESULT;
    pub fn HcsEnumerateComputeSystemsInNamespace(idnamespace: ::windows_core_sys::PCWSTR, query: ::windows_core_sys::PCWSTR, operation: HCS_OPERATION) -> ::windows_core_sys::HRESULT;
    pub fn HcsExportLayer(layerpath: ::windows_core_sys::PCWSTR, exportfolderpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsExportLegacyWritableLayer(writablelayermountpath: ::windows_core_sys::PCWSTR, writablelayerfolderpath: ::windows_core_sys::PCWSTR, exportfolderpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsFormatWritableLayerVhd(vhdhandle: ::win32_foundation_sys::HANDLE) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
    pub fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetLayerVhdMountPath(vhdhandle: ::win32_foundation_sys::HANDLE, mountpath: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
    pub fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
    pub fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
    pub fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
    pub fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: ::windows_core_sys::PCWSTR, processorfeaturesstring: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGetServiceProperties(propertyquery: ::windows_core_sys::PCWSTR, result: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGrantVmAccess(vmid: ::windows_core_sys::PCWSTR, filepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsGrantVmGroupAccess(filepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsImportLayer(layerpath: ::windows_core_sys::PCWSTR, sourcefolderpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsInitializeLegacyWritableLayer(writablelayermountpath: ::windows_core_sys::PCWSTR, writablelayerfolderpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsInitializeWritableLayer(writablelayerpath: ::windows_core_sys::PCWSTR, layerdata: ::windows_core_sys::PCWSTR, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: ::windows_core_sys::PCWSTR, identity: ::win32_foundation_sys::HANDLE) -> ::windows_core_sys::HRESULT;
    pub fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsModifyServiceSettings(settings: ::windows_core_sys::PCWSTR, result: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsOpenComputeSystem(id: ::windows_core_sys::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_core_sys::HRESULT;
    pub fn HcsOpenComputeSystemInNamespace(idnamespace: ::windows_core_sys::PCWSTR, id: ::windows_core_sys::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_core_sys::HRESULT;
    pub fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows_core_sys::HRESULT;
    pub fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsRevokeVmAccess(vmid: ::windows_core_sys::PCWSTR, filepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsRevokeVmGroupAccess(filepath: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetupBaseOSLayer(layerpath: ::windows_core_sys::PCWSTR, vhdhandle: ::win32_foundation_sys::HANDLE, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsSetupBaseOSVolume(layerpath: ::windows_core_sys::PCWSTR, volumepath: ::windows_core_sys::PCWSTR, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsSubmitWerReport(settings: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
}
pub type HCS_CREATE_OPTIONS = i32;
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = 65536i32;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: ::win32_foundation_sys::HANDLE,
    pub SecurityDescriptor: *mut ::win32_security_sys::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub Callback: HCS_EVENT_CALLBACK,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for HCS_CREATE_OPTIONS_1 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for HCS_CREATE_OPTIONS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: ::windows_core_sys::PCWSTR,
    pub Operation: HCS_OPERATION,
}
impl ::core::marker::Copy for HCS_EVENT {}
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void)>;
pub type HCS_EVENT_OPTIONS = u32;
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = 0u32;
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = 1u32;
pub type HCS_EVENT_TYPE = i32;
pub const HcsEventInvalid: HCS_EVENT_TYPE = 0i32;
pub const HcsEventSystemExited: HCS_EVENT_TYPE = 1i32;
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = 2i32;
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = 3i32;
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = 4i32;
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = 5i32;
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = 6i32;
pub const HcsEventProcessExited: HCS_EVENT_TYPE = 65536i32;
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = 16777216i32;
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = 33554432i32;
pub type HCS_NOTIFICATIONS = i32;
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = 0i32;
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = 1i32;
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = 2i32;
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = 3i32;
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = 4i32;
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = 5i32;
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = 6i32;
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = 7i32;
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = 8i32;
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = 9i32;
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = 10i32;
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = 10i32;
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = 11i32;
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = 12i32;
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = 13i32;
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = 14i32;
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = 15i32;
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = 16i32;
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = 65536i32;
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = 16777216i32;
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = -268435456i32;
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_core_sys::HRESULT, notificationdata: ::windows_core_sys::PCWSTR)>;
pub type HCS_NOTIFICATION_FLAGS = i32;
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = 0i32;
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = -2147483648i32;
pub type HCS_OPERATION = isize;
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
pub type HCS_OPERATION_TYPE = i32;
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = -1i32;
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = 0i32;
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = 1i32;
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = 2i32;
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = 3i32;
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = 4i32;
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = 5i32;
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = 6i32;
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = 7i32;
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = 8i32;
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = 9i32;
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = 10i32;
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = 11i32;
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = 12i32;
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = 13i32;
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = 14i32;
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = 15i32;
pub type HCS_PROCESS = isize;
#[repr(C)]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: ::win32_foundation_sys::HANDLE,
    pub StdOutput: ::win32_foundation_sys::HANDLE,
    pub StdError: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for HCS_PROCESS_INFORMATION {}
impl ::core::clone::Clone for HCS_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HCS_SYSTEM = isize;
