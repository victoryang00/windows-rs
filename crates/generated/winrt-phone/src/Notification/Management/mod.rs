pub struct AccessoryManager;
impl AccessoryManager {
    pub fn RegisterAccessoryApp() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAccessoryApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetNextTriggerDetails() -> ::windows_core::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNextTriggerDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IAccessoryNotificationTriggerDetails>(result__)
        })
    }
    pub fn ProcessTriggerDetails<'a, Param0: ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails>>(pdetails: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).ProcessTriggerDetails)(::windows_core::Interface::as_raw(this), pdetails.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PhoneLineDetails() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLineDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhoneLineDetails>>(result__)
        })
    }
    pub fn GetPhoneLineDetails<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(phoneline: Param0) -> ::windows_core::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPhoneLineDetails)(::windows_core::Interface::as_raw(this), phoneline.into_param().abi(), result__.as_mut_ptr()).from_abi::<PhoneLineDetails>(result__)
        })
    }
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallOnEndpoint)(::windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallWithVideo)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallWithVideoOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).RejectPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).RejectPhoneCallWithText)(::windows_core::Interface::as_raw(this), phonecallid, textresponseid).ok() })
    }
    pub fn MakePhoneCall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCall)(::windows_core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    pub fn MakePhoneCallOnAudioEndpoint<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    pub fn MakePhoneCallWithVideo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallWithVideo)(::windows_core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    pub fn MakePhoneCallWithVideoOnAudioEndpoint<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallWithVideoOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SwapPhoneCalls)(::windows_core::Interface::as_raw(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).HoldPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid, holdcall).ok() })
    }
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EndPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn SetPhoneMute(value: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SetPhoneMute)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneMute() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneMute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SetPhoneCallAudioEndpoint)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneCallAudioEndpoint() -> ::windows_core::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallAudioEndpoint>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneCallAudioEndpoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallAudioEndpoint>(result__)
        })
    }
    pub fn SnoozeAlarm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(alarmid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarm)(::windows_core::Interface::as_raw(this), alarmid.into_param().abi()).ok() })
    }
    pub fn SnoozeAlarmForSpecifiedTime<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(alarmid: Param0, timespan: Param1) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarmForSpecifiedTime)(::windows_core::Interface::as_raw(this), alarmid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    pub fn DismissAlarm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(alarmid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DismissAlarm)(::windows_core::Interface::as_raw(this), alarmid.into_param().abi()).ok() })
    }
    pub fn SnoozeReminder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(reminderid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminder)(::windows_core::Interface::as_raw(this), reminderid.into_param().abi()).ok() })
    }
    pub fn SnoozeReminderForSpecifiedTime<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(reminderid: Param0, timespan: Param1) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminderForSpecifiedTime)(::windows_core::Interface::as_raw(this), reminderid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    pub fn DismissReminder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(reminderid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DismissReminder)(::windows_core::Interface::as_raw(this), reminderid.into_param().abi()).ok() })
    }
    pub fn GetMediaMetadata() -> ::windows_core::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMediaMetadata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaMetadata>(result__)
        })
    }
    pub fn MediaPlaybackCapabilities() -> ::windows_core::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackCapability>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackCapability>(result__)
        })
    }
    pub fn MediaPlaybackStatus() -> ::windows_core::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackStatus>::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackStatus>(result__)
        })
    }
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).PerformMediaPlaybackCommand)(::windows_core::Interface::as_raw(this), command).ok() })
    }
    pub fn DoNotDisturbEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DoNotDisturbEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn DrivingModeEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DrivingModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn BatterySaverState() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BatterySaverState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetApps() -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetApps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppNotificationInfo>>(result__)
        })
    }
    pub fn EnableNotificationsForApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EnableNotificationsForApplication)(::windows_core::Interface::as_raw(this), appid.into_param().abi()).ok() })
    }
    pub fn DisableNotificationsForApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DisableNotificationsForApplication)(::windows_core::Interface::as_raw(this), appid.into_param().abi()).ok() })
    }
    pub fn IsNotificationEnabledForApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appid: Param0) -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabledForApplication)(::windows_core::Interface::as_raw(this), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows_core::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetEnabledAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EnableAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this), accessorynotificationtypes).ok() })
    }
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DisableAllAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn GetUserConsent() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserConsent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn GetAppIcon<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appid: Param0) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppIcon)(::windows_core::Interface::as_raw(this), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        })
    }
    pub fn RingDevice() -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).RingDevice)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SpeedDialList() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedDialList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SpeedDialEntry>>(result__)
        })
    }
    pub fn ClearToast<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(instanceid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).ClearToast)(::windows_core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    pub fn IsPhonePinLocked() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPhonePinLocked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IncreaseVolume(step: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).IncreaseVolume)(::windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn DecreaseVolume(step: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).DecreaseVolume)(::windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn SetMute(mute: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).SetMute)(::windows_core::Interface::as_raw(this), mute).ok() })
    }
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).SetRingerVibrate)(::windows_core::Interface::as_raw(this), ringer, vibrate).ok() })
    }
    pub fn VolumeInfo() -> ::windows_core::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VolumeInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VolumeInfo>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAllEmailAccounts() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllEmailAccounts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<EmailAccountInfo>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetFolders<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(emailaccount: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolders)(::windows_core::Interface::as_raw(this), emailaccount.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<EmailFolderInfo>>(result__)
        })
    }
    pub fn EnableEmailNotificationEmailAccount<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(emailaccount: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).EnableEmailNotificationEmailAccount)(::windows_core::Interface::as_raw(this), emailaccount.into_param().abi()).ok() })
    }
    pub fn DisableEmailNotificationEmailAccount<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(emailaccount: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).DisableEmailNotificationEmailAccount)(::windows_core::Interface::as_raw(this), emailaccount.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn EnableEmailNotificationFolderFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>>(emailaccount: Param0, folders: Param1) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).EnableEmailNotificationFolderFilter)(::windows_core::Interface::as_raw(this), emailaccount.into_param().abi(), folders.into_param().abi()).ok() })
    }
    pub fn UpdateEmailReadStatus<'a, Param0: ::windows_core::IntoParam<'a, BinaryId>>(messageentryid: Param0, isread: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).UpdateEmailReadStatus)(::windows_core::Interface::as_raw(this), messageentryid.into_param().abi(), isread).ok() })
    }
    pub fn SnoozeAlarmByInstanceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(instanceid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarmByInstanceId)(::windows_core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    pub fn DismissAlarmByInstanceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(instanceid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).DismissAlarmByInstanceId)(::windows_core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    pub fn SnoozeReminderByInstanceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(instanceid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminderByInstanceId)(::windows_core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    pub fn DismissReminderByInstanceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(instanceid: Param0) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).DismissReminderByInstanceId)(::windows_core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    pub fn IAccessoryManager<R, F: FnOnce(&IAccessoryManager) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessoryManager, IAccessoryManager> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessoryManager2<R, F: FnOnce(&IAccessoryManager2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessoryManager, IAccessoryManager2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessoryManager3<R, F: FnOnce(&IAccessoryManager3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessoryManager, IAccessoryManager3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AccessoryManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for AccessoryNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AccessoryNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AccessoryNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessoryNotificationType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(::windows_core::IUnknown);
impl AlarmNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlarmId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AlarmId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ReminderState>::zeroed();
            (::windows_core::Interface::vtable(this).ReminderState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReminderState>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AlarmNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlarmNotificationTriggerDetails {}
impl ::core::fmt::Debug for AlarmNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlarmNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails;{38f5fa30-c738-4da2-908c-775d83c36abb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAlarmNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: AlarmNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &AlarmNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct AppNotificationInfo(::windows_core::IUnknown);
impl AppNotificationInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationInfo {}
impl ::core::fmt::Debug for AppNotificationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppNotificationInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AppNotificationInfo;{2157bea5-e286-45d3-9bea-f790fc216e0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppNotificationInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
impl ::core::convert::From<AppNotificationInfo> for ::windows_core::IUnknown {
    fn from(value: AppNotificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows_core::IUnknown {
    fn from(value: &AppNotificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppNotificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotificationInfo> for ::windows_core::IInspectable {
    fn from(value: AppNotificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows_core::IInspectable {
    fn from(value: &AppNotificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppNotificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct BinaryId(::windows_core::IUnknown);
impl BinaryId {
    pub fn Id(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for BinaryId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BinaryId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BinaryId {}
impl ::core::fmt::Debug for BinaryId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BinaryId").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BinaryId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.BinaryId;{4f0da531-5595-44b4-9181-ce4efa3fc168})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BinaryId {
    type Vtable = IBinaryId_Vtbl;
    const IID: ::windows_core::GUID = <IBinaryId as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
impl ::core::convert::From<BinaryId> for ::windows_core::IUnknown {
    fn from(value: BinaryId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BinaryId> for ::windows_core::IUnknown {
    fn from(value: &BinaryId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BinaryId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BinaryId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BinaryId> for ::windows_core::IInspectable {
    fn from(value: BinaryId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BinaryId> for ::windows_core::IInspectable {
    fn from(value: &BinaryId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BinaryId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BinaryId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for CalendarChangedEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CalendarChangedEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for CalendarChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(::windows_core::IUnknown);
impl CalendarChangedNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EventType(&self) -> ::windows_core::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CalendarChangedEvent>::zeroed();
            (::windows_core::Interface::vtable(this).EventType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CalendarChangedEvent>(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CalendarChangedNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CalendarChangedNotificationTriggerDetails {}
impl ::core::fmt::Debug for CalendarChangedNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails;{4b8a3bfc-279d-42ab-9c68-3e87977bf216})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <ICalendarChangedNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: CalendarChangedNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &CalendarChangedNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(::windows_core::IUnknown);
impl CortanaTileNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LargeContent1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LargeContent1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LargeContent2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LargeContent2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EmphasizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EmphasizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NonWrappedSmallContent1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NonWrappedSmallContent2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NonWrappedSmallContent3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NonWrappedSmallContent4(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CortanaTileNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CortanaTileNotificationTriggerDetails {}
impl ::core::fmt::Debug for CortanaTileNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaTileNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails;{dc0f01d5-1489-46bb-b73b-7f90067ecf27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaTileNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: CortanaTileNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &CortanaTileNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct EmailAccountInfo(::windows_core::IUnknown);
impl EmailAccountInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailAccountInfo {}
impl ::core::fmt::Debug for EmailAccountInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAccountInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailAccountInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailAccountInfo;{dfbc02ab-bda0-4568-927e-b2ede35818a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEmailAccountInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
impl ::core::convert::From<EmailAccountInfo> for ::windows_core::IUnknown {
    fn from(value: EmailAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows_core::IUnknown {
    fn from(value: &EmailAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailAccountInfo> for ::windows_core::IInspectable {
    fn from(value: EmailAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows_core::IInspectable {
    fn from(value: &EmailAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct EmailFolderInfo(::windows_core::IUnknown);
impl EmailFolderInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailFolderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailFolderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailFolderInfo {}
impl ::core::fmt::Debug for EmailFolderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFolderInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailFolderInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailFolderInfo;{c207150e-e237-46d6-90e6-4f529eeac1e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEmailFolderInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
impl ::core::convert::From<EmailFolderInfo> for ::windows_core::IUnknown {
    fn from(value: EmailFolderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows_core::IUnknown {
    fn from(value: &EmailFolderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailFolderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailFolderInfo> for ::windows_core::IInspectable {
    fn from(value: EmailFolderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows_core::IInspectable {
    fn from(value: &EmailFolderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailFolderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(::windows_core::IUnknown);
impl EmailNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ParentFolderName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SenderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SenderName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SenderAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SenderAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn EmailMessage(&self) -> ::windows_core::Result<::winrt_applicationmodel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EmailMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Email::EmailMessage>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows_core::Result<BinaryId> {
        let this = &::windows_core::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageEntryId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BinaryId>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailNotificationTriggerDetails;{f3b82612-46cf-4e70-8e0d-7b2e04ab492b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IEmailNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: EmailNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &EmailNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(::windows_core::IUnknown);
impl EmailReadNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ParentFolderName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows_core::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MessageEntryId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BinaryId>(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailReadNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailReadNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailReadNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailReadNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails;{f5b7a087-06f3-4e3e-8c42-325e67010413})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IEmailReadNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: EmailReadNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &EmailReadNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager {
    type Vtable = IAccessoryManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RegisterAccessoryApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNextTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProcessTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetails: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub PhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PhoneLineDetails: usize,
    pub GetPhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AcceptPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallOnEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub RejectPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub RejectPhoneCallWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, textresponseid: u32) -> ::windows_core::HRESULT,
    pub MakePhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub SwapPhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows_core::HRESULT,
    pub HoldPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, holdcall: bool) -> ::windows_core::HRESULT,
    pub EndPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub SetPhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub PhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub SnoozeAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SnoozeAlarmForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID, timespan: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DismissAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SnoozeReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SnoozeReminderForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID, timespan: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DismissReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MediaPlaybackCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackCapability) -> ::windows_core::HRESULT,
    pub MediaPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows_core::HRESULT,
    pub PerformMediaPlaybackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: PlaybackCommand) -> ::windows_core::HRESULT,
    pub DoNotDisturbEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DrivingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub BatterySaverState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetApps: usize,
    pub EnableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessorynotificationtypes: i32) -> ::windows_core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub GetAppIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    GetAppIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbacad44d_d393_46c6_b80c_15fdf44d5386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SpeedDialList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPhonePinLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IncreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows_core::HRESULT,
    pub DecreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mute: bool) -> ::windows_core::HRESULT,
    pub SetRingerVibrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ringer: bool, vibrate: bool) -> ::windows_core::HRESULT,
    pub VolumeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetAllEmailAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAllEmailAccounts: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, folders: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageentryid: ::windows_core::RawPtr, isread: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(::windows_core::IUnknown);
impl IAccessoryNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccessoryNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccessoryNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessoryNotificationTriggerDetails {}
impl ::core::fmt::Debug for IAccessoryNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessoryNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6968a7d4-e3ca-49cb-8c87-2c11cdff9646}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TimeCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows_core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlarmId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinaryId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBinaryId {
    type Vtable = IBinaryId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinaryId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CalendarChangedEvent) -> ::windows_core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAccountInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAccountInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolderInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolderInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub EmailMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    EmailMessage: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows_core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMetadata_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PhoneLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CallTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallTransport) -> ::windows_core::HRESULT,
    pub CallMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneMediaType) -> ::windows_core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallState) -> ::windows_core::HRESULT,
    pub ConferenceCallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub PresetTextResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PresetTextResponses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineRegistrationState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb30cd77d_0147_498c_8241_bf0cabc60a25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNotificationType) -> ::windows_core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReminderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Appointment: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeedDialEntry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeedDialEntry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextResponse {
    type Vtable = ITextResponse_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextResponse_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VibrateState) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(::windows_core::IUnknown);
impl MediaControlsTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackStatus>::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackStatus>(result__)
        }
    }
    pub fn MediaMetadata(&self) -> ::windows_core::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MediaMetadata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaMetadata>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaControlsTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaControlsTriggerDetails {}
impl ::core::fmt::Debug for MediaControlsTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaControlsTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaControlsTriggerDetails;{fab4648b-ae45-4548-91ca-4ab0548e33b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IMediaControlsTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: MediaControlsTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &MediaControlsTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct MediaMetadata(::windows_core::IUnknown);
impl MediaMetadata {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Album(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Album)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Track(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Track)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaMetadata {}
impl ::core::fmt::Debug for MediaMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MediaMetadata {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaMetadata;{9b50ddf7-bb6c-4330-b3cd-0704a54cdb80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
    const IID: ::windows_core::GUID = <IMediaMetadata as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
impl ::core::convert::From<MediaMetadata> for ::windows_core::IUnknown {
    fn from(value: MediaMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows_core::IUnknown {
    fn from(value: &MediaMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MediaMetadata {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MediaMetadata {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaMetadata> for ::windows_core::IInspectable {
    fn from(value: MediaMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows_core::IInspectable {
    fn from(value: &MediaMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MediaMetadata {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MediaMetadata {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneCallAudioEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneCallAudioEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhoneCallDetails(::windows_core::IUnknown);
impl PhoneCallDetails {
    pub fn PhoneLine(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn CallId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CallId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CallTransport(&self) -> ::windows_core::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallTransport>::zeroed();
            (::windows_core::Interface::vtable(this).CallTransport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallTransport>(result__)
        }
    }
    pub fn CallMediaType(&self) -> ::windows_core::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneMediaType>::zeroed();
            (::windows_core::Interface::vtable(this).CallMediaType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneMediaType>(result__)
        }
    }
    pub fn CallDirection(&self) -> ::windows_core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallDirection>::zeroed();
            (::windows_core::Interface::vtable(this).CallDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallDirection>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallState>(result__)
        }
    }
    pub fn ConferenceCallId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ConferenceCallId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContactName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PresetTextResponses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PresetTextResponses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<TextResponse>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallDetails {}
impl ::core::fmt::Debug for PhoneCallDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneCallDetails;{0c1b6f53-f071-483e-bf33-ebd44b724447})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneCallDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
impl ::core::convert::From<PhoneCallDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneCallDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneCallDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneCallDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneCallDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneCallDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneCallDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneCallDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneCallDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneCallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneCallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneCallTransport {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneCallTransport {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallTransport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhoneLineDetails(::windows_core::IUnknown);
impl PhoneLineDetails {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LineNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DefaultOutgoingLine(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultOutgoingLine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VoicemailCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn RegistrationState(&self) -> ::windows_core::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneLineRegistrationState>::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneLineRegistrationState>(result__)
        }
    }
    pub fn MissedCallCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MissedCallCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineDetails {}
impl ::core::fmt::Debug for PhoneLineDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneLineDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneLineDetails;{47eb32dc-33ed-49b9-995c-a296bac82b77})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneLineDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
impl ::core::convert::From<PhoneLineDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneLineDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneLineDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneLineDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneLineDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneLineDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneLineDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneLineRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneLineRegistrationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineRegistrationState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneMediaType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneMediaType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneMediaType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(::windows_core::IUnknown);
impl PhoneNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhoneNotificationType(&self) -> ::windows_core::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneNotificationType>(result__)
        }
    }
    pub fn CallDetails(&self) -> ::windows_core::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CallDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallDetails>(result__)
        }
    }
    pub fn PhoneLineChangedId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLineChangedId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails;{ccc2fdf7-09c3-4118-91bc-ca6323a8d383})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: PhoneNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhoneNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PhoneNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNotificationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PlaybackCapability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlaybackCapability {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCapability").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackCapability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PlaybackCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlaybackCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackCommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for PlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlaybackStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaybackStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(::windows_core::IUnknown);
impl ReminderNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReminderId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ReminderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Appointment(&self) -> ::windows_core::Result<::winrt_applicationmodel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Appointment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Appointments::Appointment>(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ReminderState>::zeroed();
            (::windows_core::Interface::vtable(this).ReminderState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReminderState>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReminderNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReminderNotificationTriggerDetails {}
impl ::core::fmt::Debug for ReminderNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails;{5bddaa5d-9f61-4bf0-9feb-10502bc0b0c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IReminderNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: ReminderNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &ReminderNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for ReminderState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ReminderState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReminderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReminderState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpeedDialEntry(::windows_core::IUnknown);
impl SpeedDialEntry {
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NumberType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumberType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContactName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeedDialEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeedDialEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeedDialEntry {}
impl ::core::fmt::Debug for SpeedDialEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeedDialEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpeedDialEntry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.SpeedDialEntry;{9240b6db-872c-46dc-b62a-be4541b166f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
    const IID: ::windows_core::GUID = <ISpeedDialEntry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
impl ::core::convert::From<SpeedDialEntry> for ::windows_core::IUnknown {
    fn from(value: SpeedDialEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows_core::IUnknown {
    fn from(value: &SpeedDialEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpeedDialEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeedDialEntry> for ::windows_core::IInspectable {
    fn from(value: SpeedDialEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows_core::IInspectable {
    fn from(value: &SpeedDialEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpeedDialEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct TextResponse(::windows_core::IUnknown);
impl TextResponse {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for TextResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextResponse {}
impl ::core::fmt::Debug for TextResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TextResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.TextResponse;{e9cb74c3-2457-4cdb-8110-72f5e8e883e8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TextResponse {
    type Vtable = ITextResponse_Vtbl;
    const IID: ::windows_core::GUID = <ITextResponse as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
impl ::core::convert::From<TextResponse> for ::windows_core::IUnknown {
    fn from(value: TextResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextResponse> for ::windows_core::IUnknown {
    fn from(value: &TextResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TextResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TextResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextResponse> for ::windows_core::IInspectable {
    fn from(value: TextResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextResponse> for ::windows_core::IInspectable {
    fn from(value: &TextResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TextResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TextResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(::windows_core::IUnknown);
impl ToastNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Text2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Text3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Text4(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SuppressPopup(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SuppressPopup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationTriggerDetails {}
impl ::core::fmt::Debug for ToastNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ToastNotificationTriggerDetails;{c9314895-4e6d-4e9d-afec-9e921b875ae8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: ToastNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToastNotificationTriggerDetails) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for VibrateState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VibrateState {
    type Abi = Self;
}
impl ::core::fmt::Debug for VibrateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrateState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VibrateState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct VolumeInfo(::windows_core::IUnknown);
impl VolumeInfo {
    pub fn SystemVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SystemVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CallVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).CallVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MediaVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MediaVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMuted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVibrateEnabled(&self) -> ::windows_core::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VibrateState>::zeroed();
            (::windows_core::Interface::vtable(this).IsVibrateEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VibrateState>(result__)
        }
    }
}
impl ::core::clone::Clone for VolumeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VolumeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeInfo {}
impl ::core::fmt::Debug for VolumeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VolumeInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.VolumeInfo;{944dd118-7704-4481-b92e-d3ed3ece6322})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
    const IID: ::windows_core::GUID = <IVolumeInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
impl ::core::convert::From<VolumeInfo> for ::windows_core::IUnknown {
    fn from(value: VolumeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows_core::IUnknown {
    fn from(value: &VolumeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VolumeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VolumeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VolumeInfo> for ::windows_core::IInspectable {
    fn from(value: VolumeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows_core::IInspectable {
    fn from(value: &VolumeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VolumeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VolumeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
