
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Activation")]
pub mod Activation;
#[cfg(feature = "AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "AppService")]
pub mod AppService;
#[cfg(feature = "Appointments")]
pub mod Appointments;
#[cfg(feature = "Background")]
pub mod Background;
#[cfg(feature = "Calls")]
pub mod Calls;
#[cfg(feature = "Chat")]
pub mod Chat;
#[cfg(feature = "CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "Contacts")]
pub mod Contacts;
#[cfg(feature = "ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "Email")]
pub mod Email;
#[cfg(feature = "ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "Holographic")]
pub mod Holographic;
#[cfg(feature = "LockScreen")]
pub mod LockScreen;
#[cfg(feature = "Payments")]
pub mod Payments;
#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Resources")]
pub mod Resources;
#[cfg(feature = "Search")]
pub mod Search;
#[cfg(feature = "SocialInfo")]
pub mod SocialInfo;
#[cfg(feature = "Store")]
pub mod Store;
#[cfg(feature = "UserActivities")]
pub mod UserActivities;
#[cfg(feature = "UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "Wallet")]
pub mod Wallet;
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
pub type IEnteredBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type ILeavingBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type ISuspendingDeferral = *mut ::core::ffi::c_void;
pub type ISuspendingEventArgs = *mut ::core::ffi::c_void;
pub type ISuspendingOperation = *mut ::core::ffi::c_void;
pub type LeavingBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type LimitedAccessFeatureRequestResult = *mut ::core::ffi::c_void;
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
