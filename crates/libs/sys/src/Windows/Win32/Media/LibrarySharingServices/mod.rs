#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsMediaLibrarySharingDevice {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceID: unsafe extern "system" fn(this: *mut *mut Self, deviceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceID: usize,
    pub Authorization: unsafe extern "system" fn(this: *mut *mut Self, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_sys::core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut *mut Self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, deviceproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsMediaLibrarySharingDeviceProperties {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsMediaLibrarySharingDeviceProperty {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsMediaLibrarySharingDevices {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, device: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, device: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDevice: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsMediaLibrarySharingServices {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub showShareMediaCPL: unsafe extern "system" fn(this: *mut *mut Self, device: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    showShareMediaCPL: usize,
    pub userHomeMediaSharingState: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetuserHomeMediaSharingState: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub userHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut *mut Self, libraryname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    userHomeMediaSharingLibraryName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetuserHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut *mut Self, libraryname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetuserHomeMediaSharingLibraryName: usize,
    pub computerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut *mut Self, sharingallowed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetcomputerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut *mut Self, sharingallowed: i16) -> ::windows_sys::core::HRESULT,
    pub userInternetMediaSharingState: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetuserInternetMediaSharingState: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: i16) -> ::windows_sys::core::HRESULT,
    pub computerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut *mut Self, sharingallowed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetcomputerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut *mut Self, sharingallowed: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub internetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut *mut Self, securitygroup: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    internetMediaSharingSecurityGroup: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetinternetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut *mut Self, securitygroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetinternetMediaSharingSecurityGroup: usize,
    pub allowSharingToAllDevices: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetallowSharingToAllDevices: unsafe extern "system" fn(this: *mut *mut Self, sharingenabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub setDefaultAuthorization: unsafe extern "system" fn(this: *mut *mut Self, macaddresses: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorization: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setDefaultAuthorization: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setAuthorizationState: unsafe extern "system" fn(this: *mut *mut Self, macaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationstate: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setAuthorizationState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAllDevices: unsafe extern "system" fn(this: *mut *mut Self, devices: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAllDevices: usize,
    pub customSettingsApplied: unsafe extern "system" fn(this: *mut *mut Self, customsettingsapplied: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub type WindowsMediaLibrarySharingDeviceAuthorizationStatus = i32;
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 0i32;
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 1i32;
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = 2i32;
pub const WindowsMediaLibrarySharingServices: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2908232448, data2: 31588, data3: 20057, data4: [163, 141, 210, 197, 191, 81, 221, 179] };
