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
#[repr(C)]
pub struct IEnterpriseEnrollmentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnrolledEnterprise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EnterpriseEnrollmentStatus) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPackageInstallResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub InstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Management::Deployment::PackageInstallState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    InstallState: usize,
}
#[repr(C)]
pub struct IPackageInstallResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type PackageInstallResult = *mut ::core::ffi::c_void;
