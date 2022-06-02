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
#[repr(C)]
pub struct IAccessoryManager3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut *mut Self, instanceid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAlarmNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppNotificationInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBinaryId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarChangedNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CalendarChangedEvent) -> ::windows_sys::core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IEmailAccountInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEmailFolderInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IEmailNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEmailReadNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccountName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaControlsTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlaybackStatus) -> ::windows_sys::core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPhoneLineDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPhoneNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneNotificationType) -> ::windows_sys::core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IReminderNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeedDialEntry {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextResponse {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IToastNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
