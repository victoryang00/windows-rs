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
impl ::windows_sys::core::Interface for IBufferProtectUnprotectResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1201233628, data2: 27884, data3: 20026, data4: [178, 81, 158, 116, 133, 215, 158, 122] };
}
#[repr(C)]
pub struct IDataProtectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataProtectionStatus) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataProtectionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2216734913, data2: 24113, data3: 17413, data4: [149, 64, 63, 148, 58, 240, 203, 38] };
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
impl ::windows_sys::core::Interface for IDataProtectionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3054803828, data2: 37188, data3: 20196, data4: [138, 138, 48, 181, 243, 97, 67, 14] };
}
#[repr(C)]
pub struct IFileProtectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FileProtectionStatus) -> ::windows_sys::core::HRESULT,
    pub IsRoamable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileProtectionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1323918470, data2: 5246, data3: 19920, data4: [143, 175, 82, 83, 237, 145, 173, 12] };
}
#[repr(C)]
pub struct IFileProtectionInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsProtectWhileOpenSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileProtectionInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2182232652, data2: 21882, data3: 18829, data4: [142, 148, 148, 76, 213, 131, 100, 50] };
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
impl ::windows_sys::core::Interface for IFileProtectionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1481047195, data2: 58899, data3: 17003, data4: [187, 56, 136, 203, 161, 220, 154, 219] };
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
impl ::windows_sys::core::Interface for IFileProtectionManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2211620677, data2: 1155, data3: 16811, data4: [178, 213, 188, 127, 35, 215, 78, 187] };
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
impl ::windows_sys::core::Interface for IFileProtectionManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1763214490, data2: 25167, data3: 18134, data4: [178, 65, 233, 205, 95, 223, 62, 63] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IFileRevocationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627817533, data2: 7261, data3: 16992, data4: [140, 117, 145, 68, 207, 183, 139, 169] };
}
#[repr(C)]
pub struct IFileUnprotectOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAudit: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Audit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUnprotectOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2098402033, data2: 15117, data3: 19928, data4: [161, 248, 30, 197, 56, 34, 226, 243] };
}
#[repr(C)]
pub struct IFileUnprotectOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audit: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileUnprotectOptionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1370403740, data2: 55948, data3: 19519, data4: [155, 251, 203, 115, 167, 204, 224, 221] };
}
#[repr(C)]
pub struct IProtectedAccessResumedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
impl ::windows_sys::core::Interface for IProtectedAccessResumedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2890779225, data2: 23936, data3: 20117, data4: [140, 95, 133, 57, 69, 14, 235, 224] };
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
impl ::windows_sys::core::Interface for IProtectedAccessSuspendingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1973523424, data2: 41796, data3: 17055, data4: [185, 117, 4, 252, 31, 136, 193, 133] };
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
impl ::windows_sys::core::Interface for IProtectedContainerExportResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 961081237, data2: 63483, data3: 19266, data4: [175, 176, 223, 112, 180, 21, 67, 193] };
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
impl ::windows_sys::core::Interface for IProtectedContainerImportResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3451355345, data2: 59323, data3: 19738, data4: [147, 57, 52, 220, 65, 20, 159, 155] };
}
#[repr(C)]
pub struct IProtectedContentRevokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
impl ::windows_sys::core::Interface for IProtectedContentRevokedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1667786785, data2: 22713, data3: 18414, data4: [147, 217, 240, 247, 65, 207, 67, 240] };
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
impl ::windows_sys::core::Interface for IProtectedFileCreateResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 686026090, data2: 59879, data3: 18947, data4: [159, 83, 189, 177, 97, 114, 105, 155] };
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
impl ::windows_sys::core::Interface for IProtectionPolicyAuditInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1113241572, data2: 65207, data3: 17660, data4: [179, 187, 195, 196, 215, 236, 190, 187] };
}
#[repr(C)]
pub struct IProtectionPolicyAuditInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, action: ProtectionPolicyAuditAction, datadescription: ::windows_sys::core::HSTRING, sourcedescription: ::windows_sys::core::HSTRING, targetdescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithActionAndDataDescription: unsafe extern "system" fn(this: *mut *mut Self, action: ProtectionPolicyAuditAction, datadescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProtectionPolicyAuditInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2127829003, data2: 37608, data3: 17109, data4: [131, 212, 37, 68, 11, 66, 53, 73] };
}
#[repr(C)]
pub struct IProtectionPolicyManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIdentity: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProtectionPolicyManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3580902936, data2: 41101, data3: 18406, data4: [162, 64, 153, 52, 215, 22, 94, 181] };
}
#[repr(C)]
pub struct IProtectionPolicyManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProtectionPolicyManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2885112442, data2: 33845, data3: 16767, data4: [153, 182, 81, 190, 175, 54, 88, 136] };
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
impl ::windows_sys::core::Interface for IProtectionPolicyManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3233807462, data2: 35901, data3: 19798, data4: [136, 4, 198, 143, 10, 211, 46, 197] };
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
impl ::windows_sys::core::Interface for IProtectionPolicyManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3062864524, data2: 14816, data3: 17993, data4: [178, 228, 7, 10, 184, 165, 121, 179] };
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
impl ::windows_sys::core::Interface for IProtectionPolicyManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1224711820, data2: 27247, data3: 19871, data4: [188, 237, 24, 171, 83, 122, 160, 21] };
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
impl ::windows_sys::core::Interface for IProtectionPolicyManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 548902107, data2: 52413, data3: 18703, data4: [140, 131, 73, 204, 183, 122, 234, 108] };
}
#[repr(C)]
pub struct IThreadNetworkContext {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IThreadNetworkContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199459049, data2: 61203, data3: 16474, data4: [177, 44, 215, 52, 140, 111, 65, 252] };
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
