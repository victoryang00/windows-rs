#[cfg(feature = "System_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "System_Display")]
pub mod Display;
#[cfg(feature = "System_Implementation")]
pub mod Implementation;
#[cfg(feature = "System_Inventory")]
pub mod Inventory;
#[cfg(feature = "System_Power")]
pub mod Power;
#[cfg(feature = "System_Preview")]
pub mod Preview;
#[cfg(feature = "System_Profile")]
pub mod Profile;
#[cfg(feature = "System_RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "System_RemoteSystems")]
pub mod RemoteSystems;
#[cfg(feature = "System_Threading")]
pub mod Threading;
#[cfg(feature = "System_Update")]
pub mod Update;
#[cfg(feature = "System_UserProfile")]
pub mod UserProfile;
pub type AppActivationResult = *mut ::core::ffi::c_void;
pub type AppDiagnosticInfo = *mut ::core::ffi::c_void;
pub type AppDiagnosticInfoWatcher = *mut ::core::ffi::c_void;
pub type AppDiagnosticInfoWatcherEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppExecutionStateChangeResult = *mut ::core::ffi::c_void;
pub type AppMemoryReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppMemoryUsageLimitChangingEventArgs = *mut ::core::ffi::c_void;
pub type AppResourceGroupBackgroundTaskReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppResourceGroupInfo = *mut ::core::ffi::c_void;
pub type AppResourceGroupInfoWatcher = *mut ::core::ffi::c_void;
pub type AppResourceGroupInfoWatcherEventArgs = *mut ::core::ffi::c_void;
pub type AppResourceGroupInfoWatcherExecutionStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppResourceGroupMemoryReport = *mut ::core::ffi::c_void;
pub type AppResourceGroupStateReport = *mut ::core::ffi::c_void;
pub type AppUriHandlerHost = *mut ::core::ffi::c_void;
pub type AppUriHandlerRegistration = *mut ::core::ffi::c_void;
pub type AppUriHandlerRegistrationManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DispatcherQueue = *mut ::core::ffi::c_void;
pub type DispatcherQueueController = *mut ::core::ffi::c_void;
pub type DispatcherQueueHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DispatcherQueueShutdownStartingEventArgs = *mut ::core::ffi::c_void;
pub type DispatcherQueueTimer = *mut ::core::ffi::c_void;
pub type FolderLauncherOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppActivationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppActivationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1800571136, data2: 62574, data3: 20144, data4: [170, 108, 56, 175, 85, 124, 249, 237] };
}
#[repr(C)]
pub struct IAppDiagnosticInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3813189274, data2: 34953, data3: 19619, data4: [190, 7, 213, 255, 255, 95, 8, 4] };
}
#[repr(C)]
pub struct IAppDiagnosticInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResourceGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResourceGroups: usize,
    pub CreateResourceGroupWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3745971159, data2: 6426, data3: 17516, data4: [148, 115, 143, 188, 35, 116, 163, 84] };
}
#[repr(C)]
pub struct IAppDiagnosticInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfo3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3365258813, data2: 56673, data3: 19557, data4: [186, 189, 129, 161, 11, 79, 152, 21] };
}
#[repr(C)]
pub struct IAppDiagnosticInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoAsync: usize,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3462997439, data2: 4298, data3: 16584, data4: [169, 202, 197, 201, 101, 1, 134, 110] };
}
#[repr(C)]
pub struct IAppDiagnosticInfoStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, appusermodelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppUserModelId: usize,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfoStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 95570822, data2: 4096, data3: 19600, data4: [187, 159, 114, 53, 7, 28, 80, 254] };
}
#[repr(C)]
pub struct IAppDiagnosticInfoWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfoWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1968656496, data2: 467, data3: 18586, data4: [147, 37, 82, 249, 204, 110, 222, 10] };
}
#[repr(C)]
pub struct IAppDiagnosticInfoWatcherEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppDiagnosticInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1880606486, data2: 57818, data3: 19557, data4: [153, 223, 4, 109, 255, 91, 231, 26] };
}
#[repr(C)]
pub struct IAppExecutionStateChangeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppExecutionStateChangeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1862507504, data2: 63771, data3: 19960, data4: [174, 119, 48, 51, 204, 182, 145, 20] };
}
#[repr(C)]
pub struct IAppMemoryReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub PeakPrivateCommitUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCommitLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppMemoryReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1835348891, data2: 19823, data3: 17852, data4: [156, 94, 228, 155, 63, 242, 117, 141] };
}
#[repr(C)]
pub struct IAppMemoryReport2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpectedTotalCommitLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppMemoryReport2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1602172728, data2: 20919, data3: 17116, data4: [183, 237, 121, 186, 70, 210, 136, 87] };
}
#[repr(C)]
pub struct IAppMemoryUsageLimitChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub NewLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046322276, data2: 65226, data3: 19877, data4: [158, 64, 43, 198, 62, 253, 201, 121] };
}
#[repr(C)]
pub struct IAppResourceGroupBackgroundTaskReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Trigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupBackgroundTaskReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627500878, data2: 45149, data3: 16578, data4: [157, 193, 26, 79, 3, 158, 161, 32] };
}
#[repr(C)]
pub struct IAppResourceGroupInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IsShared: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBackgroundTaskReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBackgroundTaskReports: usize,
    pub GetMemoryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub GetProcessDiagnosticInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))]
    GetProcessDiagnosticInfos: usize,
    pub GetStateReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3105093498, data2: 59399, data3: 18932, data4: [132, 94, 123, 139, 220, 254, 142, 231] };
}
#[repr(C)]
pub struct IAppResourceGroupInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartSuspendAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSuspendAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartResumeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartResumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTerminateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTerminateAsync: usize,
}
impl ::windows_sys::core::Interface for IAppResourceGroupInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003144557, data2: 54021, data3: 19819, data4: [146, 247, 106, 253, 173, 114, 222, 220] };
}
#[repr(C)]
pub struct IAppResourceGroupInfoWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub ExecutionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecutionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecutionStateChanged: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupInfoWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3652231421, data2: 28250, data3: 19570, data4: [139, 23, 9, 254, 196, 162, 18, 189] };
}
#[repr(C)]
pub struct IAppResourceGroupInfoWatcherEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2054714935, data2: 25346, data3: 19759, data4: [191, 137, 28, 18, 208, 178, 166, 185] };
}
#[repr(C)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 467398103, data2: 65254, data3: 20436, data4: [152, 221, 233, 42, 44, 194, 153, 243] };
}
#[repr(C)]
pub struct IAppResourceGroupMemoryReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommitUsageLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub CommitUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppMemoryUsageLevel) -> ::windows_sys::core::HRESULT,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupMemoryReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747374257, data2: 32177, data3: 19537, data4: [162, 37, 127, 174, 45, 73, 228, 49] };
}
#[repr(C)]
pub struct IAppResourceGroupStateReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExecutionState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppResourceGroupExecutionState) -> ::windows_sys::core::HRESULT,
    pub EnergyQuotaState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppResourceGroupStateReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1384423192, data2: 12144, data3: 16950, data4: [171, 64, 208, 77, 176, 199, 185, 49] };
}
#[repr(C)]
pub struct IAppUriHandlerHost {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerHost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1565575877, data2: 37586, data3: 21513, data4: [181, 111, 127, 115, 225, 14, 164, 195] };
}
#[repr(C)]
pub struct IAppUriHandlerHost2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerHost2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 973860501, data2: 10724, data3: 20927, data4: [128, 149, 163, 192, 104, 227, 199, 42] };
}
#[repr(C)]
pub struct IAppUriHandlerHostFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerHostFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 628898966, data2: 52740, data3: 24472, data4: [150, 187, 62, 189, 62, 146, 117, 187] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAddedHostsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut *mut Self, hosts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAppAddedHostsAsync: usize,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1869852337, data2: 17769, data3: 23615, data4: [155, 160, 153, 18, 62, 234, 50, 195] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllHosts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllHosts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateHosts: unsafe extern "system" fn(this: *mut *mut Self, hosts: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateHosts: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3578637463, data2: 52025, data3: 24351, data4: [136, 62, 1, 133, 55, 48, 189, 109] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistrationManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryGetRegistration: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistrationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3861682770, data2: 44180, data3: 22352, data4: [172, 27, 108, 251, 111, 37, 2, 99] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistrationManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistrationManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3185560305, data2: 46362, data3: 24169, data4: [174, 253, 112, 136, 217, 242, 177, 35] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistrationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistrationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3587104159, data2: 22313, data3: 23414, data4: [161, 212, 2, 133, 242, 149, 193, 36] };
}
#[repr(C)]
pub struct IAppUriHandlerRegistrationManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForPackage: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForPackageForUser: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 351765369, data2: 26768, data3: 20608, data4: [144, 167, 152, 130, 74, 127, 7, 158] };
}
#[repr(C)]
pub struct IDateTimeSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetSystemDateTime: unsafe extern "system" fn(this: *mut *mut Self, utcdatetime: super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemDateTime: usize,
}
impl ::windows_sys::core::Interface for IDateTimeSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1562464465, data2: 18414, data3: 18603, data4: [165, 43, 159, 25, 84, 39, 141, 130] };
}
#[repr(C)]
pub struct IDispatcherQueue {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTimer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: DispatcherQueuePriority, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownCompleted: usize,
}
impl ::windows_sys::core::Interface for IDispatcherQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1614711012, data2: 41784, data3: 20478, data4: [164, 87, 165, 207, 185, 206, 184, 153] };
}
#[repr(C)]
pub struct IDispatcherQueue2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDispatcherQueue2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3357722183, data2: 12527, data3: 20590, data4: [189, 30, 166, 71, 174, 102, 117, 255] };
}
#[repr(C)]
pub struct IDispatcherQueueController {
    pub base__: ::windows_sys::core::IInspectable,
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownQueueAsync: usize,
}
impl ::windows_sys::core::Interface for IDispatcherQueueController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 586370662, data2: 20699, data3: 20022, data4: [169, 141, 97, 192, 27, 56, 77, 32] };
}
#[repr(C)]
pub struct IDispatcherQueueControllerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDispatcherQueueControllerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 174889184, data2: 20888, data3: 18850, data4: [163, 19, 63, 112, 209, 241, 60, 39] };
}
#[repr(C)]
pub struct IDispatcherQueueShutdownStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3295824972, data2: 65431, data3: 16576, data4: [162, 38, 204, 10, 170, 84, 94, 137] };
}
#[repr(C)]
pub struct IDispatcherQueueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDispatcherQueueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2842526679, data2: 37745, data3: 17687, data4: [146, 69, 208, 130, 74, 193, 44, 116] };
}
#[repr(C)]
pub struct IDispatcherQueueTimer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
}
impl ::windows_sys::core::Interface for IDispatcherQueueTimer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1609218845, data2: 41756, data3: 18215, data4: [177, 172, 55, 69, 70, 73, 213, 106] };
}
#[repr(C)]
pub struct IFolderLauncherOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ItemsToSelect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ItemsToSelect: usize,
}
impl ::windows_sys::core::Interface for IFolderLauncherOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3146891901, data2: 27527, data3: 17194, data4: [189, 4, 119, 108, 111, 95, 178, 171] };
}
#[repr(C)]
pub struct IKnownUserPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GuestHost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PrincipalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SessionInitiationProtocolUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownUserPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2002096410, data2: 28869, data3: 18661, data4: [182, 55, 91, 163, 68, 30, 78, 228] };
}
#[repr(C)]
pub struct IKnownUserPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AgeEnforcementRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownUserPropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1531250562, data2: 63008, data3: 22398, data4: [177, 179, 221, 86, 100, 77, 121, 177] };
}
#[repr(C)]
pub struct ILaunchUriResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LaunchUriStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Result: usize,
}
impl ::windows_sys::core::Interface for ILaunchUriResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3962022111, data2: 63189, data3: 17866, data4: [145, 58, 112, 164, 12, 92, 130, 33] };
}
#[repr(C)]
pub struct ILauncherOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TreatAsUntrusted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTreatAsUntrusted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisplayApplicationPicker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDisplayApplicationPicker: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub UI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILauncherOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3136954840, data2: 45169, data3: 19672, data4: [133, 62, 52, 18, 3, 229, 87, 211] };
}
#[repr(C)]
pub struct ILauncherOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
    #[cfg(feature = "Storage_Search")]
    pub SetNeighboringFilesQuery: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SetNeighboringFilesQuery: usize,
}
impl ::windows_sys::core::Interface for ILauncherOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1000378036, data2: 28224, data3: 19918, data4: [161, 163, 47, 83, 149, 10, 251, 73] };
}
#[repr(C)]
pub struct ILauncherOptions3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILauncherOptions3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4034332245, data2: 19299, data3: 20026, data4: [145, 7, 78, 104, 120, 65, 146, 58] };
}
#[repr(C)]
pub struct ILauncherOptions4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetLimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILauncherOptions4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4017082638, data2: 59131, data3: 18452, data4: [164, 78, 87, 232, 185, 217, 160, 27] };
}
#[repr(C)]
pub struct ILauncherStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for ILauncherStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 661737923, data2: 40510, data3: 17142, data4: [145, 164, 93, 253, 235, 35, 36, 81] };
}
#[repr(C)]
pub struct ILauncherStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersAsync: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersWithLaunchUriTypeAsync: unsafe extern "system" fn(this: *mut *mut Self, scheme: ::windows_sys::core::HSTRING, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersWithLaunchUriTypeAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindFileHandlersAsync: unsafe extern "system" fn(this: *mut *mut Self, extension: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindFileHandlersAsync: usize,
}
impl ::windows_sys::core::Interface for ILauncherStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1505374139, data2: 9419, data3: 19458, data4: [164, 196, 130, 148, 86, 157, 84, 241] };
}
#[repr(C)]
pub struct ILauncherStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for ILauncherStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 591552936, data2: 40371, data3: 18051, data4: [170, 66, 220, 111, 81, 211, 56, 71] };
}
#[repr(C)]
pub struct ILauncherStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindAppUriHandlersAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindAppUriHandlersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataForUserAsync: usize,
}
impl ::windows_sys::core::Interface for ILauncherStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3119284639, data2: 46501, data3: 16838, data4: [179, 179, 221, 27, 49, 120, 188, 242] };
}
#[repr(C)]
pub struct ILauncherStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathAsync: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, path: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsForUserAsync: usize,
}
impl ::windows_sys::core::Interface for ILauncherStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1529147268, data2: 55445, data3: 24554, data4: [145, 83, 26, 196, 154, 237, 155, 169] };
}
#[repr(C)]
pub struct ILauncherUIOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InvocationPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetInvocationPoint: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
    #[cfg(feature = "UI_Popups")]
    pub PreferredPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PreferredPlacement: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetPreferredPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetPreferredPlacement: usize,
}
impl ::windows_sys::core::Interface for ILauncherUIOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 455465582, data2: 35494, data3: 16873, data4: [130, 81, 65, 101, 245, 152, 95, 73] };
}
#[repr(C)]
pub struct ILauncherViewOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut *mut Self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
impl ::windows_sys::core::Interface for ILauncherViewOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2325424625, data2: 31911, data3: 18910, data4: [155, 211, 60, 91, 113, 132, 246, 22] };
}
#[repr(C)]
pub struct IMemoryManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppMemoryUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AppMemoryUsageLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AppMemoryUsageLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppMemoryUsageLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageLimitChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageLimitChanging: usize,
}
impl ::windows_sys::core::Interface for IMemoryManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1550591900, data2: 55242, data3: 18297, data4: [145, 136, 64, 87, 33, 156, 230, 76] };
}
#[repr(C)]
pub struct IMemoryManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAppMemoryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProcessMemoryReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1861104927, data2: 28002, data3: 16959, data4: [148, 121, 176, 31, 156, 159, 118, 105] };
}
#[repr(C)]
pub struct IMemoryManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TrySetAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 345725390, data2: 37549, data3: 20021, data4: [137, 235, 80, 223, 180, 192, 217, 28] };
}
#[repr(C)]
pub struct IMemoryManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExpectedAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3316205608, data2: 59470, data3: 18566, data4: [138, 13, 68, 179, 25, 14, 59, 114] };
}
#[repr(C)]
pub struct IProcessLauncherOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub StandardInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardOutput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardOutput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardError: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardError: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardError: usize,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessLauncherOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 813742543, data2: 62532, data3: 19075, data4: [190, 175, 165, 73, 160, 243, 34, 156] };
}
#[repr(C)]
pub struct IProcessLauncherResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExitCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessLauncherResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1414302004, data2: 34520, data3: 18833, data4: [142, 117, 236, 232, 164, 59, 107, 109] };
}
#[repr(C)]
pub struct IProcessLauncherStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsync: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::HSTRING, args: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsyncWithOptions: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::HSTRING, args: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsyncWithOptions: usize,
}
impl ::windows_sys::core::Interface for IProcessLauncherStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 866871015, data2: 11534, data3: 17547, data4: [166, 160, 193, 60, 56, 54, 208, 156] };
}
#[repr(C)]
pub struct IProcessMemoryReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrivateWorkingSetUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalWorkingSetUsage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessMemoryReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 141755816, data2: 39792, data3: 18306, data4: [135, 65, 58, 152, 43, 108, 229, 228] };
}
#[repr(C)]
pub struct IProtocolForResultsOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
impl ::windows_sys::core::Interface for IProtocolForResultsOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3582011706, data2: 28137, data3: 19752, data4: [147, 120, 248, 103, 130, 225, 130, 187] };
}
#[repr(C)]
pub struct IRemoteLauncherOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredAppIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredAppIds: usize,
}
impl ::windows_sys::core::Interface for IRemoteLauncherOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654611336, data2: 10385, data3: 19679, data4: [162, 214, 157, 255, 125, 2, 230, 147] };
}
#[repr(C)]
pub struct IRemoteLauncherStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut *mut Self, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut *mut Self, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    LaunchUriWithDataAsync: usize,
}
impl ::windows_sys::core::Interface for IRemoteLauncherStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621485203, data2: 41740, data3: 18615, data4: [159, 33, 5, 16, 38, 164, 229, 23] };
}
#[repr(C)]
pub struct IShutdownManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BeginShutdown: unsafe extern "system" fn(this: *mut *mut Self, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginShutdown: usize,
    pub CancelShutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShutdownManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1927432173, data2: 56667, data3: 19820, data4: [177, 208, 197, 122, 123, 187, 95, 148] };
}
#[repr(C)]
pub struct IShutdownManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPowerStateSupported: unsafe extern "system" fn(this: *mut *mut Self, powerstate: PowerState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub EnterPowerState: unsafe extern "system" fn(this: *mut *mut Self, powerstate: PowerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterPowerStateWithTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterPowerStateWithTimeSpan: usize,
}
impl ::windows_sys::core::Interface for IShutdownManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 258580527, data2: 39988, data3: 17351, data4: [168, 195, 112, 179, 10, 127, 117, 4] };
}
#[repr(C)]
pub struct ITimeZoneSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentTimeZoneDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTimeZoneDisplayNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTimeZoneDisplayNames: usize,
    pub CanChangeTimeZone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ChangeTimeZoneByDisplayName: unsafe extern "system" fn(this: *mut *mut Self, timezonedisplayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimeZoneSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2604346346, data2: 41217, data3: 16814, data4: [159, 189, 2, 135, 40, 186, 183, 61] };
}
#[repr(C)]
pub struct ITimeZoneSettingsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AutoUpdateTimeZoneAsync: unsafe extern "system" fn(this: *mut *mut Self, timeout: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoUpdateTimeZoneAsync: usize,
}
impl ::windows_sys::core::Interface for ITimeZoneSettingsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1432096184, data2: 14760, data3: 18938, data4: [180, 246, 162, 199, 252, 40, 66, 236] };
}
#[repr(C)]
pub struct IUser {
    pub base__: ::windows_sys::core::IInspectable,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AuthenticationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserAuthenticationStatus) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, desiredsize: UserPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
}
impl ::windows_sys::core::Interface for IUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3751421638, data2: 59206, data3: 19405, data4: [181, 212, 18, 1, 3, 196, 32, 155] };
}
#[repr(C)]
pub struct IUser2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CheckUserAgeConsentGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, consentgroup: UserAgeConsentGroup, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUserAgeConsentGroupAsync: usize,
}
impl ::windows_sys::core::Interface for IUser2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2562348584, data2: 42723, data3: 20878, data4: [137, 217, 211, 178, 177, 153, 26, 16] };
}
#[repr(C)]
pub struct IUserAuthenticationStatusChangeDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserAuthenticationStatusChangeDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2293601640, data2: 47920, data3: 17147, data4: [162, 112, 233, 144, 46, 64, 239, 167] };
}
#[repr(C)]
pub struct IUserAuthenticationStatusChangingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserAuthenticationStatus) -> ::windows_sys::core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserAuthenticationStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserAuthenticationStatusChangingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2349010728, data2: 42769, data3: 19486, data4: [171, 72, 4, 23, 156, 21, 147, 143] };
}
#[repr(C)]
pub struct IUserChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140794332, data2: 6342, data3: 18651, data4: [188, 153, 114, 79, 185, 32, 60, 204] };
}
#[repr(C)]
pub struct IUserChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedPropertyKinds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedPropertyKinds: usize,
}
impl ::windows_sys::core::Interface for IUserChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1798097732, data2: 28417, data3: 22028, data4: [151, 173, 252, 127, 50, 236, 88, 31] };
}
#[repr(C)]
pub struct IUserDeviceAssociationChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NewUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OldUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDeviceAssociationChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3172953964, data2: 47965, data3: 19835, data4: [165, 240, 200, 205, 17, 163, 141, 66] };
}
#[repr(C)]
pub struct IUserDeviceAssociationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindUserFromDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserDeviceAssociationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserDeviceAssociationChanged: usize,
}
impl ::windows_sys::core::Interface for IUserDeviceAssociationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2118721044, data2: 63578, data3: 19463, data4: [141, 169, 127, 227, 208, 84, 35, 67] };
}
#[repr(C)]
pub struct IUserPicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowGuestAccounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowGuestAccounts: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SuggestedSelectedUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedSelectedUser: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSingleUserAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleUserAsync: usize,
}
impl ::windows_sys::core::Interface for IUserPicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2102689800, data2: 61923, data3: 19052, data4: [141, 220, 169, 187, 15, 72, 138, 237] };
}
#[repr(C)]
pub struct IUserPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserPickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3727855836, data2: 32371, data3: 19958, data4: [161, 174, 77, 126, 202, 130, 180, 13] };
}
#[repr(C)]
pub struct IUserStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByType: unsafe extern "system" fn(this: *mut *mut Self, r#type: UserType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(this: *mut *mut Self, r#type: UserType, status: UserAuthenticationStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByTypeAndStatus: usize,
    pub GetFromId: unsafe extern "system" fn(this: *mut *mut Self, nonroamableid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 358527547, data2: 9258, data3: 17888, data4: [162, 233, 49, 113, 252, 106, 127, 221] };
}
#[repr(C)]
pub struct IUserStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1956871697, data2: 11957, data3: 17543, data4: [176, 213, 44, 103, 144, 224, 19, 233] };
}
#[repr(C)]
pub struct IUserWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
impl ::windows_sys::core::Interface for IUserWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 358527547, data2: 9258, data3: 17888, data4: [162, 233, 49, 113, 252, 106, 127, 187] };
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LaunchUriResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LauncherOptions = *mut ::core::ffi::c_void;
pub type LauncherUIOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProcessLauncherOptions = *mut ::core::ffi::c_void;
pub type ProcessLauncherResult = *mut ::core::ffi::c_void;
pub type ProcessMemoryReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProtocolForResultsOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteLauncherOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type User = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserAuthenticationStatusChangeDeferral = *mut ::core::ffi::c_void;
pub type UserAuthenticationStatusChangingEventArgs = *mut ::core::ffi::c_void;
pub type UserChangedEventArgs = *mut ::core::ffi::c_void;
pub type UserDeviceAssociationChangedEventArgs = *mut ::core::ffi::c_void;
pub type UserPicker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System\"`*"]
#[repr(transparent)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
