#[link(name = "windows")]
extern "system" {
    pub fn RegisterLicenseKeyWithExpiration(licensekey: ::windows_core_sys::PCWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_core_sys::HRESULT;
    pub fn ValidateLicenseKeyProtection(licensekey: ::windows_core_sys::PCWSTR, notvalidbefore: *mut ::win32_foundation_sys::FILETIME, notvalidafter: *mut ::win32_foundation_sys::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_core_sys::HRESULT;
}
pub type LicenseProtectionStatus = i32;
pub const Success: LicenseProtectionStatus = 0i32;
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;
