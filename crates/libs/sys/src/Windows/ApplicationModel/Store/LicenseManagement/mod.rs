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
impl ::windows_sys::core::Interface for ILicenseManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3047963360, data2: 55879, data3: 20256, data4: [154, 35, 9, 24, 44, 148, 118, 255] };
}
#[repr(C)]
pub struct ILicenseManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RefreshLicensesAsync: unsafe extern "system" fn(this: *mut *mut Self, refreshoption: LicenseRefreshOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshLicensesAsync: usize,
}
impl ::windows_sys::core::Interface for ILicenseManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2871968891, data2: 8057, data3: 17536, data4: [184, 126, 44, 73, 158, 96, 27, 163] };
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
impl ::windows_sys::core::Interface for ILicenseSatisfactionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1019981967, data2: 56113, data3: 18645, data4: [131, 132, 250, 23, 200, 20, 116, 226] };
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
impl ::windows_sys::core::Interface for ILicenseSatisfactionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1013403507, data2: 15495, data3: 20193, data4: [130, 1, 244, 40, 53, 155, 211, 175] };
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
