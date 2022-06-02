#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
pub type DiagnosticActionResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Diagnostics\"`*"]
#[repr(transparent)]
pub struct DiagnosticActionState(pub i32);
impl DiagnosticActionState {
    pub const Initializing: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const VerifyingTrust: Self = Self(2i32);
    pub const Detecting: Self = Self(3i32);
    pub const Resolving: Self = Self(4i32);
    pub const VerifyingResolution: Self = Self(5i32);
    pub const Executing: Self = Self(6i32);
}
impl ::core::marker::Copy for DiagnosticActionState {}
impl ::core::clone::Clone for DiagnosticActionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DiagnosticInvoker = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDiagnosticActionResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Results: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Results: usize,
}
#[repr(C)]
pub struct IDiagnosticInvoker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))]
    pub RunDiagnosticActionAsync: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Json", feature = "Foundation")))]
    RunDiagnosticActionAsync: usize,
}
#[repr(C)]
pub struct IDiagnosticInvoker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RunDiagnosticActionFromStringAsync: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunDiagnosticActionFromStringAsync: usize,
}
#[repr(C)]
pub struct IDiagnosticInvokerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessCpuUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessCpuUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub KernelTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KernelTime: usize,
    #[cfg(feature = "Foundation")]
    pub UserTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserTime: usize,
}
#[repr(C)]
pub struct IProcessDiagnosticInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessStartTime: usize,
    pub DiskUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MemoryUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessDiagnosticInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppDiagnosticInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppDiagnosticInfos: usize,
    pub IsPackaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessDiagnosticInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForProcesses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForProcesses: usize,
    pub GetForCurrentProcess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessDiagnosticInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetForProcessId: unsafe extern "system" fn(this: *mut *mut Self, processid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessDiskUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessDiskUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReadOperationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub WriteOperationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub OtherOperationCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub BytesReadCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub BytesWrittenCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub OtherBytesCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessMemoryUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessMemoryUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub NonPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PageFaultCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PageFileSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakNonPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakPageFileSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakPagedPoolSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakVirtualMemorySizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakWorkingSetSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PrivatePageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub VirtualMemorySizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub WorkingSetSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemCpuUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemCpuUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub KernelTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KernelTime: usize,
    #[cfg(feature = "Foundation")]
    pub UserTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserTime: usize,
    #[cfg(feature = "Foundation")]
    pub IdleTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IdleTime: usize,
}
#[repr(C)]
pub struct ISystemDiagnosticInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub MemoryUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsArchitectureSupported: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PreferredArchitecture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ProcessorArchitecture) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemMemoryUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemMemoryUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub TotalPhysicalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AvailableSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub CommittedSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
pub type ProcessCpuUsage = *mut ::core::ffi::c_void;
pub type ProcessCpuUsageReport = *mut ::core::ffi::c_void;
pub type ProcessDiagnosticInfo = *mut ::core::ffi::c_void;
pub type ProcessDiskUsage = *mut ::core::ffi::c_void;
pub type ProcessDiskUsageReport = *mut ::core::ffi::c_void;
pub type ProcessMemoryUsage = *mut ::core::ffi::c_void;
pub type ProcessMemoryUsageReport = *mut ::core::ffi::c_void;
pub type SystemCpuUsage = *mut ::core::ffi::c_void;
pub type SystemCpuUsageReport = *mut ::core::ffi::c_void;
pub type SystemDiagnosticInfo = *mut ::core::ffi::c_void;
pub type SystemMemoryUsage = *mut ::core::ffi::c_void;
pub type SystemMemoryUsageReport = *mut ::core::ffi::c_void;
