#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub EndpointId: ::windows_sys::core::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl ::core::marker::Copy for APPLICATION_EVENT_DATA {}
impl ::core::clone::Clone for APPLICATION_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const CONTENT_ID_GLANCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const CONTENT_ID_HOME: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub EndpointId: ::windows_sys::core::GUID,
    pub ContentId: u32,
}
impl ::core::marker::Copy for CONTENT_MISSING_EVENT_DATA {}
impl ::core::clone::Clone for CONTENT_MISSING_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
impl ::core::marker::Copy for DEVICE_USER_CHANGE_EVENT_DATA {}
impl ::core::clone::Clone for DEVICE_USER_CHANGE_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: ::windows_sys::core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl ::core::marker::Copy for EVENT_DATA_HEADER {}
impl ::core::clone::Clone for EVENT_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355358737, data2: 65209, data3: 19200, data4: [144, 244, 211, 41, 71, 174, 22, 129] };
#[repr(C)]
pub struct ISideShowBulkCapabilities {
    pub base__: ISideShowCapabilities,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, in_keycollection: *mut ::core::ffi::c_void, inout_pvalues: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowBulkCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 975929276, data2: 15061, data3: 18621, data4: [187, 241, 14, 108, 251, 209, 8, 7] };
}
#[repr(C)]
pub struct ISideShowCapabilities {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetCapability: unsafe extern "system" fn(this: *mut *mut Self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetCapability: usize,
}
impl ::windows_sys::core::Interface for ISideShowCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1398674297, data2: 49310, data3: 19028, data4: [165, 17, 89, 123, 171, 58, 114, 184] };
}
#[repr(C)]
pub struct ISideShowCapabilitiesCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, out_pdwcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, in_dwindex: u32, out_ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowCapabilitiesCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1345344919, data2: 24077, data3: 20471, data4: [179, 175, 51, 208, 217, 189, 82, 221] };
}
#[repr(C)]
pub struct ISideShowContent {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetContent: unsafe extern "system" fn(this: *mut *mut Self, in_picapabilities: *mut ::core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut *mut Self, out_pcontentid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DifferentiateContent: unsafe extern "system" fn(this: *mut *mut Self, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DifferentiateContent: usize,
}
impl ::windows_sys::core::Interface for ISideShowContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3246740205, data2: 29951, data3: 20460, data4: [190, 7, 76, 254, 210, 157, 72, 135] };
}
#[repr(C)]
pub struct ISideShowContentManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, in_picontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, in_contentid: u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(this: *mut *mut Self, in_pievents: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, out_ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowContentManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2782246507, data2: 61177, data3: 16859, data4: [141, 126, 225, 124, 51, 171, 16, 176] };
}
#[repr(C)]
pub struct ISideShowEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub ContentMissing: unsafe extern "system" fn(this: *mut *mut Self, in_contentid: u32, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplicationEvent: unsafe extern "system" fn(this: *mut *mut Self, in_picapabilities: *mut ::core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows_sys::core::HRESULT,
    pub DeviceAdded: unsafe extern "system" fn(this: *mut *mut Self, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut *mut Self, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1644087884, data2: 57012, data3: 19070, data4: [141, 117, 81, 241, 19, 45, 97, 91] };
}
#[repr(C)]
pub struct ISideShowKeyCollection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelems: *const u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowKeyCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 72643516, data2: 41851, data3: 18775, data4: [177, 68, 104, 16, 84, 17, 237, 142] };
}
#[repr(C)]
pub struct ISideShowNotification {
    pub base__: ::windows_sys::core::IUnknown,
    pub NotificationId: unsafe extern "system" fn(this: *mut *mut Self, out_pnotificationid: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetNotificationId: unsafe extern "system" fn(this: *mut *mut Self, in_notificationid: u32) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, out_ppwsztitle: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, in_pwsztitle: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, out_ppwszmessage: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut *mut Self, in_pwszmessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Image: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExpirationTime: usize,
}
impl ::windows_sys::core::Interface for ISideShowNotification {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 63517440, data2: 35506, data3: 16837, data4: [155, 121, 70, 18, 122, 48, 225, 72] };
}
#[repr(C)]
pub struct ISideShowNotificationManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, in_pinotification: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Revoke: unsafe extern "system" fn(this: *mut *mut Self, in_notificationid: u32) -> ::windows_sys::core::HRESULT,
    pub RevokeAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowNotificationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1674488073, data2: 62137, data3: 17154, data4: [181, 225, 198, 142, 109, 154, 184, 51] };
}
#[repr(C)]
pub struct ISideShowPropVariantCollection {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcelems: *const u32) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowPropVariantCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 782738761, data2: 31743, data3: 19118, data4: [186, 176, 34, 212, 49, 17, 222, 73] };
}
#[repr(C)]
pub struct ISideShowSession {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterContent: unsafe extern "system" fn(this: *mut *mut Self, in_applicationid: *const ::windows_sys::core::GUID, in_endpointid: *const ::windows_sys::core::GUID, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterNotifications: unsafe extern "system" fn(this: *mut *mut Self, in_applicationid: *const ::windows_sys::core::GUID, out_ppinotification: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISideShowSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3793957358, data2: 40573, data3: 18722, data4: [159, 194, 171, 122, 164, 28, 228, 145] };
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
impl ::core::marker::Copy for NEW_EVENT_DATA_AVAILABLE {}
impl ::core::clone::Clone for NEW_EVENT_DATA_AVAILABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SCF_BUTTON_IDS = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = 2i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = 3i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = 4i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = 5i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = 6i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = 7i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = 8i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = 9i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = 10i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = 11i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = 65280i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
impl ::core::marker::Copy for SCF_CONTEXTMENU_EVENT {}
impl ::core::clone::Clone for SCF_CONTEXTMENU_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
impl ::core::marker::Copy for SCF_EVENT_HEADER {}
impl ::core::clone::Clone for SCF_EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SCF_EVENT_IDS = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = 2i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
impl ::core::marker::Copy for SCF_MENUACTION_EVENT {}
impl ::core::clone::Clone for SCF_MENUACTION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
impl ::core::marker::Copy for SCF_NAVIGATION_EVENT {}
impl ::core::clone::Clone for SCF_NAVIGATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIDESHOW_APPLICATION_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1286959866, data2: 7483, data3: 18867, data4: [161, 122, 46, 107, 255, 5, 40, 84] };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 1u32 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SIDESHOW_COLOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = 2i32;
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1342700456, data2: 54035, data3: 17311, data4: [190, 162, 165, 2, 1, 211, 233, 168] };
pub const SIDESHOW_ENDPOINT_ICAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1308571317, data2: 40414, data3: 20342, data4: [154, 42, 150, 67, 80, 71, 6, 61] };
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2846176575, data2: 11595, data3: 18382, data4: [147, 238, 117, 159, 58, 125, 218, 79] };
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1468086356, data2: 12225, data3: 16668, data4: [165, 159, 242, 73, 39, 96, 136, 4] };
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SIDESHOW_SCREEN_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = 1i32;
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1342793532, data2: 16253, data3: 19582, data4: [153, 113, 234, 162, 233, 31, 21, 117] };
pub const SideShowKeyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3753630712, data2: 6366, data3: 18872, data4: [131, 220, 235, 199, 39, 198, 45, 148] };
pub const SideShowNotification: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 216262767, data2: 54733, data3: 17701, data4: [167, 102, 26, 186, 177, 167, 82, 245] };
pub const SideShowPropVariantCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3863016469, data2: 21406, data3: 18723, data4: [150, 205, 95, 9, 59, 194, 80, 205] };
pub const SideShowSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3791995833, data2: 63365, data3: 20130, data4: [152, 30, 196, 255, 167, 107, 188, 124] };
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
