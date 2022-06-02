#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: Self = Self(0u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetAppShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const LimitToExistingPackages: Self = Self(512u32);
}
impl ::core::marker::Copy for AddPackageByAppInstallerOptions {}
impl ::core::clone::Clone for AddPackageByAppInstallerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AddPackageOptions = *mut ::core::ffi::c_void;
pub type AppInstallerManager = *mut ::core::ffi::c_void;
pub type AutoUpdateSettingsOptions = *mut ::core::ffi::c_void;
pub type CreateSharedPackageContainerOptions = *mut ::core::ffi::c_void;
pub type CreateSharedPackageContainerResult = *mut ::core::ffi::c_void;
pub type DeleteSharedPackageContainerOptions = *mut ::core::ffi::c_void;
pub type DeleteSharedPackageContainerResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: Self = Self(0u32);
    pub const ForceApplicationShutdown: Self = Self(1u32);
    pub const DevelopmentMode: Self = Self(2u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetApplicationShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const ForceUpdateFromAnyVersion: Self = Self(262144u32);
    pub const RetainFilesOnFailure: Self = Self(2097152u32);
    pub const StageInPlace: Self = Self(4194304u32);
}
impl ::core::marker::Copy for DeploymentOptions {}
impl ::core::clone::Clone for DeploymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Management_Deployment\"`*"]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl ::core::marker::Copy for DeploymentProgress {}
impl ::core::clone::Clone for DeploymentProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
}
impl ::core::marker::Copy for DeploymentProgressState {}
impl ::core::clone::Clone for DeploymentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeploymentResult = *mut ::core::ffi::c_void;
pub type FindSharedPackageContainerOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAddPackageOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedPackageUris: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StubPackageOption) -> ::windows_sys::core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut *mut Self, value: StubPackageOption) -> ::windows_sys::core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RetainFilesOnFailure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRetainFilesOnFailure: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstallerManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAutoUpdateSettings: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, appinstallerinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearAutoUpdateSettings: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAutoUpdatesUntil: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, datetime: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAutoUpdatesUntil: usize,
}
#[repr(C)]
pub struct IAppInstallerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutoUpdateSettingsOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Version: usize,
    #[cfg(feature = "ApplicationModel")]
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::ApplicationModel::PackageVersion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SetVersion: usize,
    #[cfg(feature = "Foundation")]
    pub AppInstallerUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppInstallerUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppInstallerUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppInstallerUri: usize,
    pub OnLaunch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetOnLaunch: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowPrompt: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUpdateBlocksActivation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutomaticBackgroundTask: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAutoRepairEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RepairUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RepairUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
}
#[repr(C)]
pub struct IAutoUpdateSettingsOptionsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub CreateFromAppInstallerInfo: unsafe extern "system" fn(this: *mut *mut Self, appinstallerinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    CreateFromAppInstallerInfo: usize,
}
#[repr(C)]
pub struct ICreateSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Members: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Members: usize,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CreateCollisionOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows_sys::core::HRESULT,
    pub SetCreateCollisionOption: unsafe extern "system" fn(this: *mut *mut Self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeleteSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllUsers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllUsers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeleteSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeploymentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDeploymentResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRegistered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFindSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageAllUserProvisioningOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProjectionOrderPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProjectionOrderPackageFamilyNames: usize,
}
#[repr(C)]
pub struct IPackageManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdatePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdatePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUsers: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUsers: usize,
    pub SetPackageState: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, packagestate: PackageState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByPackageFullName: usize,
    #[cfg(feature = "Foundation")]
    pub CleanupPackageForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, usersecurityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CleanupPackageForUserAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[repr(C)]
pub struct IPackageManager10 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProvisionPackageForAllUsersWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, mainpackagefamilyname: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionPackageForAllUsersWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IPackageManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RemovePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, removaloptions: RemovalOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageByFullNameAsync: unsafe extern "system" fn(this: *mut *mut Self, mainpackagefullname: ::windows_sys::core::HSTRING, dependencypackagefullnames: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageByFullNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagefamilyname: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: usize,
    #[cfg(feature = "Foundation")]
    pub StageUserDataAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StageUserDataAsync: usize,
}
#[repr(C)]
pub struct IPackageManager3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddPackageVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, packagestorepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageVolumeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAsync: usize,
    pub ClearPackageStatus: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, status: PackageStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageWithAppDataVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, manifesturi: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, appdatavolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageWithAppDataVolumeAsync: usize,
    pub FindPackageVolumeByName: unsafe extern "system" fn(this: *mut *mut Self, volumename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindPackageVolumes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindPackageVolumes: usize,
    pub GetDefaultPackageVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MovePackageToVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MovePackageToVolumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, volume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageVolumeAsync: usize,
    pub SetDefaultPackageVolume: unsafe extern "system" fn(this: *mut *mut Self, volume: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPackageStatus: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, status: PackageStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPackageVolumeOfflineAsync: unsafe extern "system" fn(this: *mut *mut Self, packagevolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPackageVolumeOfflineAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPackageVolumeOnlineAsync: unsafe extern "system" fn(this: *mut *mut Self, packagevolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPackageVolumeOnlineAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StageUserDataWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, deploymentoptions: DeploymentOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StageUserDataWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IPackageManager4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPackageVolumesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPackageVolumesAsync: usize,
}
#[repr(C)]
pub struct IPackageManager5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, externalpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, externalpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackageByFamilyNameAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, mainpackagefamilyname: ::windows_sys::core::HSTRING, dependencypackagefamilynames: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, appdatavolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackageByFamilyNameAndOptionalPackagesAsync: usize,
    pub DebugSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageManager6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProvisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionPackageForAllUsersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut *mut Self, appinstallerfileuri: *mut ::core::ffi::c_void, options: AddPackageByAppInstallerOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageByAppInstallerFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut *mut Self, appinstallerfileuri: *mut ::core::ffi::c_void, options: AddPackageByAppInstallerOptions, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddPackageByAppInstallerFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AddPackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, options: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddPackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StagePackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, options: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StagePackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAddPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAddPackageAsync: usize,
}
#[repr(C)]
pub struct IPackageManager7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAddPackageAndRelatedSetAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAddPackageAndRelatedSetAsync: usize,
}
#[repr(C)]
pub struct IPackageManager8 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DeprovisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeprovisionPackageForAllUsersAsync: usize,
}
#[repr(C)]
pub struct IPackageManager9 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindProvisionedPackages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindProvisionedPackages: usize,
    #[cfg(feature = "Foundation")]
    pub AddPackageByUriAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPackageByUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StagePackageByUriAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StagePackageByUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RegisterPackageByUriAsync: unsafe extern "system" fn(this: *mut *mut Self, manifesturi: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterPackageByUriAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterPackagesByFullNameAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefullnames: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterPackagesByFullNameAsync: usize,
    pub SetPackageStubPreference: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, usestub: PackageStubPreference) -> ::windows_sys::core::HRESULT,
    pub GetPackageStubPreference: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut PackageStubPreference) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageManagerDebugSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub SetContentGroupStateAsync: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, contentgroupname: ::windows_sys::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    SetContentGroupStateAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub SetContentGroupStateWithPercentageAsync: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, contentgroupname: ::windows_sys::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    SetContentGroupStateWithPercentageAsync: usize,
}
#[repr(C)]
pub struct IPackageUserInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserSecurityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageInstallState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageVolume {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOffline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSystemVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MountPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageStorePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportsHardLinks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackages: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut *mut Self, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByNamePublisherWithPackagesTypes: unsafe extern "system" fn(this: *mut *mut Self, packagetypes: PackageTypes, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByNamePublisherWithPackagesTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, packagetypes: PackageTypes, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackageByPackageFullName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, packagename: ::windows_sys::core::HSTRING, packagepublisher: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagetypes: PackageTypes, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut *mut Self, usersecurityid: ::windows_sys::core::HSTRING, packagefullname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[repr(C)]
pub struct IPackageVolume2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFullTrustPackageSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAppxInstallSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAvailableSpaceAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAvailableSpaceAsync: usize,
}
#[repr(C)]
pub struct IRegisterPackageOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub AppDataVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAppDataVolume: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISharedPackageContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMembers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMembers: usize,
    pub RemovePackageFamily: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ResetData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISharedPackageContainerManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateContainer: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContainers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContainers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContainersWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContainersWithOptions: usize,
}
#[repr(C)]
pub struct ISharedPackageContainerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, usersid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForProvisioning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISharedPackageContainerMember {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISharedPackageContainerMemberFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStagePackageOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedPackageUris: usize,
    #[cfg(feature = "Foundation")]
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExternalLocationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExternalLocationUri: usize,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StubPackageOption) -> ::windows_sys::core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut *mut Self, value: StubPackageOption) -> ::windows_sys::core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUpdateSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RequirePackagesPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequirePackagesPresent: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUpdateSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
pub type PackageAllUserProvisioningOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: Self = Self(0i32);
    pub const Staged: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for PackageInstallState {}
impl ::core::clone::Clone for PackageInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageManager = *mut ::core::ffi::c_void;
pub type PackageManagerDebugSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: Self = Self(0i32);
    pub const LicenseInvalid: Self = Self(1i32);
    pub const Modified: Self = Self(2i32);
    pub const Tampered: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageState {}
impl ::core::clone::Clone for PackageState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: Self = Self(0u32);
    pub const LicenseIssue: Self = Self(1u32);
    pub const Modified: Self = Self(2u32);
    pub const Tampered: Self = Self(4u32);
    pub const Disabled: Self = Self(8u32);
}
impl ::core::marker::Copy for PackageStatus {}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: Self = Self(0i32);
    pub const Stub: Self = Self(1i32);
}
impl ::core::marker::Copy for PackageStubPreference {}
impl ::core::clone::Clone for PackageStubPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: Self = Self(0u32);
    pub const Main: Self = Self(1u32);
    pub const Framework: Self = Self(2u32);
    pub const Resource: Self = Self(4u32);
    pub const Bundle: Self = Self(8u32);
    pub const Xap: Self = Self(16u32);
    pub const Optional: Self = Self(32u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PackageTypes {}
impl ::core::clone::Clone for PackageTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageUserInformation = *mut ::core::ffi::c_void;
pub type PackageVolume = *mut ::core::ffi::c_void;
pub type RegisterPackageOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: Self = Self(0u32);
    pub const PreserveApplicationData: Self = Self(4096u32);
    pub const PreserveRoamableApplicationData: Self = Self(128u32);
    pub const RemoveForAllUsers: Self = Self(524288u32);
}
impl ::core::marker::Copy for RemovalOptions {}
impl ::core::clone::Clone for RemovalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SharedPackageContainer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: Self = Self(0i32);
    pub const MergeWithExisting: Self = Self(1i32);
    pub const ReplaceExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for SharedPackageContainerCreationCollisionOptions {}
impl ::core::clone::Clone for SharedPackageContainerCreationCollisionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SharedPackageContainerManager = *mut ::core::ffi::c_void;
pub type SharedPackageContainerMember = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const BlockedByPolicy: Self = Self(1i32);
    pub const AlreadyExists: Self = Self(2i32);
    pub const PackageFamilyExistsInAnotherContainer: Self = Self(3i32);
    pub const NotFound: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for SharedPackageContainerOperationStatus {}
impl ::core::clone::Clone for SharedPackageContainerOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StagePackageOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Management_Deployment\"`*"]
#[repr(transparent)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: Self = Self(0i32);
    pub const InstallFull: Self = Self(1i32);
    pub const InstallStub: Self = Self(2i32);
    pub const UsePreference: Self = Self(3i32);
}
impl ::core::marker::Copy for StubPackageOption {}
impl ::core::clone::Clone for StubPackageOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UpdateSharedPackageContainerOptions = *mut ::core::ffi::c_void;
pub type UpdateSharedPackageContainerResult = *mut ::core::ffi::c_void;
