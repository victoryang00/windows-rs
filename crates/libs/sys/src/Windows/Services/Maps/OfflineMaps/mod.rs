#[repr(C)]
pub struct IOfflineMapPackage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OfflineMapPackageStatus) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EnclosingRegionName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EstimatedSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartDownloadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartDownloadAsync: usize,
}
impl ::windows_sys::core::Interface for IOfflineMapPackage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811717435, data2: 42421, data3: 16708, data4: [181, 37, 230, 140, 136, 98, 102, 75] };
}
#[repr(C)]
pub struct IOfflineMapPackageQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OfflineMapPackageQueryStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Packages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Packages: usize,
}
impl ::windows_sys::core::Interface for IOfflineMapPackageQueryResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1431852049, data2: 14817, data3: 20033, data4: [164, 225, 95, 72, 114, 190, 225, 153] };
}
#[repr(C)]
pub struct IOfflineMapPackageStartDownloadResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOfflineMapPackageStartDownloadResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3647322392, data2: 54486, data3: 19198, data4: [147, 120, 62, 199, 30, 241, 28, 61] };
}
#[repr(C)]
pub struct IOfflineMapPackageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, querypoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInBoundingBoxAsync: unsafe extern "system" fn(this: *mut *mut Self, queryboundingbox: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInBoundingBoxAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInGeocircleAsync: unsafe extern "system" fn(this: *mut *mut Self, querycircle: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInGeocircleAsync: usize,
}
impl ::windows_sys::core::Interface for IOfflineMapPackageStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 408844578, data2: 43057, data3: 19120, data4: [148, 31, 105, 152, 250, 146, 146, 133] };
}
pub type OfflineMapPackage = *mut ::core::ffi::c_void;
pub type OfflineMapPackageQueryResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageQueryStatus {}
impl ::core::clone::Clone for OfflineMapPackageQueryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OfflineMapPackageStartDownloadResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const DeniedWithoutCapability: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStartDownloadStatus {}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Deleting: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStatus {}
impl ::core::clone::Clone for OfflineMapPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
