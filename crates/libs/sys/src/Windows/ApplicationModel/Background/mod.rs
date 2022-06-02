pub type ActivitySensorTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithWakeupCapability: Self = Self(1i32);
    pub const AllowedWithoutWakeupCapability: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
}
impl ::core::marker::Copy for AlarmAccessStatus {}
impl ::core::clone::Clone for AlarmAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastTrigger = *mut ::core::ffi::c_void;
pub type AppBroadcastTriggerProviderInfo = *mut ::core::ffi::c_void;
pub type ApplicationTrigger = *mut ::core::ffi::c_void;
pub type ApplicationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ApplicationTriggerResult {}
impl ::core::clone::Clone for ApplicationTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppointmentStoreNotificationTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundAccessRequestKind {}
impl ::core::clone::Clone for BackgroundAccessRequestKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: Self = Self(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const AlwaysAllowed: Self = Self(4i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(5i32);
    pub const DeniedBySystemPolicy: Self = Self(6i32);
    pub const DeniedByUser: Self = Self(7i32);
}
impl ::core::marker::Copy for BackgroundAccessStatus {}
impl ::core::clone::Clone for BackgroundAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundTaskBuilder = *mut ::core::ffi::c_void;
pub type BackgroundTaskCanceledEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: Self = Self(0i32);
    pub const Terminating: Self = Self(1i32);
    pub const LoggingOff: Self = Self(2i32);
    pub const ServicingUpdate: Self = Self(3i32);
    pub const IdleTask: Self = Self(4i32);
    pub const Uninstall: Self = Self(5i32);
    pub const ConditionLoss: Self = Self(6i32);
    pub const SystemPolicy: Self = Self(7i32);
    pub const QuietHoursEntered: Self = Self(8i32);
    pub const ExecutionTimeExceeded: Self = Self(9i32);
    pub const ResourceRevocation: Self = Self(10i32);
    pub const EnergySaver: Self = Self(11i32);
}
impl ::core::marker::Copy for BackgroundTaskCancellationReason {}
impl ::core::clone::Clone for BackgroundTaskCancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundTaskCompletedEventArgs = *mut ::core::ffi::c_void;
pub type BackgroundTaskCompletedEventHandler = *mut ::core::ffi::c_void;
pub type BackgroundTaskDeferral = *mut ::core::ffi::c_void;
pub type BackgroundTaskProgressEventArgs = *mut ::core::ffi::c_void;
pub type BackgroundTaskProgressEventHandler = *mut ::core::ffi::c_void;
pub type BackgroundTaskRegistration = *mut ::core::ffi::c_void;
pub type BackgroundTaskRegistrationGroup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTaskThrottleCounter {}
impl ::core::clone::Clone for BackgroundTaskThrottleCounter {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundWorkCostValue {}
impl ::core::clone::Clone for BackgroundWorkCostValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BluetoothLEAdvertisementPublisherTrigger = *mut ::core::ffi::c_void;
pub type BluetoothLEAdvertisementWatcherTrigger = *mut ::core::ffi::c_void;
pub type CachedFileUpdaterTrigger = *mut ::core::ffi::c_void;
pub type CachedFileUpdaterTriggerDetails = *mut ::core::ffi::c_void;
pub type ChatMessageNotificationTrigger = *mut ::core::ffi::c_void;
pub type ChatMessageReceivedNotificationTrigger = *mut ::core::ffi::c_void;
pub type CommunicationBlockingAppSetAsActiveTrigger = *mut ::core::ffi::c_void;
pub type ContactStoreNotificationTrigger = *mut ::core::ffi::c_void;
pub type ContentPrefetchTrigger = *mut ::core::ffi::c_void;
pub type ConversationalAgentTrigger = *mut ::core::ffi::c_void;
pub type CustomSystemEventTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for CustomSystemEventTriggerRecurrence {}
impl ::core::clone::Clone for CustomSystemEventTriggerRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceConnectionChangeTrigger = *mut ::core::ffi::c_void;
pub type DeviceManufacturerNotificationTrigger = *mut ::core::ffi::c_void;
pub type DeviceServicingTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const LowBattery: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceTriggerResult {}
impl ::core::clone::Clone for DeviceTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeviceUseTrigger = *mut ::core::ffi::c_void;
pub type DeviceWatcherTrigger = *mut ::core::ffi::c_void;
pub type EmailStoreNotificationTrigger = *mut ::core::ffi::c_void;
pub type GattCharacteristicNotificationTrigger = *mut ::core::ffi::c_void;
pub type GattServiceProviderTrigger = *mut ::core::ffi::c_void;
pub type GattServiceProviderTriggerResult = *mut ::core::ffi::c_void;
pub type GeovisitTrigger = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IActivitySensorTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SubscribedActivities: usize,
    pub ReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivitySensorTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, reportintervalinmilliseconds: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAlarmApplicationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AlarmAccessStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAppBroadcastTrigger: unsafe extern "system" fn(this: *mut *mut Self, providerkey: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastTriggerProviderInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLogoResource: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LogoResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetVideoKeyFrameInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetVideoKeyFrameInterval: usize,
    #[cfg(feature = "Foundation")]
    pub VideoKeyFrameInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoKeyFrameInterval: usize,
    pub SetMaxVideoBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxVideoBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxVideoWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxVideoWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxVideoHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxVideoHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[repr(C)]
pub struct IApplicationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBackgroundCondition {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForApplicationAsync: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForApplicationAsync: usize,
    pub RemoveAccess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveAccessForApplication: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundAccessStatus) -> ::windows_sys::core::HRESULT,
    pub GetAccessStatusForApplication: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut BackgroundAccessStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedaccess: BackgroundAccessRequestKind, reason: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindAsync: usize,
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindForModernStandbyAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedaccess: BackgroundAccessRequestKind, reason: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindForModernStandbyAsync: usize,
    pub GetAccessStatusForModernStandby: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundAccessStatus) -> ::windows_sys::core::HRESULT,
    pub GetAccessStatusForModernStandbyForApplication: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut BackgroundAccessStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTask {
    pub base__: ::windows_sys::core::IInspectable,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, taskinstance: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskBuilder {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTaskEntryPoint: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TaskEntryPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTrigger: unsafe extern "system" fn(this: *mut *mut Self, trigger: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddCondition: unsafe extern "system" fn(this: *mut *mut Self, condition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskBuilder2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCancelOnConditionLoss: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CancelOnConditionLoss: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskBuilder3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsNetworkRequested: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsNetworkRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskBuilder4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTaskGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskBuilder5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTaskEntryPointClsid: unsafe extern "system" fn(this: *mut *mut Self, taskentrypoint: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CheckResult: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub TriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut *mut Self, cancelhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
    pub SuspendedCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskInstance2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetThrottleCount: unsafe extern "system" fn(this: *mut *mut Self, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskInstance4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IBackgroundTaskProgressEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Progress: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgress: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgress: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self, canceltask: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskRegistration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Trigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskRegistration3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTaskGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTaskGroups: usize,
    pub GetTaskGroup: unsafe extern "system" fn(this: *mut *mut Self, groupid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IBackgroundWorkCostStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentBackgroundWorkCost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundWorkCostValue) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Advertisement: usize,
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredTransmitPowerLevelInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredTransmitPowerLevelInDBm: usize,
    pub UseExtendedFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseExtendedFormat: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOutOfRangeTimeout: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SetSignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    AdvertisementFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    SetAdvertisementFilter: usize,
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICachedFileUpdaterTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICachedFileUpdaterTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateTarget: usize,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateRequest: usize,
    pub CanRequestUserInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessageNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IChatMessageReceivedNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContactStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContentPrefetchTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub WaitInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitInterval: usize,
}
#[repr(C)]
pub struct IContentPrefetchTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, waitinterval: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[repr(C)]
pub struct ICustomSystemEventTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICustomSystemEventTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggerid: ::windows_sys::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceConnectionChangeTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDeviceManufacturerNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub TriggerQualifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TriggerQualifier: usize,
    #[cfg(feature = "deprecated")]
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OneShot: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDeviceManufacturerNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggerqualifier: ::windows_sys::core::HSTRING, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[repr(C)]
pub struct IDeviceServicingTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, expectedduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, expectedduration: super::super::Foundation::TimeSpan, arguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[repr(C)]
pub struct IDeviceUseTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, arguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[repr(C)]
pub struct IDeviceWatcherTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IEmailStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    EventTriggeringMode: usize,
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, characteristic: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Create: usize,
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub CreateWithEventTriggeringMode: unsafe extern "system" fn(this: *mut *mut Self, characteristic: *mut ::core::ffi::c_void, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))]
    CreateWithEventTriggeringMode: usize,
}
#[repr(C)]
pub struct IGattServiceProviderTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub SetAdvertisingParameters: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    SetAdvertisingParameters: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub AdvertisingParameters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    AdvertisingParameters: usize,
}
#[repr(C)]
pub struct IGattServiceProviderTriggerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Trigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    Error: usize,
}
#[repr(C)]
pub struct IGattServiceProviderTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, triggerid: ::windows_sys::core::HSTRING, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[repr(C)]
pub struct IGeovisitTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub MonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    MonitoringScope: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetMonitoringScope: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetMonitoringScope: usize,
}
#[repr(C)]
pub struct ILocationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LocationTriggerType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILocationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: LocationTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMaintenanceTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMaintenanceTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaProcessingTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[repr(C)]
pub struct INetworkOperatorHotspotAuthenticationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct INetworkOperatorNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhoneTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    TriggerType: usize,
}
#[repr(C)]
pub struct IPhoneTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    Create: usize,
}
#[repr(C)]
pub struct IPushNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRfcommConnectionTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub InboundConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    InboundConnection: usize,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub OutboundConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    OutboundConnection: usize,
    pub AllowMultipleConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowMultipleConnections: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    SetProtectionLevel: usize,
    #[cfg(feature = "Networking")]
    pub RemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    RemoteHostName: usize,
    #[cfg(feature = "Networking")]
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetRemoteHostName: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISensorDataThresholdTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Sensors")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, threshold: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))]
    Create: usize,
}
#[repr(C)]
pub struct ISmartCardTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_SmartCards")]
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    TriggerType: usize,
}
#[repr(C)]
pub struct ISmartCardTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_SmartCards")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    Create: usize,
}
#[repr(C)]
pub struct ISmsMessageReceivedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Sms")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    Create: usize,
}
#[repr(C)]
pub struct ISocketActivityTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, tracker: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, storagelibrary: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub CreateFromLibraries: unsafe extern "system" fn(this: *mut *mut Self, storagelibraries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    CreateFromLibraries: usize,
}
#[repr(C)]
pub struct ISystemCondition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConditionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemConditionType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemConditionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, conditiontype: SystemConditionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemTriggerType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: SystemTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimeTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimeTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToastNotificationActionTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserNotificationChangedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Notifications")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    Create: usize,
}
pub type LocationTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: Self = Self(0i32);
}
impl ::core::marker::Copy for LocationTriggerType {}
impl ::core::clone::Clone for LocationTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MaintenanceTrigger = *mut ::core::ffi::c_void;
pub type MediaProcessingTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaProcessingTriggerResult {}
impl ::core::clone::Clone for MediaProcessingTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandDeviceServiceNotificationTrigger = *mut ::core::ffi::c_void;
pub type MobileBroadbandPcoDataChangeTrigger = *mut ::core::ffi::c_void;
pub type MobileBroadbandPinLockStateChangeTrigger = *mut ::core::ffi::c_void;
pub type MobileBroadbandRadioStateChangeTrigger = *mut ::core::ffi::c_void;
pub type MobileBroadbandRegistrationStateChangeTrigger = *mut ::core::ffi::c_void;
pub type NetworkOperatorDataUsageTrigger = *mut ::core::ffi::c_void;
pub type NetworkOperatorHotspotAuthenticationTrigger = *mut ::core::ffi::c_void;
pub type NetworkOperatorNotificationTrigger = *mut ::core::ffi::c_void;
pub type PaymentAppCanMakePaymentTrigger = *mut ::core::ffi::c_void;
pub type PhoneTrigger = *mut ::core::ffi::c_void;
pub type PushNotificationTrigger = *mut ::core::ffi::c_void;
pub type RcsEndUserMessageAvailableTrigger = *mut ::core::ffi::c_void;
pub type RfcommConnectionTrigger = *mut ::core::ffi::c_void;
pub type SecondaryAuthenticationFactorAuthenticationTrigger = *mut ::core::ffi::c_void;
pub type SensorDataThresholdTrigger = *mut ::core::ffi::c_void;
pub type SmartCardTrigger = *mut ::core::ffi::c_void;
pub type SmsMessageReceivedTrigger = *mut ::core::ffi::c_void;
pub type SocketActivityTrigger = *mut ::core::ffi::c_void;
pub type StorageLibraryChangeTrackerTrigger = *mut ::core::ffi::c_void;
pub type StorageLibraryContentChangedTrigger = *mut ::core::ffi::c_void;
pub type SystemCondition = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: Self = Self(0i32);
    pub const UserPresent: Self = Self(1i32);
    pub const UserNotPresent: Self = Self(2i32);
    pub const InternetAvailable: Self = Self(3i32);
    pub const InternetNotAvailable: Self = Self(4i32);
    pub const SessionConnected: Self = Self(5i32);
    pub const SessionDisconnected: Self = Self(6i32);
    pub const FreeNetworkAvailable: Self = Self(7i32);
    pub const BackgroundWorkCostNotHigh: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemConditionType {}
impl ::core::clone::Clone for SystemConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemTrigger = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: Self = Self(0i32);
    pub const SmsReceived: Self = Self(1i32);
    pub const UserPresent: Self = Self(2i32);
    pub const UserAway: Self = Self(3i32);
    pub const NetworkStateChange: Self = Self(4i32);
    pub const ControlChannelReset: Self = Self(5i32);
    pub const InternetAvailable: Self = Self(6i32);
    pub const SessionConnected: Self = Self(7i32);
    pub const ServicingComplete: Self = Self(8i32);
    pub const LockScreenApplicationAdded: Self = Self(9i32);
    pub const LockScreenApplicationRemoved: Self = Self(10i32);
    pub const TimeZoneChange: Self = Self(11i32);
    pub const OnlineIdConnectedStateChange: Self = Self(12i32);
    pub const BackgroundWorkCostChange: Self = Self(13i32);
    pub const PowerStateChange: Self = Self(14i32);
    pub const DefaultSignInAccountChange: Self = Self(15i32);
}
impl ::core::marker::Copy for SystemTriggerType {}
impl ::core::clone::Clone for SystemTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TetheringEntitlementCheckTrigger = *mut ::core::ffi::c_void;
pub type TimeTrigger = *mut ::core::ffi::c_void;
pub type ToastNotificationActionTrigger = *mut ::core::ffi::c_void;
pub type ToastNotificationHistoryChangedTrigger = *mut ::core::ffi::c_void;
pub type UserNotificationChangedTrigger = *mut ::core::ffi::c_void;
