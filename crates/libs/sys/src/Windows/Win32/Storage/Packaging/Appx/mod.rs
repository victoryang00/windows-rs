#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn AddPackageDependency(packagedependencyid: ::windows_sys::core::PCWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckIsMSIXPackage(packagefullname: ::windows_sys::core::PCWSTR, ismsixpackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn CreatePackageVirtualizationContext(packagefamilyname: ::windows_sys::core::PCWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn DeactivatePackageVirtualizationContext(cookie: usize);
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn DeletePackageDependency(packagedependencyid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindPackagesByPackageFamily(packagefamilyname: ::windows_sys::core::PCWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut ::windows_sys::core::PWSTR, bufferlength: *mut u32, buffer: ::windows_sys::core::PWSTR, packageproperties: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatApplicationUserModelId(packagefamilyname: ::windows_sys::core::PCWSTR, packagerelativeapplicationid: ::windows_sys::core::PCWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath(pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName(packagefullname: ::windows_sys::core::PCWSTR, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName2(packagefullname: ::windows_sys::core::PCWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagesByPackageFamily(packagefamilyname: ::windows_sys::core::PCWSTR, count: *mut u32, packagefullnames: *mut ::windows_sys::core::PWSTR, bufferlength: *mut u32, buffer: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessesInVirtualizationContext(packagefamilyname: ::windows_sys::core::PCWSTR, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: ::windows_sys::core::PCWSTR, packagefullname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackageOrigin(packagefullname: ::windows_sys::core::PCWSTR, origin: *mut PackageOrigin) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName(packagefullname: ::windows_sys::core::PCWSTR, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName2(packagefullname: ::windows_sys::core::PCWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullName(packagefullname: ::windows_sys::core::PCWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullNameForUser(usersid: super::super::super::Foundation::PSID, packagefullname: ::windows_sys::core::PCWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromFullName(packagefullname: ::windows_sys::core::PCWSTR, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageIdFromFullName(packagefullname: ::windows_sys::core::PCWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: ::windows_sys::core::PCWSTR, packagenamelength: *mut u32, packagename: ::windows_sys::core::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseApplicationUserModelId(applicationusermodelid: ::windows_sys::core::PCWSTR, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_sys::core::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
    pub fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCreatePackageDependency(user: super::super::super::Foundation::PSID, packagefamilyname: ::windows_sys::core::PCWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: ::windows_sys::core::PCWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyApplicationUserModelId(applicationusermodelid: ::windows_sys::core::PCWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFamilyName(packagefamilyname: ::windows_sys::core::PCWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFullName(packagefullname: ::windows_sys::core::PCWSTR) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageId(packageid: *const PACKAGE_ID) -> super::super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: ::windows_sys::core::PCWSTR) -> super::super::super::Foundation::WIN32_ERROR;
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_BUNDLE_FOOTPRINT_FILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_CAPABILITIES = u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = 128u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_CAPABILITY_CLASS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_COMPRESSION_OPTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for APPX_ENCRYPTED_EXEMPTIONS {}
impl ::core::clone::Clone for APPX_ENCRYPTED_EXEMPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_ENCRYPTED_PACKAGE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows_sys::core::PCWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: *mut *mut *mut *mut super::super::super::System::Com::IUri,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows_sys::core::PCWSTR,
    pub blockMapHashAlgorithm: *mut *mut *mut *mut super::super::super::System::Com::IUri,
    pub options: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_FOOTPRINT_FILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl ::core::marker::Copy for APPX_KEY_INFO {}
impl ::core::clone::Clone for APPX_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_PACKAGE_ARCHITECTURE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_PACKAGE_ARCHITECTURE2 = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = 65535i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: *mut *mut *mut *mut super::super::super::System::Com::IUri,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: *mut *mut *mut *mut super::super::super::System::Com::IStream,
    pub fileName: ::windows_sys::core::PCWSTR,
    pub contentType: ::windows_sys::core::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type APPX_PACKAGING_CONTEXT_CHANGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AddPackageDependencyOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyClrCompat = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyCreateFileAccess = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyLifecycleManagement = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyMediaFoundationCodecLoading = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyProcessTerminationMethod = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyShowDeveloperDiagnostic = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyThreadInitializationType = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type AppPolicyWindowingModel = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = 3i32;
pub const AppxBundleFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 932054086, data2: 21380, data3: 17335, data4: [136, 119, 231, 219, 221, 136, 52, 70] };
pub const AppxEncryptionFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3697692637, data2: 55400, data3: 18158, data4: [135, 128, 141, 25, 108, 183, 57, 247] };
pub const AppxFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1480761664, data2: 65439, data3: 16742, data4: [143, 92, 98, 245, 183, 176, 199, 129] };
pub const AppxPackageEditor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4026856138, data2: 44732, data3: 19213, data4: [191, 88, 229, 22, 213, 188, 192, 171] };
pub const AppxPackagingDiagnosticEventSinkManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1355418182, data2: 5512, data3: 16737, data4: [142, 210, 239, 158, 70, 156, 237, 93] };
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type CreatePackageDependencyOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type DX_FEATURE_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = 3i32;
#[repr(C)]
pub struct IAppxBlockMapBlock {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetHash: unsafe extern "system" fn(this: *mut *mut Self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetCompressedSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBlockMapBlocksEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, block: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxBlockMapFile {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBlocks: unsafe extern "system" fn(this: *mut *mut Self, blocks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLocalFileHeaderSize: unsafe extern "system" fn(this: *mut *mut Self, lfhsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetUncompressedSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ValidateFileHash: unsafe extern "system" fn(this: *mut *mut Self, filestream: *mut ::core::ffi::c_void, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ValidateFileHash: usize,
}
#[repr(C)]
pub struct IAppxBlockMapFilesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, file: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxBlockMapReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(this: *mut *mut Self, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHashMethod: unsafe extern "system" fn(this: *mut *mut Self, hashmethod: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHashMethod: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, blockmapstream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[repr(C)]
pub struct IAppxBundleFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleManifestReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleManifestReader: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestOptionalBundleInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageId: unsafe extern "system" fn(this: *mut *mut Self, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(this: *mut *mut Self, filename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(this: *mut *mut Self, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, optionalbundle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageType: unsafe extern "system" fn(this: *mut *mut Self, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows_sys::core::HRESULT,
    pub GetPackageId: unsafe extern "system" fn(this: *mut *mut Self, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(this: *mut *mut Self, filename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetResources: unsafe extern "system" fn(this: *mut *mut Self, resources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsPackageReference: unsafe extern "system" fn(this: *mut *mut Self, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsPackageReference: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(this: *mut *mut Self, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsNonQualifiedResourcePackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsDefaultApplicablePackage: unsafe extern "system" fn(this: *mut *mut Self, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsDefaultApplicablePackage: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo3 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(this: *mut *mut Self, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo4 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsStub: unsafe extern "system" fn(this: *mut *mut Self, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsStub: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfoEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, packageinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageId: unsafe extern "system" fn(this: *mut *mut Self, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(this: *mut *mut Self, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[repr(C)]
pub struct IAppxBundleManifestReader2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetOptionalBundles: unsafe extern "system" fn(this: *mut *mut Self, optionalbundles: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFootprintFile: unsafe extern "system" fn(this: *mut *mut Self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetBlockMap: unsafe extern "system" fn(this: *mut *mut Self, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(this: *mut *mut Self, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPayloadPackages: unsafe extern "system" fn(this: *mut *mut Self, payloadpackages: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPayloadPackage: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, payloadpackage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleWriter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleWriter2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[repr(C)]
pub struct IAppxBundleWriter3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, hashmethodstring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxBundleWriter4 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPayloadPackage: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPayloadPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPackageReference: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddExternalPackageReference: usize,
}
#[repr(C)]
pub struct IAppxContentGroup {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, groupname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(this: *mut *mut Self, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxContentGroupFilesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxContentGroupMapReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRequiredGroup: unsafe extern "system" fn(this: *mut *mut Self, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(this: *mut *mut Self, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxContentGroupMapWriter {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddAutomaticGroup: unsafe extern "system" fn(this: *mut *mut Self, groupname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub AddAutomaticFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxContentGroupsEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPayloadPackageEncrypted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddExternalPackageReference: usize,
}
#[repr(C)]
pub struct IAppxEncryptedPackageWriter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFileEncrypted: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFileEncrypted: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxEncryptedPackageWriter2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFilesEncrypted: unsafe extern "system" fn(this: *mut *mut Self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFilesEncrypted: usize,
}
#[repr(C)]
pub struct IAppxEncryptionFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptPackage: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EncryptBundle: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptBundle: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptBundle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleReader: usize,
}
#[repr(C)]
pub struct IAppxEncryptionFactory2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedPackageWriter: usize,
}
#[repr(C)]
pub struct IAppxEncryptionFactory3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptBundle: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleWriter: usize,
}
#[repr(C)]
pub struct IAppxEncryptionFactory4 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
}
#[repr(C)]
pub struct IAppxFactory {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateManifestReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateManifestReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlockMapReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlockMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValidatedBlockMapReader: unsafe extern "system" fn(this: *mut *mut Self, blockmapstream: *mut ::core::ffi::c_void, signaturefilename: ::windows_sys::core::PCWSTR, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValidatedBlockMapReader: usize,
}
#[repr(C)]
pub struct IAppxFactory2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, contentgroupmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSourceContentGroupMapReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, reader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSourceContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapWriter: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, contentgroupmapwriter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapWriter: usize,
}
#[repr(C)]
pub struct IAppxFile {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCompressionOption: unsafe extern "system" fn(this: *mut *mut Self, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows_sys::core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut *mut Self, contenttype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, filename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[repr(C)]
pub struct IAppxFilesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, file: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestApplication {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, appusermodelid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestApplicationsEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, application: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestCapabilitiesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, capability: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, devicecapability: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestDriverConstraint {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut *mut Self, minversion: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetMinDate: unsafe extern "system" fn(this: *mut *mut Self, mindate: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestDriverConstraintsEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, driverconstraint: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestDriverDependenciesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, driverdependency: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestDriverDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDriverConstraints: unsafe extern "system" fn(this: *mut *mut Self, driverconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, hostruntimedependency: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut *mut Self, publisher: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut *mut Self, minversion: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependency2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestMainPackageDependenciesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, mainpackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestMainPackageDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut *mut Self, publisher: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestOSPackageDependenciesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, ospackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestOSPackageDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, version: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestOptionalPackageInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsOptionalPackage: unsafe extern "system" fn(this: *mut *mut Self, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsOptionalPackage: usize,
    pub GetMainPackageName: unsafe extern "system" fn(this: *mut *mut Self, mainpackagename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestPackageDependenciesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, dependency: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestPackageDependency {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut *mut Self, publisher: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut *mut Self, minversion: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestPackageDependency2 {
    pub base__: IAppxManifestPackageDependency,
    pub GetMaxMajorVersionTested: unsafe extern "system" fn(this: *mut *mut Self, maxmajorversiontested: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestPackageDependency3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsOptional: unsafe extern "system" fn(this: *mut *mut Self, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsOptional: usize,
}
#[repr(C)]
pub struct IAppxManifestPackageId {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetArchitecture: unsafe extern "system" fn(this: *mut *mut Self, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows_sys::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut *mut Self, publisher: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut *mut Self, packageversion: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetResourceId: unsafe extern "system" fn(this: *mut *mut Self, resourceid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ComparePublisher: unsafe extern "system" fn(this: *mut *mut Self, other: ::windows_sys::core::PCWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComparePublisher: usize,
    pub GetPackageFullName: unsafe extern "system" fn(this: *mut *mut Self, packagefullname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, packagefamilyname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestPackageId2 {
    pub base__: IAppxManifestPackageId,
    pub GetArchitecture2: unsafe extern "system" fn(this: *mut *mut Self, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestProperties {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoolValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoolValue: usize,
    pub GetStringValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestQualifiedResource {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetScale: unsafe extern "system" fn(this: *mut *mut Self, scale: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetDXFeatureLevel: unsafe extern "system" fn(this: *mut *mut Self, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestQualifiedResourcesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, resource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPackageId: unsafe extern "system" fn(this: *mut *mut Self, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut *mut Self, packageproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPackageDependencies: unsafe extern "system" fn(this: *mut *mut Self, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, capabilities: *mut APPX_CAPABILITIES) -> ::windows_sys::core::HRESULT,
    pub GetResources: unsafe extern "system" fn(this: *mut *mut Self, resources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(this: *mut *mut Self, devicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrerequisite: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetApplications: unsafe extern "system" fn(this: *mut *mut Self, applications: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut *mut Self, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[repr(C)]
pub struct IAppxManifestReader2 {
    pub base__: IAppxManifestReader,
    pub GetQualifiedResources: unsafe extern "system" fn(this: *mut *mut Self, resources: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestReader3 {
    pub base__: IAppxManifestReader2,
    pub GetCapabilitiesByCapabilityClass: unsafe extern "system" fn(this: *mut *mut Self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(this: *mut *mut Self, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestReader4 {
    pub base__: IAppxManifestReader3,
    pub GetOptionalPackageInfo: unsafe extern "system" fn(this: *mut *mut Self, optionalpackageinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestReader5 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMainPackageDependencies: unsafe extern "system" fn(this: *mut *mut Self, mainpackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestReader6 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(this: *mut *mut Self, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsNonQualifiedResourcePackage: usize,
}
#[repr(C)]
pub struct IAppxManifestReader7 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDriverDependencies: unsafe extern "system" fn(this: *mut *mut Self, driverdependencies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOSPackageDependencies: unsafe extern "system" fn(this: *mut *mut Self, ospackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHostRuntimeDependencies: unsafe extern "system" fn(this: *mut *mut Self, hostruntimedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxManifestResourcesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, resource: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCurrent: unsafe extern "system" fn(this: *mut *mut Self, targetdevicefamily: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut *mut Self, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[repr(C)]
pub struct IAppxManifestTargetDeviceFamily {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut *mut Self, minversion: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetMaxVersionTested: unsafe extern "system" fn(this: *mut *mut Self, maxversiontested: *mut u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxPackageEditor {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, workingdirectory: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackage: unsafe extern "system" fn(this: *mut *mut Self, updatedpackagestream: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackageUsingBaselineBlockMap: unsafe extern "system" fn(this: *mut *mut Self, updatedpackagestream: *mut ::core::ffi::c_void, baselineblockmapstream: *mut ::core::ffi::c_void, baselinepackagefullname: ::windows_sys::core::PCWSTR, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackageUsingBaselineBlockMap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdatePackage: unsafe extern "system" fn(this: *mut *mut Self, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdatePackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateEncryptedPackage: unsafe extern "system" fn(this: *mut *mut Self, baselineencryptedpackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateEncryptedPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdatePackageManifest: unsafe extern "system" fn(this: *mut *mut Self, packagestream: *mut ::core::ffi::c_void, updatedmanifeststream: *mut ::core::ffi::c_void, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdatePackageManifest: usize,
}
#[repr(C)]
pub struct IAppxPackageReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBlockMap: unsafe extern "system" fn(this: *mut *mut Self, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFootprintFile: unsafe extern "system" fn(this: *mut *mut Self, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPayloadFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPayloadFiles: unsafe extern "system" fn(this: *mut *mut Self, filesenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(this: *mut *mut Self, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxPackageWriter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFile: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::PCWSTR, contenttype: ::windows_sys::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, manifest: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[repr(C)]
pub struct IAppxPackageWriter2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, manifest: *mut ::core::ffi::c_void, contentgroupmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[repr(C)]
pub struct IAppxPackageWriter3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFiles: unsafe extern "system" fn(this: *mut *mut Self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFiles: usize,
}
#[repr(C)]
pub struct IAppxPackagingDiagnosticEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReportContextChange: unsafe extern "system" fn(this: *mut *mut Self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: ::windows_sys::core::PCSTR, contextmessage: ::windows_sys::core::PCWSTR, detailsmessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, errormessage: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxPackagingDiagnosticEventSinkManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetSinkForProcess: unsafe extern "system" fn(this: *mut *mut Self, sink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppxSourceContentGroupMapReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRequiredGroup: unsafe extern "system" fn(this: *mut *mut Self, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(this: *mut *mut Self, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::clone::Clone for PACKAGEDEPENDENCY_CONTEXT__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: ::windows_sys::core::PWSTR,
    pub publisher: ::windows_sys::core::PWSTR,
    pub resourceId: ::windows_sys::core::PWSTR,
    pub publisherId: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PACKAGE_ID {}
impl ::core::clone::Clone for PACKAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: ::windows_sys::core::PWSTR,
    pub packageFullName: ::windows_sys::core::PWSTR,
    pub packageFamilyName: ::windows_sys::core::PWSTR,
    pub packageId: PACKAGE_ID,
}
impl ::core::marker::Copy for PACKAGE_INFO {}
impl ::core::clone::Clone for PACKAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION {}
impl ::core::clone::Clone for PACKAGE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::clone::Clone for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type PackageDependencyLifetimeKind = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type PackageDependencyProcessorArchitectures = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type PackageOrigin = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Unknown: PackageOrigin = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Unsigned: PackageOrigin = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Inbox: PackageOrigin = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Store: PackageOrigin = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_DeveloperSigned: PackageOrigin = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_LineOfBusiness: PackageOrigin = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub type PackagePathType = i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Install: PackagePathType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Mutable: PackagePathType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Effective: PackagePathType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_MachineExternal: PackagePathType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_UserExternal: PackagePathType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_EffectiveExternal: PackagePathType = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for _PACKAGE_INFO_REFERENCE {}
impl ::core::clone::Clone for _PACKAGE_INFO_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
