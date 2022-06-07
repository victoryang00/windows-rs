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
impl ::windows_sys::core::Interface for IActivitySensorTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3504161602, data2: 58235, data3: 18467, data4: [165, 254, 107, 49, 223, 239, 222, 176] };
}
#[repr(C)]
pub struct IActivitySensorTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, reportintervalinmilliseconds: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivitySensorTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2804322755, data2: 14391, data3: 17655, data4: [131, 27, 1, 50, 204, 135, 43, 195] };
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
impl ::windows_sys::core::Interface for IAlarmApplicationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3389258299, data2: 52454, data3: 19938, data4: [176, 155, 150, 40, 189, 51, 187, 190] };
}
#[repr(C)]
pub struct IAppBroadcastTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProviderInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1960113302, data2: 36151, data3: 17644, data4: [148, 129, 42, 11, 152, 84, 235, 72] };
}
#[repr(C)]
pub struct IAppBroadcastTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAppBroadcastTrigger: unsafe extern "system" fn(this: *mut *mut Self, providerkey: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 671850308, data2: 8948, data3: 17944, data4: [160, 46, 231, 228, 17, 235, 114, 56] };
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
impl ::windows_sys::core::Interface for IAppBroadcastTriggerProviderInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061738285, data2: 40424, data3: 17440, data4: [156, 226, 94, 255, 143, 23, 55, 107] };
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
impl ::windows_sys::core::Interface for IApplicationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 189171248, data2: 38260, data3: 18732, data4: [158, 147, 26, 58, 230, 51, 95, 233] };
}
#[repr(C)]
pub struct IApplicationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
impl ::windows_sys::core::Interface for IApplicationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547804850, data2: 8729, data3: 19102, data4: [156, 94, 65, 208, 71, 247, 110, 130] };
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IAppointmentStoreNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1691616268, data2: 49665, data3: 17069, data4: [170, 42, 226, 27, 163, 66, 91, 109] };
}
#[repr(C)]
pub struct IBackgroundCondition {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IBackgroundCondition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2923995630, data2: 35153, data3: 16394, data4: [131, 2, 156, 156, 154, 42, 58, 59] };
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
impl ::windows_sys::core::Interface for IBackgroundExecutionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3894864472, data2: 26281, data3: 19777, data4: [131, 212, 180, 193, 140, 135, 184, 70] };
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedaccess: BackgroundAccessRequestKind, reason: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindAsync: usize,
}
impl ::windows_sys::core::Interface for IBackgroundExecutionManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1184572655, data2: 39867, data3: 19992, data4: [153, 154, 253, 101, 18, 147, 27, 233] };
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
impl ::windows_sys::core::Interface for IBackgroundExecutionManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2561004534, data2: 23077, data3: 23404, data4: [145, 146, 215, 122, 67, 223, 237, 196] };
}
#[repr(C)]
pub struct IBackgroundTask {
    pub base__: ::windows_sys::core::IInspectable,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, taskinstance: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2098451764, data2: 64786, data3: 17358, data4: [140, 34, 234, 31, 241, 60, 6, 223] };
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
impl ::windows_sys::core::Interface for IBackgroundTaskBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 55661838, data2: 15972, data3: 17778, data4: [169, 58, 132, 7, 90, 55, 201, 23] };
}
#[repr(C)]
pub struct IBackgroundTaskBuilder2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCancelOnConditionLoss: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CancelOnConditionLoss: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskBuilder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1793576881, data2: 4175, data3: 16493, data4: [141, 182, 132, 74, 87, 15, 66, 187] };
}
#[repr(C)]
pub struct IBackgroundTaskBuilder3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsNetworkRequested: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsNetworkRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskBuilder3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 684150602, data2: 35753, data3: 19465, data4: [162, 79, 25, 104, 62, 44, 146, 76] };
}
#[repr(C)]
pub struct IBackgroundTaskBuilder4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTaskGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskBuilder4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1196811554, data2: 52130, data3: 20021, data4: [189, 22, 166, 218, 127, 28, 25, 170] };
}
#[repr(C)]
pub struct IBackgroundTaskBuilder5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTaskEntryPointClsid: unsafe extern "system" fn(this: *mut *mut Self, taskentrypoint: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskBuilder5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124847094, data2: 39413, data3: 19188, data4: [188, 173, 71, 49, 208, 51, 13, 67] };
}
#[repr(C)]
pub struct IBackgroundTaskCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CheckResult: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1448945103, data2: 61961, data3: 18676, data4: [153, 103, 43, 24, 79, 123, 251, 240] };
}
#[repr(C)]
pub struct IBackgroundTaskDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479625581, data2: 44839, data3: 19923, data4: [132, 110, 36, 238, 64, 202, 221, 37] };
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
impl ::windows_sys::core::Interface for IBackgroundTaskInstance {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2254166650, data2: 8664, data3: 17779, data4: [143, 50, 146, 138, 27, 6, 65, 246] };
}
#[repr(C)]
pub struct IBackgroundTaskInstance2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetThrottleCount: unsafe extern "system" fn(this: *mut *mut Self, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskInstance2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1333592438, data2: 3190, data3: 20404, data4: [137, 109, 93, 225, 134, 65, 34, 246] };
}
#[repr(C)]
pub struct IBackgroundTaskInstance4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IBackgroundTaskInstance4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2133455420, data2: 43524, data3: 19208, data4: [151, 176, 6, 216, 116, 205, 171, 245] };
}
#[repr(C)]
pub struct IBackgroundTaskProgressEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskProgressEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4212418732, data2: 33586, data3: 19722, data4: [149, 50, 3, 234, 230, 132, 218, 49] };
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
impl ::windows_sys::core::Interface for IBackgroundTaskRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 275074242, data2: 41582, data3: 17343, data4: [140, 18, 31, 180, 13, 191, 191, 160] };
}
#[repr(C)]
pub struct IBackgroundTaskRegistration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Trigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskRegistration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1631110915, data2: 48006, data3: 16658, data4: [175, 195, 127, 147, 155, 22, 110, 59] };
}
#[repr(C)]
pub struct IBackgroundTaskRegistration3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TaskGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskRegistration3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4264788373, data2: 37923, data3: 19851, data4: [131, 13, 177, 221, 44, 123, 173, 213] };
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
impl ::windows_sys::core::Interface for IBackgroundTaskRegistrationGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 716280218, data2: 34587, data3: 16743, data4: [138, 118, 5, 92, 214, 123, 91, 35] };
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTaskRegistrationGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2212047721, data2: 17615, data3: 17969, data4: [151, 64, 3, 199, 216, 116, 27, 197] };
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
impl ::windows_sys::core::Interface for IBackgroundTaskRegistrationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1280585577, data2: 45056, data3: 17082, data4: [160, 147, 106, 86, 60, 101, 227, 248] };
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
impl ::windows_sys::core::Interface for IBackgroundTaskRegistrationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 390817566, data2: 45581, data3: 20393, data4: [173, 154, 233, 58, 214, 199, 30, 1] };
}
#[repr(C)]
pub struct IBackgroundTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IBackgroundTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2226364504, data2: 24615, data3: 19335, data4: [151, 144, 189, 243, 247, 87, 219, 215] };
}
#[repr(C)]
pub struct IBackgroundWorkCostStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentBackgroundWorkCost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundWorkCostValue) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundWorkCostStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3342902882, data2: 49936, data3: 19330, data4: [179, 227, 59, 207, 185, 228, 199, 125] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Advertisement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Advertisement: usize,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2872976914, data2: 9683, data3: 18606, data4: [135, 36, 216, 24, 119, 174, 97, 41] };
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
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementPublisherTrigger2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2854801508, data2: 14580, data3: 22909, data4: [181, 151, 78, 85, 88, 140, 101, 3] };
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
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcherTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 447420441, data2: 48353, data3: 18667, data4: [168, 39, 89, 251, 124, 238, 82, 166] };
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBluetoothLEAdvertisementWatcherTrigger2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 968189849, data2: 60217, data3: 23222, data4: [153, 50, 170, 158, 69, 73, 96, 77] };
}
#[repr(C)]
pub struct ICachedFileUpdaterTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICachedFileUpdaterTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3793530603, data2: 13042, data3: 19761, data4: [181, 83, 185, 224, 27, 222, 55, 224] };
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
impl ::windows_sys::core::Interface for ICachedFileUpdaterTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1904446483, data2: 4884, data3: 18356, data4: [149, 151, 220, 126, 36, 140, 23, 204] };
}
#[repr(C)]
pub struct IChatMessageNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IChatMessageNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1362838463, data2: 7488, data3: 23645, data4: [120, 245, 201, 35, 254, 227, 115, 158] };
}
#[repr(C)]
pub struct IChatMessageReceivedNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IChatMessageReceivedNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1050899982, data2: 47861, data3: 16503, data4: [136, 233, 6, 12, 246, 240, 198, 213] };
}
#[repr(C)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICommunicationBlockingAppSetAsActiveTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4220646026, data2: 5797, data3: 18541, data4: [151, 76, 120, 53, 168, 71, 123, 226] };
}
#[repr(C)]
pub struct IContactStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IContactStoreNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3358802331, data2: 18181, data3: 17777, data4: [154, 22, 6, 185, 151, 191, 156, 150] };
}
#[repr(C)]
pub struct IContentPrefetchTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub WaitInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitInterval: usize,
}
impl ::windows_sys::core::Interface for IContentPrefetchTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1896228846, data2: 1274, data3: 17419, data4: [128, 192, 23, 50, 2, 25, 158, 93] };
}
#[repr(C)]
pub struct IContentPrefetchTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, waitinterval: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IContentPrefetchTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3261349594, data2: 35331, data3: 16542, data4: [184, 196, 136, 129, 76, 40, 204, 182] };
}
#[repr(C)]
pub struct ICustomSystemEventTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICustomSystemEventTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4082722712, data2: 53099, data3: 20212, data4: [160, 202, 41, 207, 74, 39, 140, 135] };
}
#[repr(C)]
pub struct ICustomSystemEventTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggerid: ::windows_sys::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICustomSystemEventTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1808471749, data2: 62172, data3: 16818, data4: [158, 253, 185, 107, 220, 209, 60, 237] };
}
#[repr(C)]
pub struct IDeviceConnectionChangeTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceConnectionChangeTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2424790628, data2: 15581, data3: 20219, data4: [171, 28, 91, 59, 106, 96, 206, 52] };
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IDeviceConnectionChangeTriggerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3286901866, data2: 20221, data3: 17560, data4: [170, 96, 164, 228, 227, 177, 122, 185] };
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
impl ::windows_sys::core::Interface for IDeviceManufacturerNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2166852277, data2: 16811, data3: 5850, data4: [134, 194, 127, 123, 240, 145, 47, 91] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IDeviceManufacturerNotificationTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2035670645, data2: 9659, data3: 16723, data4: [161, 162, 48, 41, 252, 171, 182, 82] };
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
impl ::windows_sys::core::Interface for IDeviceServicingTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 447879085, data2: 28212, data3: 18899, data4: [158, 111, 23, 241, 182, 223, 168, 129] };
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
impl ::windows_sys::core::Interface for IDeviceUseTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 229015569, data2: 13135, data3: 19799, data4: [182, 236, 109, 202, 100, 180, 18, 228] };
}
#[repr(C)]
pub struct IDeviceWatcherTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IDeviceWatcherTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2757853149, data2: 34163, data3: 16992, data4: [190, 252, 91, 236, 137, 203, 105, 61] };
}
#[repr(C)]
pub struct IEmailStoreNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IEmailStoreNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2557282010, data2: 18411, data3: 17000, data4: [164, 242, 243, 247, 113, 136, 56, 138] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3797913544, data2: 1686, data3: 18255, data4: [167, 50, 242, 146, 176, 206, 188, 93] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    EventTriggeringMode: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTrigger2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2468520644, data2: 44558, data3: 17138, data4: [178, 140, 245, 19, 114, 230, 146, 69] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, characteristic: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1471814037, data2: 45379, data3: 17781, data4: [159, 107, 253, 89, 217, 58, 206, 26] };
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub CreateWithEventTriggeringMode: unsafe extern "system" fn(this: *mut *mut Self, characteristic: *mut ::core::ffi::c_void, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))]
    CreateWithEventTriggeringMode: usize,
}
impl ::windows_sys::core::Interface for IGattCharacteristicNotificationTriggerFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1503193375, data2: 35411, data3: 20127, data4: [163, 44, 35, 205, 51, 102, 76, 238] };
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
impl ::windows_sys::core::Interface for IGattServiceProviderTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3720782825, data2: 5463, data3: 19416, data4: [133, 66, 70, 138, 160, 198, 150, 246] };
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
impl ::windows_sys::core::Interface for IGattServiceProviderTriggerResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1011257777, data2: 45464, data3: 20100, data4: [186, 212, 207, 74, 210, 153, 237, 58] };
}
#[repr(C)]
pub struct IGattServiceProviderTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, triggerid: ::windows_sys::core::HSTRING, serviceuuid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
impl ::windows_sys::core::Interface for IGattServiceProviderTriggerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3021185898, data2: 58004, data3: 17809, data4: [165, 166, 100, 137, 26, 130, 129, 83] };
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
impl ::windows_sys::core::Interface for IGeovisitTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1209593258, data2: 1249, data3: 16679, data4: [154, 76, 25, 53, 27, 138, 128, 164] };
}
#[repr(C)]
pub struct ILocationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LocationTriggerType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1197894172, data2: 26743, data3: 18462, data4: [128, 38, 255, 126, 20, 168, 17, 160] };
}
#[repr(C)]
pub struct ILocationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: LocationTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocationTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285653767, data2: 65385, data3: 19977, data4: [170, 139, 19, 132, 234, 71, 94, 152] };
}
#[repr(C)]
pub struct IMaintenanceTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMaintenanceTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1746422915, data2: 64546, data3: 19685, data4: [132, 26, 114, 57, 169, 129, 0, 71] };
}
#[repr(C)]
pub struct IMaintenanceTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMaintenanceTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1262345006, data2: 38877, data3: 17961, data4: [136, 176, 176, 108, 249, 72, 42, 229] };
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
impl ::windows_sys::core::Interface for IMediaProcessingTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2593504869, data2: 35410, data3: 19248, data4: [144, 17, 207, 56, 4, 14, 168, 176] };
}
#[repr(C)]
pub struct INetworkOperatorHotspotAuthenticationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for INetworkOperatorHotspotAuthenticationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881224081, data2: 12289, data3: 19941, data4: [131, 199, 222, 97, 216, 136, 49, 208] };
}
#[repr(C)]
pub struct INetworkOperatorNotificationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkOperatorNotificationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2416483526, data2: 25549, data3: 18444, data4: [149, 209, 110, 106, 239, 128, 30, 74] };
}
#[repr(C)]
pub struct INetworkOperatorNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INetworkOperatorNotificationTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 170016256, data2: 10199, data3: 17235, data4: [173, 185, 146, 101, 170, 234, 87, 157] };
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
impl ::windows_sys::core::Interface for IPhoneTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2379213211, data2: 54469, data3: 18929, data4: [183, 211, 130, 232, 122, 14, 157, 222] };
}
#[repr(C)]
pub struct IPhoneTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IPhoneTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2698591450, data2: 24513, data3: 18683, data4: [165, 70, 50, 38, 32, 64, 21, 123] };
}
#[repr(C)]
pub struct IPushNotificationTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPushNotificationTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1842933019, data2: 17806, data3: 20418, data4: [188, 46, 213, 102, 79, 119, 237, 25] };
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessageAvailableTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2557283690, data2: 45814, data3: 18047, data4: [169, 120, 164, 64, 145, 193, 26, 102] };
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
impl ::windows_sys::core::Interface for IRfcommConnectionTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3905211106, data2: 2899, data3: 17508, data4: [147, 148, 253, 135, 86, 84, 222, 100] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4063752999, data2: 20865, data3: 20260, data4: [150, 167, 112, 10, 78, 95, 172, 98] };
}
#[repr(C)]
pub struct ISensorDataThresholdTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISensorDataThresholdTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1539371890, data2: 54411, data3: 19327, data4: [171, 236, 21, 249, 186, 204, 18, 226] };
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Sensors")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, threshold: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISensorDataThresholdTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2451564149, data2: 32240, data3: 19875, data4: [151, 179, 229, 68, 238, 133, 127, 230] };
}
#[repr(C)]
pub struct ISmartCardTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_SmartCards")]
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    TriggerType: usize,
}
impl ::windows_sys::core::Interface for ISmartCardTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4114335148, data2: 33994, data3: 18802, data4: [140, 233, 229, 143, 151, 179, 122, 80] };
}
#[repr(C)]
pub struct ISmartCardTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_SmartCards")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISmartCardTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1673483459, data2: 35265, data3: 19968, data4: [169, 211, 151, 198, 41, 38, 157, 173] };
}
#[repr(C)]
pub struct ISmsMessageReceivedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Sms")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ISmsMessageReceivedTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3929725128, data2: 27556, data3: 19122, data4: [141, 33, 188, 107, 9, 199, 117, 100] };
}
#[repr(C)]
pub struct ISocketActivityTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISocketActivityTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2847668240, data2: 40414, data3: 20362, data4: [131, 227, 176, 224, 231, 165, 13, 112] };
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, tracker: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IStorageLibraryChangeTrackerTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 514916304, data2: 23173, data3: 18846, data4: [168, 136, 130, 70, 7, 18, 79, 80] };
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTrigger {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IStorageLibraryContentChangedTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 372760743, data2: 33436, data3: 17852, data4: [146, 155, 161, 231, 234, 120, 216, 155] };
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
impl ::windows_sys::core::Interface for IStorageLibraryContentChangedTriggerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2141133625, data2: 24464, data3: 19986, data4: [145, 78, 167, 216, 224, 187, 251, 24] };
}
#[repr(C)]
pub struct ISystemCondition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConditionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemConditionType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemCondition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244274806, data2: 35269, data3: 16907, data4: [171, 211, 251, 48, 48, 71, 33, 40] };
}
#[repr(C)]
pub struct ISystemConditionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, conditiontype: SystemConditionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemConditionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530150385, data2: 1447, data3: 18862, data4: [135, 215, 22, 178, 184, 185, 165, 83] };
}
#[repr(C)]
pub struct ISystemTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TriggerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemTriggerType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 494978934, data2: 14152, data3: 17507, data4: [141, 126, 39, 109, 193, 57, 172, 28] };
}
#[repr(C)]
pub struct ISystemTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, triggertype: SystemTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3892585428, data2: 34705, data3: 17785, data4: [129, 38, 135, 236, 138, 170, 64, 122] };
}
#[repr(C)]
pub struct ITimeTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimeTrigger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1701729622, data2: 2858, data3: 17271, data4: [186, 112, 59, 69, 169, 53, 84, 127] };
}
#[repr(C)]
pub struct ITimeTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimeTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 952533758, data2: 39764, data3: 17894, data4: [178, 243, 38, 155, 135, 166, 247, 52] };
}
#[repr(C)]
pub struct IToastNotificationActionTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationActionTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2963143719, data2: 25728, data3: 17225, data4: [129, 37, 151, 179, 239, 170, 10, 58] };
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, applicationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationHistoryChangedTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2177301165, data2: 34711, data3: 18309, data4: [129, 180, 176, 204, 203, 115, 209, 217] };
}
#[repr(C)]
pub struct IUserNotificationChangedTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Notifications")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IUserNotificationChangedTriggerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3402908524, data2: 27051, data3: 19992, data4: [164, 138, 94, 210, 172, 67, 89, 87] };
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
