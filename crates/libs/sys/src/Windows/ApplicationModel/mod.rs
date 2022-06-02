#[cfg(feature = "ApplicationModel_Activation")]
pub mod Activation;
#[cfg(feature = "ApplicationModel_AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "ApplicationModel_AppService")]
pub mod AppService;
#[cfg(feature = "ApplicationModel_Appointments")]
pub mod Appointments;
#[cfg(feature = "ApplicationModel_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls")]
pub mod Calls;
#[cfg(feature = "ApplicationModel_Chat")]
pub mod Chat;
#[cfg(feature = "ApplicationModel_CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "ApplicationModel_Contacts")]
pub mod Contacts;
#[cfg(feature = "ApplicationModel_ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "ApplicationModel_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "ApplicationModel_Email")]
pub mod Email;
#[cfg(feature = "ApplicationModel_ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "ApplicationModel_Holographic")]
pub mod Holographic;
#[cfg(feature = "ApplicationModel_LockScreen")]
pub mod LockScreen;
#[cfg(feature = "ApplicationModel_Payments")]
pub mod Payments;
#[cfg(feature = "ApplicationModel_Preview")]
pub mod Preview;
#[cfg(feature = "ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "ApplicationModel_Search")]
pub mod Search;
#[cfg(feature = "ApplicationModel_SocialInfo")]
pub mod SocialInfo;
#[cfg(feature = "ApplicationModel_Store")]
pub mod Store;
#[cfg(feature = "ApplicationModel_UserActivities")]
pub mod UserActivities;
#[cfg(feature = "ApplicationModel_UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "ApplicationModel_UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "ApplicationModel_VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "ApplicationModel_Wallet")]
pub mod Wallet;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AddResourcePackageOptions(pub u32);
impl AddResourcePackageOptions {
    pub const None: Self = Self(0u32);
    pub const ForceTargetAppShutdown: Self = Self(1u32);
    pub const ApplyUpdateIfAvailable: Self = Self(2u32);
}
impl ::core::marker::Copy for AddResourcePackageOptions {}
impl ::core::clone::Clone for AddResourcePackageOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppDisplayInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppExecutionContext(pub i32);
impl AppExecutionContext {
    pub const Unknown: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Guest: Self = Self(2i32);
}
impl ::core::marker::Copy for AppExecutionContext {}
impl ::core::clone::Clone for AppExecutionContext {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppInfo = *mut ::core::ffi::c_void;
pub type AppInstallerInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct AppInstallerPolicySource(pub i32);
impl AppInstallerPolicySource {
    pub const Default: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl ::core::marker::Copy for AppInstallerPolicySource {}
impl ::core::clone::Clone for AppInstallerPolicySource {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppInstance = *mut ::core::ffi::c_void;
pub type EnteredBackgroundEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct FullTrustLaunchResult(pub i32);
impl FullTrustLaunchResult {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for FullTrustLaunchResult {}
impl ::core::clone::Clone for FullTrustLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FullTrustProcessLaunchResult = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppDisplayInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogo: unsafe extern "system" fn(this: *mut *mut Self, size: super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogo: usize,
}
#[repr(C)]
pub struct IAppInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExecutionContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppExecutionContext) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInfo4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SupportedFileExtensions: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFromAppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, appusermodelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFromAppUserModelIdForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, appusermodelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFromAppUserModelIdForUser: usize,
}
#[repr(C)]
pub struct IAppInstallerInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(C)]
pub struct IAppInstallerInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnLaunch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageVersion) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastChecked: usize,
    #[cfg(feature = "Foundation")]
    pub PausedUntil: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PausedUntil: usize,
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
    pub PolicySource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppInstallerPolicySource) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppInstanceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecommendedInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub GetActivatedEventArgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    GetActivatedEventArgs: usize,
    pub FindOrRegisterInstanceForKey: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInstances: usize,
}
#[repr(C)]
pub struct ICameraApplicationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesignModeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesignModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesignModeStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesignMode2Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnteredBackgroundEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IFullTrustProcessLaunchResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaunchResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FullTrustLaunchResult) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFullTrustProcessLauncherStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, parametergroupid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithParametersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppAsync: unsafe extern "system" fn(this: *mut *mut Self, fulltrustpackagerelativeappid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, fulltrustpackagerelativeappid: ::windows_sys::core::HSTRING, parametergroupid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithParametersAsync: usize,
}
#[repr(C)]
pub struct IFullTrustProcessLauncherStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut *mut Self, commandline: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullTrustProcessForAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut *mut Self, fulltrustpackagerelativeappid: ::windows_sys::core::HSTRING, commandline: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullTrustProcessForAppWithArgumentsAsync: usize,
}
#[repr(C)]
pub struct ILeavingBackgroundEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ILimitedAccessFeatureRequestResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub FeatureId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LimitedAccessFeatureStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedRemovalDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedRemovalDate: usize,
}
#[repr(C)]
pub struct ILimitedAccessFeaturesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryUnlockFeature: unsafe extern "system" fn(this: *mut *mut Self, featureid: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, attestation: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub InstalledLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstalledLocation: usize,
    pub IsFramework: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Dependencies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Dependencies: usize,
}
#[repr(C)]
pub struct IPackage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublisherDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    pub IsResourcePackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBundle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDevelopmentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackage3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstalledDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstalledDate: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntriesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntriesAsync: usize,
}
#[repr(C)]
pub struct IPackage4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignatureKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageSignatureKind) -> ::windows_sys::core::HRESULT,
    pub IsOptional: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VerifyContentIntegrityAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerifyContentIntegrityAsync: usize,
}
#[repr(C)]
pub struct IPackage5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContentGroupsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContentGroupsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContentGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContentGroupAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsAsync: unsafe extern "system" fn(this: *mut *mut Self, names: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsWithPriorityAsync: unsafe extern "system" fn(this: *mut *mut Self, names: *mut ::core::ffi::c_void, movetoheadofqueue: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsWithPriorityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetInUseAsync: unsafe extern "system" fn(this: *mut *mut Self, inuse: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInUseAsync: usize,
}
#[repr(C)]
pub struct IPackage6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAppInstallerInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckUpdateAvailabilityAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUpdateAvailabilityAsync: usize,
}
#[repr(C)]
pub struct IPackage7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub MutableLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MutableLocation: usize,
    #[cfg(feature = "Storage")]
    pub EffectiveLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveLocation: usize,
}
#[repr(C)]
pub struct IPackage8 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub EffectiveExternalLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub MachineExternalLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MachineExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub UserExternalLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    UserExternalLocation: usize,
    pub InstalledPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MutablePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EffectivePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EffectiveExternalPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MachineExternalPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserExternalPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetLogoAsRandomAccessStreamReference: unsafe extern "system" fn(this: *mut *mut Self, size: super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetLogoAsRandomAccessStreamReference: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntries: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntries: usize,
    pub IsStub: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageCatalog {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PackageStaging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStaging: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStaging: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalling: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalling: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStatusChanged: usize,
}
#[repr(C)]
pub struct IPackageCatalog2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PackageContentGroupStaging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageContentGroupStaging: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageContentGroupStaging: usize,
    #[cfg(feature = "Foundation")]
    pub AddOptionalPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, optionalpackagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddOptionalPackageAsync: usize,
}
#[repr(C)]
pub struct IPackageCatalog3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, optionalpackagefamilynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
#[repr(C)]
pub struct IPackageCatalog4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddResourcePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, resourcepackagefamilyname: ::windows_sys::core::HSTRING, resourceid: ::windows_sys::core::HSTRING, options: AddResourcePackageOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddResourcePackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveResourcePackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, resourcepackages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveResourcePackagesAsync: usize,
}
#[repr(C)]
pub struct IPackageCatalogAddOptionalPackageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageCatalogAddResourcePackageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageCatalogRemoveOptionalPackagesResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageCatalogRemoveResourcePackagesResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageCatalogStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenForCurrentPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageContentGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageContentGroupState) -> ::windows_sys::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageContentGroupStagingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub ContentGroupName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsContentGroupRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageContentGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequiredGroupName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageId {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageVersion) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub Architecture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::System::ProcessorArchitecture) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Architecture: usize,
    pub ResourceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublisherId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageIdWithMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageInstallingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageStagingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub VerifyIsOK: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub NotAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PackageOffline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DataOffline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Disabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub NeedsRemediation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub LicenseIssue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Modified: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Tampered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DependencyIssue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Servicing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DeploymentInProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageStatus2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPartiallyStaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageUninstallingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageUpdateAvailabilityResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Availability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageUpdateAvailability) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageUpdatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SourcePackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TargetPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPackageWithMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InstallDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallDate: usize,
    pub GetThumbnailToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Launch: unsafe extern "system" fn(this: *mut *mut Self, parameters: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Launch: usize,
}
#[repr(C)]
pub struct IStartupTask {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestEnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestEnableAsync: usize,
    pub Disable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StartupTaskState) -> ::windows_sys::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStartupTaskStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForCurrentPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForCurrentPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAsync: unsafe extern "system" fn(this: *mut *mut Self, taskid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAsync: usize,
}
#[repr(C)]
pub struct ISuspendingDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISuspendingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SuspendingOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISuspendingOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
pub type LeavingBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type LimitedAccessFeatureRequestResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct LimitedAccessFeatureStatus(pub i32);
impl LimitedAccessFeatureStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
    pub const AvailableWithoutToken: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LimitedAccessFeatureStatus {}
impl ::core::clone::Clone for LimitedAccessFeatureStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Package = *mut ::core::ffi::c_void;
pub type PackageCatalog = *mut ::core::ffi::c_void;
pub type PackageCatalogAddOptionalPackageResult = *mut ::core::ffi::c_void;
pub type PackageCatalogAddResourcePackageResult = *mut ::core::ffi::c_void;
pub type PackageCatalogRemoveOptionalPackagesResult = *mut ::core::ffi::c_void;
pub type PackageCatalogRemoveResourcePackagesResult = *mut ::core::ffi::c_void;
pub type PackageContentGroup = *mut ::core::ffi::c_void;
pub type PackageContentGroupStagingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageContentGroupState(pub i32);
impl PackageContentGroupState {
    pub const NotStaged: Self = Self(0i32);
    pub const Queued: Self = Self(1i32);
    pub const Staging: Self = Self(2i32);
    pub const Staged: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageContentGroupState {}
impl ::core::clone::Clone for PackageContentGroupState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageId = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct PackageInstallProgress {
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for PackageInstallProgress {}
impl ::core::clone::Clone for PackageInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageInstallingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageSignatureKind(pub i32);
impl PackageSignatureKind {
    pub const None: Self = Self(0i32);
    pub const Developer: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
    pub const Store: Self = Self(3i32);
    pub const System: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageSignatureKind {}
impl ::core::clone::Clone for PackageSignatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageStagingEventArgs = *mut ::core::ffi::c_void;
pub type PackageStatus = *mut ::core::ffi::c_void;
pub type PackageStatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type PackageUninstallingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct PackageUpdateAvailability(pub i32);
impl PackageUpdateAvailability {
    pub const Unknown: Self = Self(0i32);
    pub const NoUpdates: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Required: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageUpdateAvailability {}
impl ::core::clone::Clone for PackageUpdateAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageUpdateAvailabilityResult = *mut ::core::ffi::c_void;
pub type PackageUpdatingEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"ApplicationModel\"`*"]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for PackageVersion {}
impl ::core::clone::Clone for PackageVersion {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StartupTask = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel\"`*"]
#[repr(transparent)]
pub struct StartupTaskState(pub i32);
impl StartupTaskState {
    pub const Disabled: Self = Self(0i32);
    pub const DisabledByUser: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const EnabledByPolicy: Self = Self(4i32);
}
impl ::core::marker::Copy for StartupTaskState {}
impl ::core::clone::Clone for StartupTaskState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SuspendingDeferral = *mut ::core::ffi::c_void;
pub type SuspendingEventArgs = *mut ::core::ffi::c_void;
pub type SuspendingOperation = *mut ::core::ffi::c_void;
