pub type DualSimTile = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDualSimTile {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsPinnedToStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
}
impl ::windows_sys::core::Interface for IDualSimTile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 339390995, data2: 53343, data3: 16449, data4: [161, 140, 62, 63, 203, 117, 180, 30] };
}
#[repr(C)]
pub struct IDualSimTileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetTileForSim2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateDisplayNameForSim1Async: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateDisplayNameForSim1Async: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateTileUpdaterForSim2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateTileUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateBadgeUpdaterForSim2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateBadgeUpdaterForSim2: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim1: usize,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSim2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSim2: usize,
}
impl ::windows_sys::core::Interface for IDualSimTileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1347845278, data2: 50575, data3: 19913, data4: [182, 232, 250, 103, 119, 238, 235, 55] };
}
#[repr(C)]
pub struct IToastNotificationManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Notifications")]
    pub CreateToastNotifierForSecondaryTile: unsafe extern "system" fn(this: *mut *mut Self, tileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    CreateToastNotifierForSecondaryTile: usize,
}
impl ::windows_sys::core::Interface for IToastNotificationManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 655881547, data2: 20703, data3: 17493, data4: [142, 110, 65, 224, 252, 142, 19, 206] };
}
