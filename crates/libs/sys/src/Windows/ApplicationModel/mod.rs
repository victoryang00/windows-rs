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
impl ::windows_sys::core::Interface for IAppDisplayInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 451612931, data2: 58580, data3: 16810, data4: [164, 246, 196, 162, 118, 231, 158, 172] };
}
#[repr(C)]
pub struct IAppInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3481229747, data2: 27145, data3: 19944, data4: [166, 192, 87, 146, 213, 104, 128, 209] };
}
#[repr(C)]
pub struct IAppInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3192594266, data2: 8344, data3: 17179, data4: [189, 37, 179, 8, 120, 116, 141, 71] };
}
#[repr(C)]
pub struct IAppInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExecutionContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppExecutionContext) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInfo3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 161975878, data2: 37796, data3: 18142, data4: [147, 151, 8, 67, 181, 113, 21, 234] };
}
#[repr(C)]
pub struct IAppInfo4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SupportedFileExtensions: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInfo4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 791985643, data2: 5641, data3: 17748, data4: [159, 51, 18, 225, 232, 3, 224, 212] };
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
impl ::windows_sys::core::Interface for IAppInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474946090, data2: 58507, data3: 20236, data4: [155, 11, 121, 195, 248, 149, 125, 215] };
}
#[repr(C)]
pub struct IAppInstallerInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
impl ::windows_sys::core::Interface for IAppInstallerInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 699083456, data2: 54518, data3: 17059, data4: [173, 205, 214, 88, 60, 101, 149, 8] };
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
impl ::windows_sys::core::Interface for IAppInstallerInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3524203400, data2: 33366, data3: 22908, data4: [133, 17, 200, 78, 197, 13, 94, 43] };
}
#[repr(C)]
pub struct IAppInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppInstance {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1734290247, data2: 62047, data3: 17714, data4: [159, 214, 54, 51, 224, 99, 77, 1] };
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
impl ::windows_sys::core::Interface for IAppInstanceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2635196287, data2: 40614, data3: 18351, data4: [166, 236, 70, 120, 76, 91, 162, 84] };
}
#[repr(C)]
pub struct ICameraApplicationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraApplicationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2509888974, data2: 39891, data3: 17244, data4: [128, 84, 193, 173, 213, 0, 40, 254] };
}
#[repr(C)]
pub struct IDesignModeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesignModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDesignModeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 741905356, data2: 63514, data3: 20090, data4: [184, 87, 118, 168, 8, 135, 225, 133] };
}
#[repr(C)]
pub struct IDesignModeStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesignMode2Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDesignModeStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2161082679, data2: 45156, data3: 18520, data4: [190, 200, 62, 186, 34, 53, 117, 53] };
}
#[repr(C)]
pub struct IEnteredBackgroundEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IEnteredBackgroundEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146257090, data2: 38951, data3: 16445, data4: [170, 237, 236, 202, 154, 193, 115, 152] };
}
#[repr(C)]
pub struct IFullTrustProcessLaunchResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LaunchResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FullTrustLaunchResult) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFullTrustProcessLaunchResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2300041352, data2: 60923, data3: 20831, data4: [142, 34, 94, 188, 235, 105, 223, 217] };
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
impl ::windows_sys::core::Interface for IFullTrustProcessLauncherStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3615785855, data2: 4352, data3: 15467, data4: [164, 85, 246, 38, 44, 195, 49, 182] };
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
impl ::windows_sys::core::Interface for IFullTrustProcessLauncherStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2341394223, data2: 46684, data3: 22223, data4: [161, 167, 43, 247, 124, 188, 110, 168] };
}
#[repr(C)]
pub struct ILeavingBackgroundEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for ILeavingBackgroundEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 969338010, data2: 44654, data3: 18169, data4: [160, 122, 207, 194, 63, 136, 115, 62] };
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
impl ::windows_sys::core::Interface for ILimitedAccessFeatureRequestResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3562100390, data2: 7716, data3: 24029, data4: [171, 180, 97, 136, 171, 164, 213, 191] };
}
#[repr(C)]
pub struct ILimitedAccessFeaturesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryUnlockFeature: unsafe extern "system" fn(this: *mut *mut Self, featureid: ::windows_sys::core::HSTRING, token: ::windows_sys::core::HSTRING, attestation: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILimitedAccessFeaturesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2347111124, data2: 12331, data3: 24511, data4: [166, 50, 26, 153, 228, 62, 137, 37] };
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
impl ::windows_sys::core::Interface for IPackage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 373061935, data2: 48501, data3: 16700, data4: [191, 35, 177, 254, 123, 149, 216, 37] };
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
impl ::windows_sys::core::Interface for IPackage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2791387062, data2: 30344, data3: 19150, data4: [149, 251, 53, 149, 56, 231, 170, 1] };
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
impl ::windows_sys::core::Interface for IPackage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1601407841, data2: 63594, data3: 18711, data4: [147, 209, 241, 238, 157, 59, 53, 217] };
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
impl ::windows_sys::core::Interface for IPackage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1705955758, data2: 47451, data3: 17676, data4: [136, 43, 98, 85, 24, 127, 57, 126] };
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
impl ::windows_sys::core::Interface for IPackage5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 243543508, data2: 55724, data3: 17901, data4: [154, 30, 116, 206, 5, 107, 38, 53] };
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
impl ::windows_sys::core::Interface for IPackage6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2333792578, data2: 4823, data3: 18260, data4: [174, 78, 99, 140, 188, 14, 58, 46] };
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
impl ::windows_sys::core::Interface for IPackage7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2264894769, data2: 41700, data3: 17888, data4: [151, 50, 40, 58, 109, 136, 253, 225] };
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
impl ::windows_sys::core::Interface for IPackage8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 743985019, data2: 52778, data3: 19430, data4: [160, 147, 119, 207, 187, 42, 126, 161] };
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
impl ::windows_sys::core::Interface for IPackageCatalog {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587872081, data2: 40419, data3: 17477, data4: [190, 116, 145, 251, 50, 90, 190, 254] };
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
impl ::windows_sys::core::Interface for IPackageCatalog2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2527464502, data2: 36855, data3: 17220, data4: [182, 191, 238, 100, 194, 32, 126, 210] };
}
#[repr(C)]
pub struct IPackageCatalog3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, optionalpackagefamilynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
impl ::windows_sys::core::Interface for IPackageCatalog3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2531089544, data2: 34871, data3: 17401, data4: [144, 21, 3, 52, 52, 186, 20, 243] };
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
impl ::windows_sys::core::Interface for IPackageCatalog4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3279698331, data2: 17612, data3: 19323, data4: [139, 175, 121, 108, 4, 234, 211, 185] };
}
#[repr(C)]
pub struct IPackageCatalogAddOptionalPackageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageCatalogAddOptionalPackageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1005653204, data2: 46303, data3: 18355, data4: [169, 99, 226, 250, 131, 47, 125, 211] };
}
#[repr(C)]
pub struct IPackageCatalogAddResourcePackageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageCatalogAddResourcePackageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2520174093, data2: 15895, data3: 18751, data4: [170, 8, 204, 236, 111, 222, 246, 153] };
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
impl ::windows_sys::core::Interface for IPackageCatalogRemoveOptionalPackagesResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 701692283, data2: 55668, data3: 20068, data4: [147, 89, 34, 202, 223, 215, 152, 40] };
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
impl ::windows_sys::core::Interface for IPackageCatalogRemoveResourcePackagesResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2926679817, data2: 6738, data3: 17185, data4: [135, 179, 229, 161, 161, 121, 129, 167] };
}
#[repr(C)]
pub struct IPackageCatalogStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenForCurrentPackage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageCatalogStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2710345366, data2: 58971, data3: 17972, data4: [186, 33, 94, 99, 235, 114, 68, 167] };
}
#[repr(C)]
pub struct IPackageContentGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageContentGroupState) -> ::windows_sys::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageContentGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2405591389, data2: 4618, data3: 18328, data4: [181, 225, 88, 0, 221, 168, 242, 225] };
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
impl ::windows_sys::core::Interface for IPackageContentGroupStagingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1031520894, data2: 28455, data3: 17516, data4: [152, 110, 212, 115, 61, 77, 145, 19] };
}
#[repr(C)]
pub struct IPackageContentGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequiredGroupName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageContentGroupStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1894675993, data2: 24338, data3: 19346, data4: [185, 234, 108, 202, 218, 19, 188, 117] };
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
impl ::windows_sys::core::Interface for IPackageId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 450586206, data2: 14279, data3: 18320, data4: [153, 128, 221, 122, 231, 78, 139, 178] };
}
#[repr(C)]
pub struct IPackageIdWithMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageIdWithMetadata {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1079474812, data2: 3230, data3: 17469, data4: [144, 116, 133, 95, 92, 224, 160, 141] };
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
impl ::windows_sys::core::Interface for IPackageInstallingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2540969655, data2: 43898, data3: 16410, data4: [139, 97, 235, 14, 127, 175, 242, 55] };
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
impl ::windows_sys::core::Interface for IPackageStagingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 272721965, data2: 21730, data3: 20305, data4: [184, 40, 158, 247, 4, 108, 33, 15] };
}
#[repr(C)]
pub struct IPackageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1314081759, data2: 10592, data3: 18552, data4: [151, 164, 150, 36, 222, 183, 47, 45] };
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
impl ::windows_sys::core::Interface for IPackageStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1608994673, data2: 41829, data3: 19465, data4: [160, 45, 4, 109, 82, 94, 161, 218] };
}
#[repr(C)]
pub struct IPackageStatus2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPartiallyStaged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageStatus2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4096326291, data2: 31830, data3: 18530, data4: [172, 250, 171, 174, 220, 192, 105, 77] };
}
#[repr(C)]
pub struct IPackageStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1132294477, data2: 48512, data3: 19056, data4: [188, 80, 246, 231, 150, 80, 149, 117] };
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
impl ::windows_sys::core::Interface for IPackageUninstallingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1145285202, data2: 43810, data3: 17613, data4: [130, 187, 78, 201, 184, 39, 54, 122] };
}
#[repr(C)]
pub struct IPackageUpdateAvailabilityResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Availability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PackageUpdateAvailability) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPackageUpdateAvailabilityResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 290344969, data2: 6554, data3: 18593, data4: [160, 121, 49, 60, 69, 99, 74, 113] };
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
impl ::windows_sys::core::Interface for IPackageUpdatingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3447407144, data2: 64884, data3: 17470, data4: [177, 20, 35, 230, 119, 176, 232, 111] };
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
impl ::windows_sys::core::Interface for IPackageWithMetadata {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2509543296, data2: 7657, data3: 16626, data4: [180, 82, 13, 233, 241, 145, 0, 18] };
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
impl ::windows_sys::core::Interface for IStartupTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4150010824, data2: 46578, data3: 20332, data4: [136, 221, 54, 203, 29, 89, 157, 23] };
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
impl ::windows_sys::core::Interface for IStartupTaskStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3998965949, data2: 41288, data3: 16807, data4: [178, 110, 232, 184, 138, 30, 98, 248] };
}
#[repr(C)]
pub struct ISuspendingDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISuspendingDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1494484233, data2: 35785, data3: 20148, data4: [182, 54, 218, 189, 196, 244, 111, 102] };
}
#[repr(C)]
pub struct ISuspendingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SuspendingOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISuspendingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2516982789, data2: 11706, data3: 19720, data4: [176, 189, 43, 48, 161, 49, 198, 170] };
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
impl ::windows_sys::core::Interface for ISuspendingOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2644822593, data2: 8417, data3: 20123, data4: [159, 101, 169, 244, 53, 52, 12, 58] };
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
