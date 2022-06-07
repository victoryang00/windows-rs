#[repr(C)]
pub struct ILockApplicationHost {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestUnlock: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Unlocking: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Unlocking: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnlocking: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnlocking: usize,
}
impl ::windows_sys::core::Interface for ILockApplicationHost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 955134381, data2: 55631, data3: 20092, data4: [129, 250, 79, 68, 54, 80, 98, 129] };
}
#[repr(C)]
pub struct ILockApplicationHostStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockApplicationHostStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4103056270, data2: 9175, data3: 20067, data4: [150, 161, 102, 111, 245, 45, 59, 44] };
}
#[repr(C)]
pub struct ILockScreenBadge {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Glyph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Glyph: usize,
    #[cfg(feature = "Foundation")]
    pub Number: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Number: usize,
    pub AutomationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LaunchApp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenBadge {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3914401241, data2: 11263, data3: 19888, data4: [155, 79, 56, 36, 119, 139, 156, 154] };
}
#[repr(C)]
pub struct ILockScreenInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LockScreenImageChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockScreenImageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLockScreenImageChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLockScreenImageChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LockScreenImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LockScreenImage: usize,
    #[cfg(feature = "Foundation")]
    pub BadgesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BadgesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBadgesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBadgesChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Badges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Badges: usize,
    #[cfg(feature = "Foundation")]
    pub DetailTextChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DetailTextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDetailTextChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDetailTextChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DetailText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DetailText: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmIconChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmIconChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAlarmIconChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAlarmIconChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AlarmIcon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AlarmIcon: usize,
}
impl ::windows_sys::core::Interface for ILockScreenInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4120553052, data2: 38673, data3: 19913, data4: [166, 48, 149, 182, 203, 140, 218, 208] };
}
#[repr(C)]
pub struct ILockScreenUnlockingDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenUnlockingDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2122128086, data2: 20995, data3: 17383, data4: [155, 214, 124, 57, 71, 209, 227, 254] };
}
#[repr(C)]
pub struct ILockScreenUnlockingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
impl ::windows_sys::core::Interface for ILockScreenUnlockingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1155973127, data2: 30203, data3: 19131, data4: [159, 139, 130, 71, 72, 144, 12, 113] };
}
pub type LockApplicationHost = *mut ::core::ffi::c_void;
pub type LockScreenBadge = *mut ::core::ffi::c_void;
pub type LockScreenInfo = *mut ::core::ffi::c_void;
pub type LockScreenUnlockingDeferral = *mut ::core::ffi::c_void;
pub type LockScreenUnlockingEventArgs = *mut ::core::ffi::c_void;
