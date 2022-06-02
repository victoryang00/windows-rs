#[repr(C)]
pub struct ILicenseManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AddLicenseAsync: unsafe extern "system" fn(this: *mut *mut Self, license: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AddLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSatisfactionInfosAsync: unsafe extern "system" fn(this: *mut *mut Self, contentids: *mut ::core::ffi::c_void, keyids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSatisfactionInfosAsync: usize,
}
#[repr(C)]
pub struct ILicenseManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RefreshLicensesAsync: unsafe extern "system" fn(this: *mut *mut Self, refreshoption: LicenseRefreshOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshLicensesAsync: usize,
}
#[repr(C)]
pub struct ILicenseSatisfactionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SatisfiedByDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SatisfiedByOpenLicense: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SatisfiedByTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SatisfiedByPass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SatisfiedByInstallMedia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SatisfiedBySignedInUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSatisfied: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILicenseSatisfactionResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub LicenseSatisfactionInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LicenseSatisfactionInfos: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: Self = Self(0i32);
    pub const AllLicenses: Self = Self(1i32);
}
impl ::core::marker::Copy for LicenseRefreshOption {}
impl ::core::clone::Clone for LicenseRefreshOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LicenseSatisfactionInfo = *mut ::core::ffi::c_void;
pub type LicenseSatisfactionResult = *mut ::core::ffi::c_void;
