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
impl ::windows_sys::core::Interface for IAddPackageOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 97443864, data2: 63119, data3: 16939, data4: [149, 164, 102, 103, 158, 199, 127, 192] };
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
impl ::windows_sys::core::Interface for IAppInstallerManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3891143107, data2: 8451, data3: 21486, data4: [155, 24, 104, 175, 234, 176, 3, 61] };
}
#[repr(C)]
pub struct IAppInstallerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstallerManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3378147029, data2: 64601, data3: 21302, data4: [155, 46, 43, 7, 197, 230, 20, 52] };
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
impl ::windows_sys::core::Interface for IAutoUpdateSettingsOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1732844935, data2: 13793, data3: 20778, data4: [137, 104, 26, 232, 141, 27, 230, 211] };
}
#[repr(C)]
pub struct IAutoUpdateSettingsOptionsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub CreateFromAppInstallerInfo: unsafe extern "system" fn(this: *mut *mut Self, appinstallerinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    CreateFromAppInstallerInfo: usize,
}
impl ::windows_sys::core::Interface for IAutoUpdateSettingsOptionsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2289775485, data2: 3077, data3: 21712, data4: [189, 73, 59, 183, 162, 192, 132, 203] };
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
impl ::windows_sys::core::Interface for ICreateSharedPackageContainerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3266014926, data2: 63076, data3: 23694, data4: [164, 179, 42, 51, 39, 109, 61, 222] };
}
#[repr(C)]
pub struct ICreateSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateSharedPackageContainerResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3465023679, data2: 5404, data3: 22279, data4: [185, 54, 73, 126, 86, 74, 252, 122] };
}
#[repr(C)]
pub struct IDeleteSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AllUsers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllUsers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeleteSharedPackageContainerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2642511455, data2: 39022, data3: 20792, data4: [139, 93, 56, 77, 142, 102, 237, 108] };
}
#[repr(C)]
pub struct IDeleteSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeleteSharedPackageContainerResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 892962948, data2: 22326, data3: 20859, data4: [133, 188, 229, 152, 200, 26, 178, 132] };
}
#[repr(C)]
pub struct IDeploymentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeploymentResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627292590, data2: 46973, data3: 19487, data4: [138, 123, 32, 230, 173, 81, 94, 243] };
}
#[repr(C)]
pub struct IDeploymentResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRegistered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeploymentResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4228804956, data2: 23041, data3: 19415, data4: [188, 241, 56, 28, 140, 130, 224, 74] };
}
#[repr(C)]
pub struct IFindSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFindSharedPackageContainerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3020933374, data2: 33668, data3: 21708, data4: [129, 125, 174, 9, 211, 182, 166, 6] };
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
impl ::windows_sys::core::Interface for IPackageAllUserProvisioningOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3660950050, data2: 7648, data3: 23870, data4: [153, 255, 210, 79, 49, 24, 191, 94] };
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
impl ::windows_sys::core::Interface for IPackageManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2591902565, data2: 24207, data3: 20423, data4: [162, 229, 127, 105, 37, 203, 139, 83] };
}
#[repr(C)]
pub struct IPackageManager10 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProvisionPackageForAllUsersWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, mainpackagefamilyname: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionPackageForAllUsersWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IPackageManager10 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2815938686, data2: 11878, data3: 16531, data4: [174, 213, 224, 147, 237, 135, 179, 187] };
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
impl ::windows_sys::core::Interface for IPackageManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4155166861, data2: 2112, data3: 18162, data4: [181, 216, 202, 212, 118, 147, 160, 149] };
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
impl ::windows_sys::core::Interface for IPackageManager3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3668810056, data2: 14065, data3: 16807, data4: [145, 136, 188, 38, 62, 13, 203, 114] };
}
#[repr(C)]
pub struct IPackageManager4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPackageVolumesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPackageVolumesAsync: usize,
}
impl ::windows_sys::core::Interface for IPackageManager4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1014077795, data2: 47798, data3: 18111, data4: [143, 247, 218, 71, 25, 35, 10, 230] };
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
impl ::windows_sys::core::Interface for IPackageManager5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1897869591, data2: 6909, data3: 17171, data4: [151, 140, 155, 182, 225, 184, 100, 167] };
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
impl ::windows_sys::core::Interface for IPackageManager6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 138930441, data2: 21453, data3: 20047, data4: [131, 46, 87, 209, 128, 246, 228, 71] };
}
#[repr(C)]
pub struct IPackageManager7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAddPackageAndRelatedSetAsync: unsafe extern "system" fn(this: *mut *mut Self, packageuri: *mut ::core::ffi::c_void, dependencypackageuris: *mut ::core::ffi::c_void, deploymentoptions: DeploymentOptions, targetvolume: *mut ::core::ffi::c_void, optionalpackagefamilynames: *mut ::core::ffi::c_void, relatedpackageuris: *mut ::core::ffi::c_void, packageuristoinstall: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAddPackageAndRelatedSetAsync: usize,
}
impl ::windows_sys::core::Interface for IPackageManager7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4068889844, data2: 11175, data3: 19328, data4: [136, 214, 190, 21, 249, 162, 63, 186] };
}
#[repr(C)]
pub struct IPackageManager8 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DeprovisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeprovisionPackageForAllUsersAsync: usize,
}
impl ::windows_sys::core::Interface for IPackageManager8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3092730672, data2: 4760, data3: 20194, data4: [128, 238, 127, 101, 156, 93, 39, 130] };
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
impl ::windows_sys::core::Interface for IPackageManager9 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 447189045, data2: 52337, data3: 19246, data4: [128, 166, 199, 4, 29, 133, 121, 167] };
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
impl ::windows_sys::core::Interface for IPackageManagerDebugSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 442570371, data2: 43400, data3: 20431, data4: [143, 15, 206, 23, 88, 152, 232, 235] };
}
#[repr(C)]
pub struct IPackageUserInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserSecurityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageInstallState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageUserInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4130878499, data2: 64009, data3: 19644, data4: [144, 85, 21, 202, 39, 94, 46, 126] };
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
impl ::windows_sys::core::Interface for IPackageVolume {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3475403459, data2: 6720, data3: 17488, data4: [151, 57, 42, 206, 46, 137, 136, 83] };
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
impl ::windows_sys::core::Interface for IPackageVolume2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1185664814, data2: 40404, data3: 18338, data4: [171, 140, 198, 64, 131, 73, 188, 216] };
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
impl ::windows_sys::core::Interface for IRegisterPackageOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1735463591, data2: 20692, data3: 18796, data4: [132, 21, 6, 2, 180, 198, 211, 191] };
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
impl ::windows_sys::core::Interface for ISharedPackageContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 394205865, data2: 5406, data3: 24311, data4: [177, 217, 47, 186, 11, 75, 13, 23] };
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
impl ::windows_sys::core::Interface for ISharedPackageContainerManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3191156840, data2: 7927, data3: 23240, data4: [171, 63, 11, 159, 97, 47, 2, 116] };
}
#[repr(C)]
pub struct ISharedPackageContainerManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, usersid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForProvisioning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedPackageContainerManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 787833672, data2: 33674, data3: 24405, data4: [168, 158, 17, 152, 162, 198, 39, 230] };
}
#[repr(C)]
pub struct ISharedPackageContainerMember {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedPackageContainerMember {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4262265912, data2: 17353, data3: 21542, data4: [184, 156, 247, 155, 248, 93, 223, 244] };
}
#[repr(C)]
pub struct ISharedPackageContainerMemberFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedPackageContainerMemberFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1236324075, data2: 18831, data3: 23138, data4: [183, 56, 179, 202, 13, 67, 103, 4] };
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
impl ::windows_sys::core::Interface for IStagePackageOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 185666716, data2: 47453, data3: 19542, data4: [189, 54, 109, 101, 104, 0, 208, 107] };
}
#[repr(C)]
pub struct IUpdateSharedPackageContainerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RequirePackagesPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequirePackagesPresent: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUpdateSharedPackageContainerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2154245763, data2: 29076, data3: 23033, data4: [181, 185, 218, 165, 55, 95, 19, 10] };
}
#[repr(C)]
pub struct IUpdateSharedPackageContainerResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUpdateSharedPackageContainerResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2856353271, data2: 50989, data3: 21592, data4: [174, 163, 70, 69, 182, 168, 238, 153] };
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
