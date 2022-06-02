#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableA(lpszdestination: ::windows_sys::core::PCSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableW(lpszdestination: ::windows_sys::core::PCWSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_AOL: u32 = 4u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensLogon {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Logon: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Logoff: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logoff: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartShell: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartShell: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayLock: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayLock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUnlock: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUnlock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartScreenSaver: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartScreenSaver: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StopScreenSaver: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StopScreenSaver: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensLogon2 {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Logon: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Logoff: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Logoff: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionDisconnect: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionDisconnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionReconnect: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionReconnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostShell: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostShell: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensNetwork {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionMade: unsafe extern "system" fn(this: *mut *mut Self, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionMade: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionMadeNoQOCInfo: unsafe extern "system" fn(this: *mut *mut Self, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionMadeNoQOCInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionLost: unsafe extern "system" fn(this: *mut *mut Self, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionLost: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestinationReachable: unsafe extern "system" fn(this: *mut *mut Self, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestinationReachable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestinationReachableNoQOCInfo: unsafe extern "system" fn(this: *mut *mut Self, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestinationReachableNoQOCInfo: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensOnNow {
    pub base__: super::Com::IDispatch,
    pub OnACPower: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub OnBatteryPower: unsafe extern "system" fn(this: *mut *mut Self, dwbatterylifepercent: u32) -> ::windows_sys::core::HRESULT,
    pub BatteryLow: unsafe extern "system" fn(this: *mut *mut Self, dwbatterylifepercent: u32) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
impl ::core::marker::Copy for QOCINFO {}
impl ::core::clone::Clone for QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SENS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583494910, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_LOGON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583477296, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_LOGON2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583477328, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_NETWORK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583477280, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_ONNOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583477312, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_PUBLISHER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1609440214, data2: 23451, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_LCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3549661872, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_WININET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3549661877, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub type SENS_CONNECTION_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`*"]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
impl ::core::marker::Copy for SENS_QOCINFO {}
impl ::core::clone::Clone for SENS_QOCINFO {
    fn clone(&self) -> Self {
        *self
    }
}
