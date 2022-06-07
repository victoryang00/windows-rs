pub type Enterprise = *mut ::core::ffi::c_void;
pub type EnterpriseEnrollmentResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: Self = Self(0i32);
    pub const CancelledByUser: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for EnterpriseEnrollmentStatus {}
impl ::core::clone::Clone for EnterpriseEnrollmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_Management_Deployment\"`*"]
#[repr(transparent)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
}
impl ::core::marker::Copy for EnterpriseStatus {}
impl ::core::clone::Clone for EnterpriseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IEnterprise {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WorkplaceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidFrom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidFrom: usize,
    #[cfg(feature = "Foundation")]
    pub EnrollmentValidTo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnrollmentValidTo: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EnterpriseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnterprise {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2522427277, data2: 34156, data3: 17446, data4: [169, 71, 176, 99, 7, 113, 128, 120] };
}
#[repr(C)]
pub struct IEnterpriseEnrollmentManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EnrolledEnterprises: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnrolledEnterprises: usize,
    pub CurrentEnterprise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidateEnterprisesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateEnterprisesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestEnrollmentAsync: unsafe extern "system" fn(this: *mut *mut Self, enrollmenttoken: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnrollmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnenrollmentAsync: unsafe extern "system" fn(this: *mut *mut Self, enterprise: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnenrollmentAsync: usize,
}
impl ::windows_sys::core::Interface for IEnterpriseEnrollmentManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 553251728, data2: 11369, data3: 16856, data4: [136, 230, 228, 179, 136, 64, 38, 203] };
}
#[repr(C)]
pub struct IEnterpriseEnrollmentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EnterpriseEnrollmentStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnterpriseEnrollmentResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2683772134, data2: 37083, data3: 17218, data4: [179, 38, 23, 41, 170, 145, 48, 28] };
}
#[repr(C)]
pub struct IInstallationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, sourcelocation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackagePreloadedAsync: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, sourcelocation: *mut ::core::ffi::c_void, instanceid: ::windows_sys::core::HSTRING, offerid: ::windows_sys::core::HSTRING, license: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackagePreloadedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingPackageInstalls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingPackageInstalls: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesForCurrentPublisher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesForCurrentPublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
}
impl ::windows_sys::core::Interface for IInstallationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2459608888, data2: 36169, data3: 17068, data4: [128, 201, 180, 173, 121, 60, 67, 242] };
}
#[repr(C)]
pub struct IInstallationManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Management_Deployment"))]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, removaloptions: super::super::super::Management::Deployment::RemovalOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Management_Deployment")))]
    RemovePackageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: super::super::super::Management::Deployment::DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
}
impl ::windows_sys::core::Interface for IInstallationManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2087464125, data2: 64074, data3: 19598, data4: [171, 151, 217, 89, 69, 47, 25, 229] };
}
#[repr(C)]
pub struct IPackageInstallResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
impl ::windows_sys::core::Interface for IPackageInstallResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 870903509, data2: 3966, data3: 17523, data4: [150, 124, 125, 110, 28, 14, 125, 225] };
}
#[repr(C)]
pub struct IPackageInstallResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageInstallResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1900665097, data2: 16377, data3: 16877, data4: [167, 23, 43, 198, 95, 252, 97, 210] };
}
pub type PackageInstallResult = *mut ::core::ffi::c_void;
