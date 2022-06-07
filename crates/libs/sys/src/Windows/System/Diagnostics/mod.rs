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
impl ::windows_sys::core::Interface for IDiagnosticActionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3261440662, data2: 59195, data3: 16535, data4: [178, 143, 52, 66, 240, 61, 216, 49] };
}
#[repr(C)]
pub struct IDiagnosticInvoker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Data_Json", feature = "Foundation"))]
    pub RunDiagnosticActionAsync: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Json", feature = "Foundation")))]
    RunDiagnosticActionAsync: usize,
}
impl ::windows_sys::core::Interface for IDiagnosticInvoker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 410724106, data2: 739, data3: 20358, data4: [132, 252, 253, 216, 146, 181, 148, 15] };
}
#[repr(C)]
pub struct IDiagnosticInvoker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RunDiagnosticActionFromStringAsync: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunDiagnosticActionFromStringAsync: usize,
}
impl ::windows_sys::core::Interface for IDiagnosticInvoker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820983388, data2: 5466, data3: 19282, data4: [168, 236, 7, 12, 68, 249, 80, 0] };
}
#[repr(C)]
pub struct IDiagnosticInvokerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDiagnosticInvokerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559943390, data2: 61788, data3: 17748, data4: [168, 19, 193, 19, 195, 136, 27, 9] };
}
#[repr(C)]
pub struct IProcessCpuUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessCpuUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 196813938, data2: 51391, data3: 16954, data4: [168, 16, 181, 89, 174, 67, 84, 226] };
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
impl ::windows_sys::core::Interface for IProcessCpuUsageReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2322439340, data2: 14727, data3: 20015, data4: [161, 25, 107, 95, 162, 20, 241, 180] };
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
impl ::windows_sys::core::Interface for IProcessDiagnosticInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3895504971, data2: 12302, data3: 20198, data4: [160, 171, 91, 95, 82, 49, 180, 52] };
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
impl ::windows_sys::core::Interface for IProcessDiagnosticInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2505624346, data2: 15627, data3: 18924, data4: [171, 112, 79, 122, 17, 40, 5, 222] };
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
impl ::windows_sys::core::Interface for IProcessDiagnosticInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 792834656, data2: 46239, data3: 17036, data4: [170, 14, 132, 116, 79, 73, 202, 149] };
}
#[repr(C)]
pub struct IProcessDiagnosticInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetForProcessId: unsafe extern "system" fn(this: *mut *mut Self, processid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessDiagnosticInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1250334871, data2: 39065, data3: 19012, data4: [162, 155, 9, 22, 99, 190, 9, 182] };
}
#[repr(C)]
pub struct IProcessDiskUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessDiskUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1524075517, data2: 32337, data3: 20051, data4: [191, 170, 90, 110, 225, 170, 187, 248] };
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
impl ::windows_sys::core::Interface for IProcessDiskUsageReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1075193853, data2: 21341, data3: 19487, data4: [129, 184, 218, 84, 225, 190, 99, 94] };
}
#[repr(C)]
pub struct IProcessMemoryUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessMemoryUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4111147675, data2: 33404, data3: 17079, data4: [176, 124, 14, 50, 98, 126, 107, 62] };
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
impl ::windows_sys::core::Interface for IProcessMemoryUsageReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3267853498, data2: 6481, data3: 18053, data4: [133, 50, 126, 116, 158, 207, 142, 235] };
}
#[repr(C)]
pub struct ISystemCpuUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemCpuUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1614263212, data2: 726, data3: 16948, data4: [131, 98, 127, 227, 173, 200, 31, 95] };
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
impl ::windows_sys::core::Interface for ISystemCpuUsageReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 740741298, data2: 38019, data3: 20322, data4: [171, 87, 130, 178, 157, 151, 25, 184] };
}
#[repr(C)]
pub struct ISystemDiagnosticInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub MemoryUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemDiagnosticInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2727411205, data2: 57331, data3: 16511, data4: [154, 27, 11, 43, 49, 124, 168, 0] };
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemDiagnosticInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3557076001, data2: 64637, data3: 17904, data4: [154, 63, 57, 32, 58, 237, 159, 126] };
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsArchitectureSupported: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::ProcessorArchitecture, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PreferredArchitecture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ProcessorArchitecture) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemDiagnosticInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2044645769, data2: 27385, data3: 19881, data4: [164, 34, 21, 247, 50, 85, 179, 235] };
}
#[repr(C)]
pub struct ISystemMemoryUsage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMemoryUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 402638229, data2: 5890, data3: 18895, data4: [170, 39, 47, 10, 50, 89, 20, 4] };
}
#[repr(C)]
pub struct ISystemMemoryUsageReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub TotalPhysicalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AvailableSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub CommittedSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMemoryUsageReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 946224263, data2: 10911, data3: 16442, data4: [189, 25, 44, 243, 232, 22, 149, 0] };
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
