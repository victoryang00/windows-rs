pub type BufferProtectUnprotectResult = *mut ::core::ffi::c_void;
pub type DataProtectionInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: Self = Self(0i32);
    pub const Protected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Unprotected: Self = Self(3i32);
    pub const LicenseExpired: Self = Self(4i32);
    pub const AccessSuspended: Self = Self(5i32);
}
impl ::core::marker::Copy for DataProtectionStatus {}
impl ::core::clone::Clone for DataProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
impl ::core::marker::Copy for EnforcementLevel {}
impl ::core::clone::Clone for EnforcementLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FileProtectionInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: Self = Self(0i32);
    pub const Unknown: Self = Self(0i32);
    pub const Unprotected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Protected: Self = Self(3i32);
    pub const ProtectedByOtherUser: Self = Self(4i32);
    pub const ProtectedToOtherEnterprise: Self = Self(5i32);
    pub const NotProtectable: Self = Self(6i32);
    pub const ProtectedToOtherIdentity: Self = Self(7i32);
    pub const LicenseExpired: Self = Self(8i32);
    pub const AccessSuspended: Self = Self(9i32);
    pub const FileInUse: Self = Self(10i32);
}
impl ::core::marker::Copy for FileProtectionStatus {}
impl ::core::clone::Clone for FileProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FileUnprotectOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBufferProtectUnprotectResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataProtectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataProtectionStatus) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataProtectionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, unprotectedstream: *mut ::core::ffi::c_void, identity: ::windows_sys::core::HSTRING, protectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, protectedstream: *mut ::core::ffi::c_void, unprotectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, protecteddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStreamProtectionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, protectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStreamProtectionInfoAsync: usize,
}
#[repr(C)]
pub struct IFileProtectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FileProtectionStatus) -> ::windows_sys::core::HRESULT,
    pub IsRoamable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileProtectionInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsProtectWhileOpenSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileProtectionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyProtectionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveFileAsContainerAsync: unsafe extern "system" fn(this: *mut *mut Self, protectedfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveFileAsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerAsync: unsafe extern "system" fn(this: *mut *mut Self, containerfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAsync: unsafe extern "system" fn(this: *mut *mut Self, containerfile: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateProtectedAndOpenAsync: unsafe extern "system" fn(this: *mut *mut Self, parentfolder: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, identity: ::windows_sys::core::HSTRING, collisionoption: super::super::Storage::CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateProtectedAndOpenAsync: usize,
}
#[repr(C)]
pub struct IFileProtectionManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsContainerAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, containerfile: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, collisionoption: super::super::Storage::NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveFileAsContainerWithSharingAsync: unsafe extern "system" fn(this: *mut *mut Self, protectedfile: *mut ::core::ffi::c_void, sharedwithidentities: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveFileAsContainerWithSharingAsync: usize,
}
#[repr(C)]
pub struct IFileProtectionManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectWithOptionsAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFileRevocationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut *mut Self, storageitem: *mut ::core::ffi::c_void, enterpriseidentity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut *mut Self, sourcestorageitem: *mut ::core::ffi::c_void, targetstorageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    CopyProtectionAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Revoke: unsafe extern "system" fn(this: *mut *mut Self, enterpriseidentity: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Revoke: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, storageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    GetStatusAsync: usize,
}
#[repr(C)]
pub struct IFileUnprotectOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAudit: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Audit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileUnprotectOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audit: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectedAccessResumedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[repr(C)]
pub struct IProtectedAccessSuspendingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IProtectedContainerExportResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProtectedImportExportStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[repr(C)]
pub struct IProtectedContainerImportResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProtectedImportExportStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[repr(C)]
pub struct IProtectedContentRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[repr(C)]
pub struct IProtectedFileCreateResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyAuditInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAction: unsafe extern "system" fn(this: *mut *mut Self, value: ProtectionPolicyAuditAction) -> ::windows_sys::core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProtectionPolicyAuditAction) -> ::windows_sys::core::HRESULT,
    pub SetDataDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DataDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSourceDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTargetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TargetDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyAuditInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, action: ProtectionPolicyAuditAction, datadescription: ::windows_sys::core::HSTRING, sourcedescription: ::windows_sys::core::HSTRING, targetdescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithActionAndDataDescription: unsafe extern "system" fn(this: *mut *mut Self, action: ProtectionPolicyAuditAction, datadescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIdentity: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsIdentityManaged: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryApplyProcessUIPolicy: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ClearProcessUIPolicy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CreateCurrentThreadNetworkContext: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub GetPrimaryManagedIdentityForNetworkEndpointAsync: unsafe extern "system" fn(this: *mut *mut Self, endpointhost: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    GetPrimaryManagedIdentityForNetworkEndpointAsync: usize,
    pub RevokeContent: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessSuspending: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessSuspending: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessResumed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessResumed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedContentRevoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedContentRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedContentRevoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedContentRevoked: usize,
    pub CheckAccess: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct IProtectionPolicyManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub HasContentBeenRevokedSince: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, since: super::super::Foundation::DateTime, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HasContentBeenRevokedSince: usize,
    pub CheckAccessForApp: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, apppackagefamilyname: ::windows_sys::core::HSTRING, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, apppackagefamilyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppAsync: usize,
    pub GetEnforcementLevel: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut EnforcementLevel) -> ::windows_sys::core::HRESULT,
    pub IsUserDecryptionAllowed: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsProtectionUnderLockRequired: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PolicyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePolicyChanged: usize,
    pub IsProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, apppackagefamilyname: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, apppackagefamilyname: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithMessageAsync: usize,
    pub LogAuditEvent: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionPolicyManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRoamableProtectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithBehaviorAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, targetidentity: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithBehaviorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceidentity: ::windows_sys::core::HSTRING, apppackagefamilyname: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceitemlist: *mut ::core::ffi::c_void, apppackagefamilyname: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceitemlist: *mut ::core::ffi::c_void, apppackagefamilyname: ::windows_sys::core::HSTRING, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceitemlist: *mut ::core::ffi::c_void, processid: u32, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceitemlist: *mut ::core::ffi::c_void, processid: u32, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::windows_sys::core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredAsync: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredForNewFileAsync: unsafe extern "system" fn(this: *mut *mut Self, parentfolder: *mut ::core::ffi::c_void, identity: ::windows_sys::core::HSTRING, desiredname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredForNewFileAsync: usize,
    pub PrimaryManagedIdentity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetPrimaryManagedIdentityForIdentity: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThreadNetworkContext {
    pub base__: ::windows_sys::core::IInspectable,
}
pub type ProtectedAccessResumedEventArgs = *mut ::core::ffi::c_void;
pub type ProtectedAccessSuspendingEventArgs = *mut ::core::ffi::c_void;
pub type ProtectedContainerExportResult = *mut ::core::ffi::c_void;
pub type ProtectedContainerImportResult = *mut ::core::ffi::c_void;
pub type ProtectedContentRevokedEventArgs = *mut ::core::ffi::c_void;
pub type ProtectedFileCreateResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: Self = Self(0i32);
    pub const Undetermined: Self = Self(1i32);
    pub const Unprotected: Self = Self(2i32);
    pub const Revoked: Self = Self(3i32);
    pub const NotRoamable: Self = Self(4i32);
    pub const ProtectedToOtherIdentity: Self = Self(5i32);
    pub const LicenseExpired: Self = Self(6i32);
    pub const AccessSuspended: Self = Self(7i32);
}
impl ::core::marker::Copy for ProtectedImportExportStatus {}
impl ::core::clone::Clone for ProtectedImportExportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for ProtectionPolicyAuditAction {}
impl ::core::clone::Clone for ProtectionPolicyAuditAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProtectionPolicyAuditInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionPolicyEvaluationResult {}
impl ::core::clone::Clone for ProtectionPolicyEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProtectionPolicyManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
impl ::core::marker::Copy for ProtectionPolicyRequestAccessBehavior {}
impl ::core::clone::Clone for ProtectionPolicyRequestAccessBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ThreadNetworkContext = *mut ::core::ffi::c_void;
