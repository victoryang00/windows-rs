#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: Self = Self(0u32);
    pub const Phone: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Reminder: Self = Self(4u32);
    pub const Alarm: Self = Self(8u32);
    pub const Toast: Self = Self(16u32);
    pub const AppUninstalled: Self = Self(32u32);
    pub const Dnd: Self = Self(64u32);
    pub const DrivingMode: Self = Self(128u32);
    pub const BatterySaver: Self = Self(256u32);
    pub const Media: Self = Self(512u32);
    pub const CortanaTile: Self = Self(1024u32);
    pub const ToastCleared: Self = Self(2048u32);
    pub const CalendarChanged: Self = Self(4096u32);
    pub const VolumeChanged: Self = Self(8192u32);
    pub const EmailReadStatusChanged: Self = Self(16384u32);
}
impl ::core::marker::Copy for AccessoryNotificationType {}
impl ::core::clone::Clone for AccessoryNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AlarmNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type AppNotificationInfo = *mut ::core::ffi::c_void;
pub type BinaryId = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: Self = Self(0i32);
    pub const AppointmentAdded: Self = Self(1i32);
    pub const AppointmentChanged: Self = Self(2i32);
    pub const AppointmentDeleted: Self = Self(3i32);
    pub const CalendarAdded: Self = Self(4i32);
    pub const CalendarChanged: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for CalendarChangedEvent {}
impl ::core::clone::Clone for CalendarChangedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CalendarChangedNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type CortanaTileNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type EmailAccountInfo = *mut ::core::ffi::c_void;
pub type EmailFolderInfo = *mut ::core::ffi::c_void;
pub type EmailNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type EmailReadNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAccessoryManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterAccessoryApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNextTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessTriggerDetails: unsafe extern "system" fn(this: *mut *mut Self, pdetails: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneLineDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneLineDetails: usize,
    pub GetPhoneLineDetails: unsafe extern "system" fn(this: *mut *mut Self, phoneline: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AcceptPhoneCall: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32) -> ::windows_sys::core::HRESULT,
    pub AcceptPhoneCallOnEndpoint: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub AcceptPhoneCallWithVideo: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32) -> ::windows_sys::core::HRESULT,
    pub AcceptPhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub RejectPhoneCall: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32) -> ::windows_sys::core::HRESULT,
    pub RejectPhoneCallWithText: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32, textresponseid: u32) -> ::windows_sys::core::HRESULT,
    pub MakePhoneCall: unsafe extern "system" fn(this: *mut *mut Self, phoneline: ::windows_sys::core::GUID, phonenumber: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, phoneline: ::windows_sys::core::GUID, phonenumber: ::windows_sys::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(this: *mut *mut Self, phoneline: ::windows_sys::core::GUID, phonenumber: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, phoneline: ::windows_sys::core::GUID, phonenumber: ::windows_sys::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub SwapPhoneCalls: unsafe extern "system" fn(this: *mut *mut Self, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows_sys::core::HRESULT,
    pub HoldPhoneCall: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32, holdcall: bool) -> ::windows_sys::core::HRESULT,
    pub EndPhoneCall: unsafe extern "system" fn(this: *mut *mut Self, phonecallid: u32) -> ::windows_sys::core::HRESULT,
    pub SetPhoneMute: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PhoneMute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub PhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallAudioEndpoint) -> ::windows_sys::core::HRESULT,
    pub SnoozeAlarm: unsafe extern "system" fn(this: *mut *mut Self, alarmid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeAlarmForSpecifiedTime: unsafe extern "system" fn(this: *mut *mut Self, alarmid: ::windows_sys::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeAlarmForSpecifiedTime: usize,
    pub DismissAlarm: unsafe extern "system" fn(this: *mut *mut Self, alarmid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SnoozeReminder: unsafe extern "system" fn(this: *mut *mut Self, reminderid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeReminderForSpecifiedTime: unsafe extern "system" fn(this: *mut *mut Self, reminderid: ::windows_sys::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeReminderForSpecifiedTime: usize,
    pub DismissReminder: unsafe extern "system" fn(this: *mut *mut Self, reminderid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetMediaMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MediaPlaybackCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlaybackCapability) -> ::windows_sys::core::HRESULT,
    pub MediaPlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlaybackStatus) -> ::windows_sys::core::HRESULT,
    pub PerformMediaPlaybackCommand: unsafe extern "system" fn(this: *mut *mut Self, command: PlaybackCommand) -> ::windows_sys::core::HRESULT,
    pub DoNotDisturbEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DrivingModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub BatterySaverState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApps: usize,
    pub EnableNotificationsForApplication: unsafe extern "system" fn(this: *mut *mut Self, appid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(this: *mut *mut Self, appid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(this: *mut *mut Self, appid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut *mut Self, accessorynotificationtypes: i32) -> ::windows_sys::core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAppIcon: unsafe extern "system" fn(this: *mut *mut Self, appid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAppIcon: usize,
}
impl ::windows_sys::core::Interface for IAccessoryManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218407212, data2: 34877, data3: 19111, data4: [188, 167, 250, 75, 184, 191, 254, 230] };
}
#[repr(C)]
pub struct IAccessoryManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RingDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SpeedDialList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsPhonePinLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IncreaseVolume: unsafe extern "system" fn(this: *mut *mut Self, step: i32) -> ::windows_sys::core::HRESULT,
    pub DecreaseVolume: unsafe extern "system" fn(this: *mut *mut Self, step: i32) -> ::windows_sys::core::HRESULT,
    pub SetMute: unsafe extern "system" fn(this: *mut *mut Self, mute: bool) -> ::windows_sys::core::HRESULT,
    pub SetRingerVibrate: unsafe extern "system" fn(this: *mut *mut Self, ringer: bool, vibrate: bool) -> ::windows_sys::core::HRESULT,
    pub VolumeInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllEmailAccounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllEmailAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFolders: unsafe extern "system" fn(this: *mut *mut Self, emailaccount: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut *mut Self, emailaccount: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut *mut Self, emailaccount: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(this: *mut *mut Self, emailaccount: ::windows_sys::core::HSTRING, folders: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(this: *mut *mut Self, messageentryid: *mut ::core::ffi::c_void, isread: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccessoryManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3133854797, data2: 54163, data3: 18118, data4: [184, 12, 21, 253, 244, 77, 83, 134] };
}
#[repr(C)]
pub struct IAccessoryManager3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccessoryManager3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2180469047, data2: 60871, data3: 18400, data4: [178, 247, 126, 87, 124, 131, 63, 125] };
}
#[repr(C)]
pub struct IAccessoryNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TimeCreated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeCreated: usize,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AccessoryNotificationType) -> ::windows_sys::core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccessoryNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1768466388, data2: 58314, data3: 18891, data4: [140, 135, 44, 17, 205, 255, 150, 70] };
}
#[repr(C)]
pub struct IAlarmNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlarmId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ReminderState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAlarmNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 955644464, data2: 51000, data3: 19874, data4: [144, 140, 119, 93, 131, 195, 106, 187] };
}
#[repr(C)]
pub struct IAlarmNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAlarmNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474382954, data2: 29013, data3: 16638, data4: [169, 194, 123, 210, 18, 126, 248, 83] };
}
#[repr(C)]
pub struct IAppNotificationInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppNotificationInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 559398565, data2: 57990, data3: 17875, data4: [155, 234, 247, 144, 252, 33, 110, 14] };
}
#[repr(C)]
pub struct IBinaryId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBinaryId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1326294321, data2: 21909, data3: 17588, data4: [145, 129, 206, 78, 250, 63, 193, 104] };
}
#[repr(C)]
pub struct ICalendarChangedNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CalendarChangedEvent) -> ::windows_sys::core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICalendarChangedNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1267350524, data2: 10141, data3: 17067, data4: [156, 104, 62, 135, 151, 123, 242, 22] };
}
#[repr(C)]
pub struct ICortanaTileNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub TileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICortanaTileNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3691971029, data2: 5257, data3: 18107, data4: [183, 59, 127, 144, 6, 126, 207, 39] };
}
#[repr(C)]
pub struct IEmailAccountInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailAccountInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3753640619, data2: 48544, data3: 17768, data4: [146, 126, 178, 237, 227, 88, 24, 161] };
}
#[repr(C)]
pub struct IEmailFolderInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailFolderInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3255244046, data2: 57911, data3: 18134, data4: [144, 230, 79, 82, 158, 234, 193, 226] };
}
#[repr(C)]
pub struct IEmailNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")]
    pub EmailMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))]
    EmailMessage: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
impl ::windows_sys::core::Interface for IEmailNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4088931858, data2: 18127, data3: 20080, data4: [142, 13, 123, 46, 4, 171, 73, 43] };
}
#[repr(C)]
pub struct IEmailNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 377513955, data2: 50543, data3: 20167, data4: [190, 209, 247, 52, 224, 141, 229, 178] };
}
#[repr(C)]
pub struct IEmailReadNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailReadNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4122452103, data2: 1779, data3: 20030, data4: [140, 66, 50, 94, 103, 1, 4, 19] };
}
#[repr(C)]
pub struct IMediaControlsTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlaybackStatus) -> ::windows_sys::core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaControlsTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4206126219, data2: 44613, data3: 17736, data4: [145, 202, 74, 176, 84, 142, 51, 181] };
}
#[repr(C)]
pub struct IMediaMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Track: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
impl ::windows_sys::core::Interface for IMediaMetadata {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2605768183, data2: 47980, data3: 17200, data4: [179, 205, 7, 4, 165, 76, 219, 128] };
}
#[repr(C)]
pub struct IPhoneCallDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CallTransport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallTransport) -> ::windows_sys::core::HRESULT,
    pub CallMediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneMediaType) -> ::windows_sys::core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallDirection) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallState) -> ::windows_sys::core::HRESULT,
    pub ConferenceCallId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresetTextResponses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresetTextResponses: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 203124563, data2: 61553, data3: 18494, data4: [191, 51, 235, 212, 75, 114, 68, 71] };
}
#[repr(C)]
pub struct IPhoneLineDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineRegistrationState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1206596316, data2: 13293, data3: 18873, data4: [153, 92, 162, 150, 186, 200, 43, 119] };
}
#[repr(C)]
pub struct IPhoneLineDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3003963261, data2: 327, data3: 18828, data4: [130, 65, 191, 12, 171, 198, 10, 37] };
}
#[repr(C)]
pub struct IPhoneNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneNotificationType) -> ::windows_sys::core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3435331063, data2: 2499, data3: 16664, data4: [145, 188, 202, 99, 35, 168, 211, 131] };
}
#[repr(C)]
pub struct IReminderNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReminderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Appointment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Appointment: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ReminderState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReminderNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1541253725, data2: 40801, data3: 19440, data4: [159, 235, 16, 80, 43, 192, 176, 194] };
}
#[repr(C)]
pub struct IReminderNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReminderNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3876977088, data2: 20557, data3: 19471, data4: [166, 179, 188, 185, 114, 44, 108, 221] };
}
#[repr(C)]
pub struct ISpeedDialEntry {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeedDialEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2453714651, data2: 34604, data3: 18140, data4: [182, 42, 190, 69, 65, 177, 102, 248] };
}
#[repr(C)]
pub struct ITextResponse {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3922425027, data2: 9303, data3: 19675, data4: [129, 16, 114, 245, 232, 232, 131, 232] };
}
#[repr(C)]
pub struct IToastNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3375450261, data2: 20077, data3: 20125, data4: [175, 236, 158, 146, 27, 135, 90, 232] };
}
#[repr(C)]
pub struct IToastNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IToastNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1040480733, data2: 51908, data3: 20320, data4: [175, 163, 185, 37, 217, 216, 60, 147] };
}
#[repr(C)]
pub struct IVolumeInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VibrateState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVolumeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2488127768, data2: 30468, data3: 17537, data4: [185, 46, 211, 237, 62, 206, 99, 34] };
}
pub type MediaControlsTriggerDetails = *mut ::core::ffi::c_void;
pub type MediaMetadata = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Speaker: Self = Self(1i32);
    pub const Handsfree: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioEndpoint {}
impl ::core::clone::Clone for PhoneCallAudioEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: Self = Self(0i32);
    pub const Outgoing: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: Self = Self(0i32);
    pub const Ringing: Self = Self(1i32);
    pub const Talking: Self = Self(2i32);
    pub const Held: Self = Self(3i32);
    pub const Ended: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallState {}
impl ::core::clone::Clone for PhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: Self = Self(0i32);
    pub const Voip: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallTransport {}
impl ::core::clone::Clone for PhoneCallTransport {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneLineDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Roaming: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineRegistrationState {}
impl ::core::clone::Clone for PhoneLineRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: Self = Self(0i32);
    pub const AudioVideo: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneMediaType {}
impl ::core::clone::Clone for PhoneMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: Self = Self(0i32);
    pub const CallChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const PhoneCallAudioEndpointChanged: Self = Self(3i32);
    pub const PhoneMuteChanged: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNotificationType {}
impl ::core::clone::Clone for PhoneNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: Self = Self(0u32);
    pub const Play: Self = Self(1u32);
    pub const Pause: Self = Self(2u32);
    pub const Stop: Self = Self(4u32);
    pub const Record: Self = Self(8u32);
    pub const FastForward: Self = Self(16u32);
    pub const Rewind: Self = Self(32u32);
    pub const Next: Self = Self(64u32);
    pub const Previous: Self = Self(128u32);
    pub const ChannelUp: Self = Self(256u32);
    pub const ChannelDown: Self = Self(512u32);
}
impl ::core::marker::Copy for PlaybackCapability {}
impl ::core::clone::Clone for PlaybackCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for PlaybackCommand {}
impl ::core::clone::Clone for PlaybackCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: Self = Self(0i32);
    pub const TrackChanged: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for PlaybackStatus {}
impl ::core::clone::Clone for PlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ReminderNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: Self = Self(0i32);
    pub const Snoozed: Self = Self(1i32);
    pub const Dismissed: Self = Self(2i32);
}
impl ::core::marker::Copy for ReminderState {}
impl ::core::clone::Clone for ReminderState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeedDialEntry = *mut ::core::ffi::c_void;
pub type TextResponse = *mut ::core::ffi::c_void;
pub type ToastNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: Self = Self(0i32);
    pub const RingerOffVibrateOn: Self = Self(1i32);
    pub const RingerOnVibrateOff: Self = Self(2i32);
    pub const RingerOnVibrateOn: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrateState {}
impl ::core::clone::Clone for VibrateState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VolumeInfo = *mut ::core::ffi::c_void;
