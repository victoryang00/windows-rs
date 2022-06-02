#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
#[repr(C)]
pub struct INetworkTransportSettings {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut *mut Self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut *mut Self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
#[repr(C)]
pub struct INotificationTransportSync {
    pub base__: ::windows_sys::core::IUnknown,
    pub CompleteDelivery: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCBuddy {
    pub base__: IRTCPresenceContact,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Notes: unsafe extern "system" fn(this: *mut *mut Self, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notes: usize,
}
#[repr(C)]
pub struct IRTCBuddy2 {
    pub base__: IRTCBuddy,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut *mut Self, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_PresenceProperty: usize,
    pub EnumeratePresenceDevices: unsafe extern "system" fn(this: *mut *mut Self, ppenumdevices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PresenceDevices: unsafe extern "system" fn(this: *mut *mut Self, ppdevicescollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PresenceDevices: usize,
    pub SubscriptionType: unsafe extern "system" fn(this: *mut *mut Self, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyEvent {
    pub base__: super::Com::IDispatch,
    pub Buddy: unsafe extern "system" fn(this: *mut *mut Self, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyEvent2 {
    pub base__: IRTCBuddyEvent,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[repr(C)]
pub struct IRTCBuddyGroup {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub AddBuddy: unsafe extern "system" fn(this: *mut *mut Self, pbuddy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut *mut Self, pbuddy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyGroupEvent {
    pub base__: super::Com::IDispatch,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Buddy: unsafe extern "system" fn(this: *mut *mut Self, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClient {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PrepareForShutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetEventFilter: unsafe extern "system" fn(this: *mut *mut Self, lfilter: i32) -> ::windows_sys::core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(this: *mut *mut Self, plfilter: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPreferredMediaTypes: unsafe extern "system" fn(this: *mut *mut Self, lmediatypes: i32, fpersistent: i16) -> ::windows_sys::core::HRESULT,
    pub PreferredMediaTypes: unsafe extern "system" fn(this: *mut *mut Self, plmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MediaCapabilities: unsafe extern "system" fn(this: *mut *mut Self, plmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSession: unsafe extern "system" fn(this: *mut *mut Self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSession: usize,
    pub SetListenForIncomingSessions: unsafe extern "system" fn(this: *mut *mut Self, enlisten: RTC_LISTEN_MODE) -> ::windows_sys::core::HRESULT,
    pub ListenForIncomingSessions: unsafe extern "system" fn(this: *mut *mut Self, penlisten: *mut RTC_LISTEN_MODE) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_NetworkAddresses: unsafe extern "system" fn(this: *mut *mut Self, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_NetworkAddresses: usize,
    pub put_Volume: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_sys::core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub put_AudioMuted: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows_sys::core::HRESULT,
    pub get_AudioMuted: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub get_IVideoWindow: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com")))]
    get_IVideoWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_PreferredAudioDevice: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_PreferredAudioDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_PreferredAudioDevice: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_PreferredAudioDevice: usize,
    pub put_PreferredVolume: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_sys::core::HRESULT,
    pub get_PreferredVolume: unsafe extern "system" fn(this: *mut *mut Self, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPreferredAEC: unsafe extern "system" fn(this: *mut *mut Self, benable: i16) -> ::windows_sys::core::HRESULT,
    pub PreferredAEC: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPreferredVideoDevice: unsafe extern "system" fn(this: *mut *mut Self, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPreferredVideoDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreferredVideoDevice: unsafe extern "system" fn(this: *mut *mut Self, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreferredVideoDevice: usize,
    pub ActiveMedia: unsafe extern "system" fn(this: *mut *mut Self, plmediatype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxBitrate: unsafe extern "system" fn(this: *mut *mut Self, lmaxbitrate: i32) -> ::windows_sys::core::HRESULT,
    pub MaxBitrate: unsafe extern "system" fn(this: *mut *mut Self, plmaxbitrate: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut *mut Self, lvalue: i32) -> ::windows_sys::core::HRESULT,
    pub TemporalSpatialTradeOff: unsafe extern "system" fn(this: *mut *mut Self, plvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub NetworkQuality: unsafe extern "system" fn(this: *mut *mut Self, plnetworkquality: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StartT120Applet: unsafe extern "system" fn(this: *mut *mut Self, enapplet: RTC_T120_APPLET) -> ::windows_sys::core::HRESULT,
    pub StopT120Applets: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub get_IsT120AppletRunning: unsafe extern "system" fn(this: *mut *mut Self, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalUserURI: unsafe extern "system" fn(this: *mut *mut Self, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalUserURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalUserURI: unsafe extern "system" fn(this: *mut *mut Self, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalUserURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalUserName: unsafe extern "system" fn(this: *mut *mut Self, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalUserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalUserName: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalUserName: usize,
    pub PlayRing: unsafe extern "system" fn(this: *mut *mut Self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows_sys::core::HRESULT,
    pub SendDTMF: unsafe extern "system" fn(this: *mut *mut Self, endtmf: RTC_DTMF) -> ::windows_sys::core::HRESULT,
    pub InvokeTuningWizard: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: isize) -> ::windows_sys::core::HRESULT,
    pub IsTuned: unsafe extern "system" fn(this: *mut *mut Self, pftuned: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClient2 {
    pub base__: IRTCClient,
    pub put_AnswerMode: unsafe extern "system" fn(this: *mut *mut Self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows_sys::core::HRESULT,
    pub get_AnswerMode: unsafe extern "system" fn(this: *mut *mut Self, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows_sys::core::HRESULT,
    pub InvokeTuningWizardEx: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, plversion: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut *mut Self, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientCurVer: unsafe extern "system" fn(this: *mut *mut Self, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientCurVer: usize,
    pub InitializeEx: unsafe extern "system" fn(this: *mut *mut Self, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSessionWithDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSessionWithDescription: usize,
    pub SetSessionDescriptionManager: unsafe extern "system" fn(this: *mut *mut Self, psessiondescriptionmanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub put_AllowedPorts: unsafe extern "system" fn(this: *mut *mut Self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows_sys::core::HRESULT,
    pub get_AllowedPorts: unsafe extern "system" fn(this: *mut *mut Self, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCClientEvent {
    pub base__: super::Com::IDispatch,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub Client: unsafe extern "system" fn(this: *mut *mut Self, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClientPortManagement {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub StartListenAddressAndPort: unsafe extern "system" fn(this: *mut *mut Self, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartListenAddressAndPort: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StopListenAddressAndPort: unsafe extern "system" fn(this: *mut *mut Self, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StopListenAddressAndPort: usize,
    pub GetPortRange: unsafe extern "system" fn(this: *mut *mut Self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClientPresence {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnablePresence: unsafe extern "system" fn(this: *mut *mut Self, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnablePresence: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Export: unsafe extern "system" fn(this: *mut *mut Self, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Export: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Import: usize,
    pub EnumerateBuddies: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Buddy: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Buddy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddBuddy: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddBuddy: usize,
    pub RemoveBuddy: unsafe extern "system" fn(this: *mut *mut Self, pbuddy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateWatchers: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Watchers: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Watchers: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Watcher: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Watcher: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWatcher: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWatcher: usize,
    pub RemoveWatcher: unsafe extern "system" fn(this: *mut *mut Self, pwatcher: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalPresenceInfo: unsafe extern "system" fn(this: *mut *mut Self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalPresenceInfo: usize,
    pub OfferWatcherMode: unsafe extern "system" fn(this: *mut *mut Self, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows_sys::core::HRESULT,
    pub SetOfferWatcherMode: unsafe extern "system" fn(this: *mut *mut Self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_sys::core::HRESULT,
    pub PrivacyMode: unsafe extern "system" fn(this: *mut *mut Self, penmode: *mut RTC_PRIVACY_MODE) -> ::windows_sys::core::HRESULT,
    pub SetPrivacyMode: unsafe extern "system" fn(this: *mut *mut Self, enmode: RTC_PRIVACY_MODE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClientPresence2 {
    pub base__: IRTCClientPresence,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnablePresenceEx: unsafe extern "system" fn(this: *mut *mut Self, pprofile: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnablePresenceEx: usize,
    pub DisablePresence: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddGroup: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddGroup: usize,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut *mut Self, pgroup: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Group: unsafe extern "system" fn(this: *mut *mut Self, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Group: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWatcherEx: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWatcherEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_WatcherEx: unsafe extern "system" fn(this: *mut *mut Self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_WatcherEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_PresenceProperty: unsafe extern "system" fn(this: *mut *mut Self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_PresenceProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut *mut Self, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_PresenceProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPresenceData: unsafe extern "system" fn(this: *mut *mut Self, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPresenceData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresenceData: unsafe extern "system" fn(this: *mut *mut Self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresenceData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut *mut Self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalPresenceInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddBuddyEx: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddBuddyEx: usize,
}
#[repr(C)]
pub struct IRTCClientProvisioning {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateProfile: unsafe extern "system" fn(this: *mut *mut Self, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateProfile: usize,
    pub EnableProfile: unsafe extern "system" fn(this: *mut *mut Self, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32) -> ::windows_sys::core::HRESULT,
    pub DisableProfile: unsafe extern "system" fn(this: *mut *mut Self, pprofile: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateProfiles: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Profiles: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProfile: unsafe extern "system" fn(this: *mut *mut Self, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProfile: usize,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut *mut Self, plsupportedsessions: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCClientProvisioning2 {
    pub base__: IRTCClientProvisioning,
    pub EnableProfileEx: unsafe extern "system" fn(this: *mut *mut Self, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCCollection {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, lcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCDispatchEventNotification {
    pub base__: super::Com::IDispatch,
}
#[repr(C)]
pub struct IRTCEnumBuddies {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumGroups {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumParticipants {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumPresenceDevices {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumProfiles {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumUserSearchResults {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEnumWatchers {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCEventNotification {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(this: *mut *mut Self, rtcevent: RTC_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCInfoEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, pbstrinfo: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Info: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InfoHeader: unsafe extern "system" fn(this: *mut *mut Self, pbstrinfoheader: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfoHeader: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCIntensityEvent {
    pub base__: super::Com::IDispatch,
    pub Level: unsafe extern "system" fn(this: *mut *mut Self, pllevel: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, plmin: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, plmax: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMediaEvent {
    pub base__: super::Com::IDispatch,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, pmediatype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub EventReason: unsafe extern "system" fn(this: *mut *mut Self, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMediaRequestEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProposedMedia: unsafe extern "system" fn(this: *mut *mut Self, plmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CurrentMedia: unsafe extern "system" fn(this: *mut *mut Self, plmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self, lmediatypes: i32) -> ::windows_sys::core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut RTC_REINVITE_STATE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMessagingEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, pbstrmessage: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Message: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageHeader: unsafe extern "system" fn(this: *mut *mut Self, pbstrmessageheader: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageHeader: usize,
    pub UserStatus: unsafe extern "system" fn(this: *mut *mut Self, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCParticipant {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub UserURI: unsafe extern "system" fn(this: *mut *mut Self, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Removable: unsafe extern "system" fn(this: *mut *mut Self, pfremovable: *mut i16) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCParticipantStateChangeEvent {
    pub base__: super::Com::IDispatch,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCPortManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMapping: unsafe extern "system" fn(this: *mut *mut Self, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMapping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateRemoteAddress: unsafe extern "system" fn(this: *mut *mut Self, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateRemoteAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMapping: unsafe extern "system" fn(this: *mut *mut Self, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMapping: usize,
}
#[repr(C)]
pub struct IRTCPresenceContact {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PresentityURI: unsafe extern "system" fn(this: *mut *mut Self, pbstrpresentityuri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PresentityURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPresentityURI: unsafe extern "system" fn(this: *mut *mut Self, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPresentityURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
    pub Persistent: unsafe extern "system" fn(this: *mut *mut Self, pfpersistent: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetPersistent: unsafe extern "system" fn(this: *mut *mut Self, fpersistent: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresenceDataEvent {
    pub base__: super::Com::IDispatch,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresenceData: unsafe extern "system" fn(this: *mut *mut Self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresenceData: usize,
}
#[repr(C)]
pub struct IRTCPresenceDevice {
    pub base__: ::windows_sys::core::IUnknown,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Notes: unsafe extern "system" fn(this: *mut *mut Self, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_PresenceProperty: unsafe extern "system" fn(this: *mut *mut Self, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_PresenceProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresenceData: unsafe extern "system" fn(this: *mut *mut Self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresenceData: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresencePropertyEvent {
    pub base__: super::Com::IDispatch,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
    pub PresenceProperty: unsafe extern "system" fn(this: *mut *mut Self, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresenceStatusEvent {
    pub base__: super::Com::IDispatch,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalPresenceInfo: unsafe extern "system" fn(this: *mut *mut Self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalPresenceInfo: usize,
}
#[repr(C)]
pub struct IRTCProfile {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Key: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub XML: unsafe extern "system" fn(this: *mut *mut Self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XML: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ProviderURI: unsafe extern "system" fn(this: *mut *mut Self, enuri: RTC_PROVIDER_URI, pbstruri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ProviderURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderData: unsafe extern "system" fn(this: *mut *mut Self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub ClientBanner: unsafe extern "system" fn(this: *mut *mut Self, pfbanner: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientMinVer: unsafe extern "system" fn(this: *mut *mut Self, pbstrminver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientMinVer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientCurVer: unsafe extern "system" fn(this: *mut *mut Self, pbstrcurver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientCurVer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientUpdateURI: unsafe extern "system" fn(this: *mut *mut Self, pbstrupdateuri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientUpdateURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientData: unsafe extern "system" fn(this: *mut *mut Self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserURI: unsafe extern "system" fn(this: *mut *mut Self, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut *mut Self, pbstruseraccount: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut *mut Self, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    pub SessionCapabilities: unsafe extern "system" fn(this: *mut *mut Self, plsupportedsessions: *mut i32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCProfile2 {
    pub base__: IRTCProfile,
    #[cfg(feature = "Win32_Foundation")]
    pub Realm: unsafe extern "system" fn(this: *mut *mut Self, pbstrrealm: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Realm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRealm: unsafe extern "system" fn(this: *mut *mut Self, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRealm: usize,
    pub AllowedAuth: unsafe extern "system" fn(this: *mut *mut Self, plallowedauth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAllowedAuth: unsafe extern "system" fn(this: *mut *mut Self, lallowedauth: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCProfileEvent {
    pub base__: super::Com::IDispatch,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut *mut Self, plcookie: *mut isize) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCProfileEvent2 {
    pub base__: IRTCProfileEvent,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCReInviteEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Accept: usize,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut RTC_REINVITE_STATE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRemoteSessionDescription: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCRegistrationStateChangeEvent {
    pub base__: super::Com::IDispatch,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCRoamingEvent {
    pub base__: super::Com::IDispatch,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[repr(C)]
pub struct IRTCSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub Client: unsafe extern "system" fn(this: *mut *mut Self, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_SESSION_STATE) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pentype: *mut RTC_SESSION_TYPE) -> ::windows_sys::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Participants: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Participants: usize,
    pub Answer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self, enreason: RTC_TERMINATE_REASON) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Redirect: unsafe extern "system" fn(this: *mut *mut Self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Redirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddParticipant: unsafe extern "system" fn(this: *mut *mut Self, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddParticipant: usize,
    pub RemoveParticipant: unsafe extern "system" fn(this: *mut *mut Self, pparticipant: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EnumerateParticipants: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanAddParticipants: unsafe extern "system" fn(this: *mut *mut Self, pfcanadd: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RedirectedUserURI: unsafe extern "system" fn(this: *mut *mut Self, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RedirectedUserURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RedirectedUserName: unsafe extern "system" fn(this: *mut *mut Self, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RedirectedUserName: usize,
    pub NextRedirectedUser: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendMessage: unsafe extern "system" fn(this: *mut *mut Self, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendMessage: usize,
    pub SendMessageStatus: unsafe extern "system" fn(this: *mut *mut Self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_sys::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, lcookie: isize) -> ::windows_sys::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub put_EncryptionKey: unsafe extern "system" fn(this: *mut *mut Self, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_EncryptionKey: usize,
}
#[repr(C)]
pub struct IRTCSession2 {
    pub base__: IRTCSession,
    #[cfg(feature = "Win32_Foundation")]
    pub SendInfo: unsafe extern "system" fn(this: *mut *mut Self, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendInfo: usize,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AnswerWithSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AnswerWithSessionDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReInviteWithSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReInviteWithSessionDescription: usize,
}
#[repr(C)]
pub struct IRTCSessionCallControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Hold: unsafe extern "system" fn(this: *mut *mut Self, lcookie: isize) -> ::windows_sys::core::HRESULT,
    pub UnHold: unsafe extern "system" fn(this: *mut *mut Self, lcookie: isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Forward: unsafe extern "system" fn(this: *mut *mut Self, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Forward: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Refer: unsafe extern "system" fn(this: *mut *mut Self, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Refer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReferredByURI: unsafe extern "system" fn(this: *mut *mut Self, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReferredByURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReferredByURI: unsafe extern "system" fn(this: *mut *mut Self, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReferredByURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReferCookie: unsafe extern "system" fn(this: *mut *mut Self, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReferCookie: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReferCookie: unsafe extern "system" fn(this: *mut *mut Self, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReferCookie: usize,
    pub IsReferred: unsafe extern "system" fn(this: *mut *mut Self, pfisreferred: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCSessionDescriptionManager {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub EvaluateSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EvaluateSessionDescription: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionOperationCompleteEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut *mut Self, plcookie: *mut isize) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionOperationCompleteEvent2 {
    pub base__: IRTCSessionOperationCompleteEvent,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRemoteSessionDescription: usize,
}
#[repr(C)]
pub struct IRTCSessionPortManagement {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetPortManager: unsafe extern "system" fn(this: *mut *mut Self, pportmanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionReferStatusEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReferStatus: unsafe extern "system" fn(this: *mut *mut Self, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionReferredEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReferredByURI: unsafe extern "system" fn(this: *mut *mut Self, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReferredByURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReferToURI: unsafe extern "system" fn(this: *mut *mut Self, pbstrreferouri: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReferToURI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReferCookie: unsafe extern "system" fn(this: *mut *mut Self, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReferCookie: usize,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetReferredSessionState: unsafe extern "system" fn(this: *mut *mut Self, enstate: RTC_SESSION_STATE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionStateChangeEvent {
    pub base__: super::Com::IDispatch,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_SESSION_STATE) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StatusText: unsafe extern "system" fn(this: *mut *mut Self, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StatusText: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionStateChangeEvent2 {
    pub base__: IRTCSessionStateChangeEvent,
    pub MediaTypes: unsafe extern "system" fn(this: *mut *mut Self, pmediatypes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_sys::core::HRESULT,
    pub IsForked: unsafe extern "system" fn(this: *mut *mut Self, pfisforked: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRemoteSessionDescription: unsafe extern "system" fn(this: *mut *mut Self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRemoteSessionDescription: usize,
}
#[repr(C)]
pub struct IRTCUserSearch {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateQuery: unsafe extern "system" fn(this: *mut *mut Self, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(this: *mut *mut Self, pquery: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCUserSearchQuery {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub put_SearchTerm: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_SearchTerm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SearchTerm: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SearchTerm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SearchTerms: unsafe extern "system" fn(this: *mut *mut Self, pbstrnames: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SearchTerms: usize,
    pub put_SearchPreference: unsafe extern "system" fn(this: *mut *mut Self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows_sys::core::HRESULT,
    pub get_SearchPreference: unsafe extern "system" fn(this: *mut *mut Self, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSearchDomain: unsafe extern "system" fn(this: *mut *mut Self, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSearchDomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SearchDomain: unsafe extern "system" fn(this: *mut *mut Self, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SearchDomain: usize,
}
#[repr(C)]
pub struct IRTCUserSearchResult {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Value: unsafe extern "system" fn(this: *mut *mut Self, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCUserSearchResultsEvent {
    pub base__: super::Com::IDispatch,
    pub EnumerateResults: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut *mut Self, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut *mut Self, plcookie: *mut isize) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoreAvailable: unsafe extern "system" fn(this: *mut *mut Self, pfmoreavailable: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCWatcher {
    pub base__: IRTCPresenceContact,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, penstate: *mut RTC_WATCHER_STATE) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, enstate: RTC_WATCHER_STATE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRTCWatcher2 {
    pub base__: IRTCWatcher,
    pub Profile: unsafe extern "system" fn(this: *mut *mut Self, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, penscope: *mut RTC_ACE_SCOPE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCWatcherEvent {
    pub base__: super::Com::IDispatch,
    pub Watcher: unsafe extern "system" fn(this: *mut *mut Self, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCWatcherEvent2 {
    pub base__: IRTCWatcherEvent,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows_sys::core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, plstatuscode: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransportSettingsInternal {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(this: *mut *mut Self, setting: *mut TRANSPORT_SETTING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(this: *mut *mut Self, setting: *mut TRANSPORT_SETTING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_DIGEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_KERBEROS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_NTLM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub const RTCClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2051205673, data2: 41655, data3: 16580, data4: [176, 145, 246, 240, 36, 170, 137, 190] };
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_ALL: u32 = 33554431u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_BUDDY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_BUDDY2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_GROUP: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_INFO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_INTENSITY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MEDIA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_MESSAGING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_PROFILE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_REINVITE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_ROAMING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_USERSEARCH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_WATCHER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCEF_WATCHER2: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_APPLICATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_IM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PC_TO_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TCP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TLS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_UDP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_ACE_SCOPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_ANSWER_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_AUDIO_DEVICE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_BUDDY_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_BUDDY_SUBSCRIPTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_CLIENT_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_DTMF = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_0: RTC_DTMF = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_1: RTC_DTMF = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_2: RTC_DTMF = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_3: RTC_DTMF = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_4: RTC_DTMF = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_5: RTC_DTMF = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_6: RTC_DTMF = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_7: RTC_DTMF = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_8: RTC_DTMF = 8i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_9: RTC_DTMF = 9i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_STAR: RTC_DTMF = 10i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_POUND: RTC_DTMF = 11i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_A: RTC_DTMF = 12i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_B: RTC_DTMF = 13i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_C: RTC_DTMF = 14i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_D: RTC_DTMF = 15i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_DTMF_FLASH: RTC_DTMF = 16i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_EVENT = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_CLIENT: RTC_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MEDIA: RTC_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_INTENSITY: RTC_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MESSAGING: RTC_EVENT = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_BUDDY: RTC_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_WATCHER: RTC_EVENT = 9i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PROFILE: RTC_EVENT = 10i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_USERSEARCH: RTC_EVENT = 11i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_INFO: RTC_EVENT = 12i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_GROUP: RTC_EVENT = 13i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = 14i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_ROAMING: RTC_EVENT = 15i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = 16i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_DATA: RTC_EVENT = 17i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = 18i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = 19i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_SESSION_REFERRED: RTC_EVENT = 20i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCE_REINVITE: RTC_EVENT = 21i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows_sys::core::HRESULT = -2131885961i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows_sys::core::HRESULT = -2131886017i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2131886042i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows_sys::core::HRESULT = -2131886041i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2131886043i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows_sys::core::HRESULT = -2131886061i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows_sys::core::HRESULT = -2131886059i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_BUDDY: ::windows_sys::core::HRESULT = -2131886006i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_GROUP: ::windows_sys::core::HRESULT = -2131885998i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_REALM: ::windows_sys::core::HRESULT = -2131886013i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_DUPLICATE_WATCHER: ::windows_sys::core::HRESULT = -2131886005i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_ACL_LIST: ::windows_sys::core::HRESULT = -2131886000i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows_sys::core::HRESULT = -2131886060i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_BUDDY_LIST: ::windows_sys::core::HRESULT = -2131886001i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows_sys::core::HRESULT = -2131885957i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_OBJECT_STATE: ::windows_sys::core::HRESULT = -2131885983i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PORTRANGE: ::windows_sys::core::HRESULT = -2131885988i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows_sys::core::HRESULT = -2131885991i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PROFILE: ::windows_sys::core::HRESULT = -2131886034i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows_sys::core::HRESULT = -2131886058i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows_sys::core::HRESULT = -2131885971i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SESSION_STATE: ::windows_sys::core::HRESULT = -2131886038i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SESSION_TYPE: ::windows_sys::core::HRESULT = -2131886039i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_INVALID_SIP_URL: ::windows_sys::core::HRESULT = -2131886062i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows_sys::core::HRESULT = -2131885958i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows_sys::core::HRESULT = -2131886036i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MALFORMED_XML: ::windows_sys::core::HRESULT = -2131886004i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows_sys::core::HRESULT = -2131885990i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MAX_REDIRECTS: ::windows_sys::core::HRESULT = -2131885960i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_AEC: ::windows_sys::core::HRESULT = -2131886044i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2131886047i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows_sys::core::HRESULT = -2131886049i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_DISABLED: ::windows_sys::core::HRESULT = -2131885970i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_ENABLED: ::windows_sys::core::HRESULT = -2131885969i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows_sys::core::HRESULT = -2131886048i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows_sys::core::HRESULT = -2131885962i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows_sys::core::HRESULT = -2131885963i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2131886046i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2131885950i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_EXIST: ::windows_sys::core::HRESULT = -2131885992i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows_sys::core::HRESULT = -2131885974i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_BUDDY: ::windows_sys::core::HRESULT = -2131885996i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_DEVICE: ::windows_sys::core::HRESULT = -2131886035i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_GROUP: ::windows_sys::core::HRESULT = -2131885999i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_PROFILE: ::windows_sys::core::HRESULT = -2131886037i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_REALM: ::windows_sys::core::HRESULT = -2131885994i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_TRANSPORT: ::windows_sys::core::HRESULT = -2131885993i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_NO_WATCHER: ::windows_sys::core::HRESULT = -2131885995i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows_sys::core::HRESULT = -2131886018i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows_sys::core::HRESULT = -2131755001i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows_sys::core::HRESULT = -2131754997i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows_sys::core::HRESULT = -2131755003i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows_sys::core::HRESULT = -2131754998i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows_sys::core::HRESULT = -2131755002i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows_sys::core::HRESULT = -2131755000i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows_sys::core::HRESULT = -2131754999i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2131885952i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_POLICY_NOT_ALLOW: ::windows_sys::core::HRESULT = -2131886012i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows_sys::core::HRESULT = -2131885956i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MAPPING_FAILED: ::windows_sys::core::HRESULT = -2131886010i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows_sys::core::HRESULT = -2131886011i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PRESENCE_ENABLED: ::windows_sys::core::HRESULT = -2131885982i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows_sys::core::HRESULT = -2131886040i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows_sys::core::HRESULT = -2131886024i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows_sys::core::HRESULT = -2131886025i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows_sys::core::HRESULT = -2131886023i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows_sys::core::HRESULT = -2131886021i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows_sys::core::HRESULT = -2131886020i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows_sys::core::HRESULT = -2131886019i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows_sys::core::HRESULT = -2131886022i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_KEY: ::windows_sys::core::HRESULT = -2131886032i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_NAME: ::windows_sys::core::HRESULT = -2131886031i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_PROVISION: ::windows_sys::core::HRESULT = -2131886033i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER: ::windows_sys::core::HRESULT = -2131886028i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows_sys::core::HRESULT = -2131886027i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows_sys::core::HRESULT = -2131886026i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_USER: ::windows_sys::core::HRESULT = -2131886030i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_NO_USER_URI: ::windows_sys::core::HRESULT = -2131886029i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows_sys::core::HRESULT = -2131886014i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows_sys::core::HRESULT = -2131885959i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows_sys::core::HRESULT = -2131885968i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2131885967i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REFER_NOT_EXIST: ::windows_sys::core::HRESULT = -2131885966i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows_sys::core::HRESULT = -2131885949i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_REJECTED: ::windows_sys::core::HRESULT = -2131885948i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows_sys::core::HRESULT = -2131885947i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_ENABLED: ::windows_sys::core::HRESULT = -2131885981i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_FAILED: ::windows_sys::core::HRESULT = -2131886002i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows_sys::core::HRESULT = -2131886003i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows_sys::core::HRESULT = -2131886070i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows_sys::core::HRESULT = -2131886067i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_MULTICAST: ::windows_sys::core::HRESULT = -2131886071i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_NOT_PRESENT: ::windows_sys::core::HRESULT = -2131886074i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_NO_MEDIA: ::windows_sys::core::HRESULT = -2131886069i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_PARSE_FAILED: ::windows_sys::core::HRESULT = -2131886073i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SDP_UPDATE_FAILED: ::windows_sys::core::HRESULT = -2131886072i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows_sys::core::HRESULT = -2131885955i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows_sys::core::HRESULT = -2131886009i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows_sys::core::HRESULT = -2131886008i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows_sys::core::HRESULT = -2131886007i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows_sys::core::HRESULT = -2131885986i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_FAILED: ::windows_sys::core::HRESULT = -2131886063i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows_sys::core::HRESULT = -2131886065i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows_sys::core::HRESULT = -2131885972i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2131886064i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows_sys::core::HRESULT = -2131885987i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows_sys::core::HRESULT = -2131886055i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows_sys::core::HRESULT = -2131886080i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_DNS_FAIL: ::windows_sys::core::HRESULT = -2131885978i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows_sys::core::HRESULT = -2131886075i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows_sys::core::HRESULT = -2131886016i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows_sys::core::HRESULT = -2131885965i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = -2131885979i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows_sys::core::HRESULT = -2131885973i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows_sys::core::HRESULT = -2131886066i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows_sys::core::HRESULT = -2131886056i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_NO_STREAM: ::windows_sys::core::HRESULT = -2131886077i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows_sys::core::HRESULT = -2131885984i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PARSE_FAILED: ::windows_sys::core::HRESULT = -2131886076i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows_sys::core::HRESULT = -2131885985i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows_sys::core::HRESULT = -2131885951i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows_sys::core::HRESULT = -2131885953i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows_sys::core::HRESULT = -2131886054i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows_sys::core::HRESULT = -2131886051i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows_sys::core::HRESULT = -2131886052i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows_sys::core::HRESULT = -2131886050i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows_sys::core::HRESULT = -2131886078i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_STREAM_PRESENT: ::windows_sys::core::HRESULT = -2131886079i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TCP_FAIL: ::windows_sys::core::HRESULT = -2131885977i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TIMEOUT: ::windows_sys::core::HRESULT = -2131886068i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TLS_FAIL: ::windows_sys::core::HRESULT = -2131885975i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows_sys::core::HRESULT = -2131885980i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2131886057i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = -2131886053i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows_sys::core::HRESULT = -2131885964i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_START_STREAM: ::windows_sys::core::HRESULT = -2131886045i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows_sys::core::HRESULT = -2131820060i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows_sys::core::HRESULT = -2131820059i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows_sys::core::HRESULT = -2131820124i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows_sys::core::HRESULT = -2131820144i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows_sys::core::HRESULT = -2131820058i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows_sys::core::HRESULT = -2131820135i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows_sys::core::HRESULT = -2131820141i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_GONE: ::windows_sys::core::HRESULT = -2131820134i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows_sys::core::HRESULT = -2131820133i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows_sys::core::HRESULT = -2131820062i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2131820139i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows_sys::core::HRESULT = -2131820138i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows_sys::core::HRESULT = -2131820140i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows_sys::core::HRESULT = -2131820142i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows_sys::core::HRESULT = -2131820137i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows_sys::core::HRESULT = -2131820131i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows_sys::core::HRESULT = -2131820136i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows_sys::core::HRESULT = -2131820130i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2131820064i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows_sys::core::HRESULT = -2131820061i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows_sys::core::HRESULT = -2131820063i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows_sys::core::HRESULT = -2131820143i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows_sys::core::HRESULT = -2131820129i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows_sys::core::HRESULT = -2131819944i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows_sys::core::HRESULT = -2131819941i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows_sys::core::HRESULT = -2131819940i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows_sys::core::HRESULT = -2131819938i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows_sys::core::HRESULT = 15663285i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_QUEUED: ::windows_sys::core::HRESULT = 15663286i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_RINGING: ::windows_sys::core::HRESULT = 15663284i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_INFO_TRYING: ::windows_sys::core::HRESULT = 15663204i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows_sys::core::HRESULT = -2131820056i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows_sys::core::HRESULT = -2131820164i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows_sys::core::HRESULT = -2131820243i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows_sys::core::HRESULT = -2131820242i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows_sys::core::HRESULT = -2131820244i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows_sys::core::HRESULT = -2131820241i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows_sys::core::HRESULT = -2131820239i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows_sys::core::HRESULT = -2131820057i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows_sys::core::HRESULT = -2131820042i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -2131820044i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows_sys::core::HRESULT = -2131820043i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows_sys::core::HRESULT = -2131820040i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows_sys::core::HRESULT = -2131820041i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2131820039i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows_sys::core::HRESULT = 15663287i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_STATUS_SUCCESS: ::windows_sys::core::HRESULT = 15663304i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_MANY_GROUPS: ::windows_sys::core::HRESULT = -2131885997i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_MANY_RETRIES: ::windows_sys::core::HRESULT = -2131885989i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows_sys::core::HRESULT = -2131885976i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2131885954i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_GROUP_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_LISTEN_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_NONE: RTC_LISTEN_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCLM_BOTH: RTC_LISTEN_MODE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_MEDIA_EVENT_REASON = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_MEDIA_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_MESSAGING_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_MESSAGING_USER_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_OFFER_WATCHER_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PARTICIPANT_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPT_SIP: RTC_PORT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PRESENCE_PROPERTY = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PRESENCE_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PRIVACY_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PROFILE_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_PROVIDER_URI = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_REGISTRATION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_REINVITE_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_RING_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_PHONE: RTC_RING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_MESSAGE: RTC_RING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRT_RINGBACK: RTC_RING_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_ROAMING_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_SECURITY_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_SECURITY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_SESSION_REFER_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_SESSION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_IDLE: RTC_SESSION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_INCOMING: RTC_SESSION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_HOLD: RTC_SESSION_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCSS_REFER: RTC_SESSION_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_SESSION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_IM: RTC_SESSION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows_sys::core::HRESULT = 15597633i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_T120_APPLET = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTA_APPSHARING: RTC_T120_APPLET = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_TERMINATE_REASON = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_DND: RTC_TERMINATE_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_USER_SEARCH_COLUMN = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = 6i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = 7i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = 8i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = 9i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_USER_SEARCH_PREFERENCE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_VIDEO_DEVICE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_WATCHER_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_WATCHER_MATCH_MODE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub type RTC_WATCHER_STATE = i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_DENIED: RTC_WATCHER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`*"]
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct TRANSPORT_SETTING {
    pub SettingId: super::super::Networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for TRANSPORT_SETTING {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for TRANSPORT_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
