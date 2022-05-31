#[link(name = "windows")]
extern "system" {
    pub fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    pub fn FileTimeToSystemTime(lpfiletime: *const ::win32_foundation_sys::FILETIME, lpsystemtime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
    pub fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    pub fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32;
    pub fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32;
    pub fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn LocalFileTimeToLocalSystemTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const ::win32_foundation_sys::FILETIME, localsystemtime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
    pub fn LocalSystemTimeToLocalFileTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const ::win32_foundation_sys::SYSTEMTIME, localfiletime: *mut ::win32_foundation_sys::FILETIME) -> ::win32_foundation_sys::BOOL;
    pub fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> ::win32_foundation_sys::BOOL;
    pub fn SystemTimeToFileTime(lpsystemtime: *const ::win32_foundation_sys::SYSTEMTIME, lpfiletime: *mut ::win32_foundation_sys::FILETIME) -> ::win32_foundation_sys::BOOL;
    pub fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const ::win32_foundation_sys::SYSTEMTIME, lplocaltime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
    pub fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const ::win32_foundation_sys::SYSTEMTIME, lplocaltime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
    pub fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const ::win32_foundation_sys::SYSTEMTIME, lpuniversaltime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
    pub fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const ::win32_foundation_sys::SYSTEMTIME, lpuniversaltime: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::win32_foundation_sys::BOOL;
}
#[repr(C)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: ::win32_foundation_sys::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: ::win32_foundation_sys::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: ::win32_foundation_sys::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: ::win32_foundation_sys::SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TSF_Authenticated: u32 = 2u32;
pub const TSF_Hardware: u32 = 1u32;
pub const TSF_IPv6: u32 = 4u32;
pub const TSF_SignatureAuthenticated: u32 = 8u32;
pub const wszW32TimeRegKeyPolicyTimeProviders: &str = "Software\\Policies\\Microsoft\\W32Time\\TimeProviders";
pub const wszW32TimeRegKeyTimeProviders: &str = "System\\CurrentControlSet\\Services\\W32Time\\TimeProviders";
pub const wszW32TimeRegValueDllName: &str = "DllName";
pub const wszW32TimeRegValueEnabled: &str = "Enabled";
pub const wszW32TimeRegValueInputProvider: &str = "InputProvider";
pub const wszW32TimeRegValueMetaDataProvider: &str = "MetaDataProvider";
