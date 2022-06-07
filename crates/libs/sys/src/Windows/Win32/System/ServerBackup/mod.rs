#[repr(C)]
pub struct IWsbApplicationAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryStatus: unsafe extern "system" fn(this: *mut *mut Self, phrresult: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWsbApplicationAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 138671863, data2: 35164, data3: 17574, data4: [176, 194, 5, 165, 2, 42, 163, 161] };
}
#[repr(C)]
pub struct IWsbApplicationBackupSupport {
    pub base__: ::windows_sys::core::IUnknown,
    pub CheckConsistency: unsafe extern "system" fn(this: *mut *mut Self, wszwritermetadata: ::windows_sys::core::PCWSTR, wszcomponentname: ::windows_sys::core::PCWSTR, wszcomponentlogicalpath: ::windows_sys::core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows_sys::core::PWSTR, rgwszsnapshotvolumepath: *const ::windows_sys::core::PWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWsbApplicationBackupSupport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520041744, data2: 18983, data3: 18093, data4: [185, 224, 8, 51, 47, 15, 79, 109] };
}
#[repr(C)]
pub struct IWsbApplicationRestoreSupport {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub PreRestore: unsafe extern "system" fn(this: *mut *mut Self, wszwritermetadata: ::windows_sys::core::PCWSTR, wszcomponentname: ::windows_sys::core::PCWSTR, wszcomponentlogicalpath: ::windows_sys::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostRestore: unsafe extern "system" fn(this: *mut *mut Self, wszwritermetadata: ::windows_sys::core::PCWSTR, wszcomponentname: ::windows_sys::core::PCWSTR, wszcomponentlogicalpath: ::windows_sys::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostRestore: usize,
    pub OrderComponents: unsafe extern "system" fn(this: *mut *mut Self, ccomponents: u32, rgcomponentname: *const ::windows_sys::core::PWSTR, rgcomponentlogicalpaths: *const ::windows_sys::core::PWSTR, prgcomponentname: *mut *mut ::windows_sys::core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub IsRollForwardSupported: unsafe extern "system" fn(this: *mut *mut Self, pbrollforwardsupported: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWsbApplicationRestoreSupport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2369510200, data2: 20200, data3: 18200, data4: [133, 249, 199, 219, 196, 171, 119, 170] };
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSBAPP_ASYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = 7995396i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSB_OB_REGISTRATION_INFO {
    pub m_wszResourceDLL: ::windows_sys::core::PWSTR,
    pub m_guidSnapinId: ::windows_sys::core::GUID,
    pub m_dwProviderName: u32,
    pub m_dwProviderIcon: u32,
    pub m_bSupportsRemoting: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSB_OB_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSB_OB_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_ENTRY {
    pub m_dwIcon: u32,
    pub m_dwStatusEntryName: u32,
    pub m_dwStatusEntryValue: u32,
    pub m_cValueTypePair: u32,
    pub m_rgValueTypePair: *mut WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub type WSB_OB_STATUS_ENTRY_PAIR_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    pub m_wszObStatusEntryPairValue: ::windows_sys::core::PWSTR,
    pub m_ObStatusEntryPairType: WSB_OB_STATUS_ENTRY_PAIR_TYPE,
}
impl ::core::marker::Copy for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {}
impl ::core::clone::Clone for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`*"]
pub struct WSB_OB_STATUS_INFO {
    pub m_guidSnapinId: ::windows_sys::core::GUID,
    pub m_cStatusEntry: u32,
    pub m_rgStatusEntry: *mut WSB_OB_STATUS_ENTRY,
}
impl ::core::marker::Copy for WSB_OB_STATUS_INFO {}
impl ::core::clone::Clone for WSB_OB_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
