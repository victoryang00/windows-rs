#[link(name = "windows")]
extern "system" {
    pub fn AddVirtualDiskParent(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parentpath: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn ApplySnapshotVhdSet(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parameters: *const APPLY_SNAPSHOT_VHDSET_PARAMETERS, flags: APPLY_SNAPSHOT_VHDSET_FLAG) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(all(feature = "win32-security-sys", feature = "win32-system-sys"))]
    pub fn AttachVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, securitydescriptor: ::win32_security_sys::PSECURITY_DESCRIPTOR, flags: ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: *const ATTACH_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn BreakMirrorVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn CompactVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: COMPACT_VIRTUAL_DISK_FLAG, parameters: *const COMPACT_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn CompleteForkVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(all(feature = "win32-security-sys", feature = "win32-system-sys"))]
    pub fn CreateVirtualDisk(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: ::windows_core_sys::PCWSTR, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, securitydescriptor: ::win32_security_sys::PSECURITY_DESCRIPTOR, flags: CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: *const CREATE_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, handle: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn DeleteSnapshotVhdSet(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parameters: *const DELETE_SNAPSHOT_VHDSET_PARAMETERS, flags: DELETE_SNAPSHOT_VHDSET_FLAG) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn DeleteVirtualDiskMetadata(virtualdiskhandle: ::win32_foundation_sys::HANDLE, item: *const ::windows_core_sys::GUID) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn DetachVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn EnumerateVirtualDiskMetadata(virtualdiskhandle: ::win32_foundation_sys::HANDLE, numberofitems: *mut u32, items: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn ExpandVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: EXPAND_VIRTUAL_DISK_FLAG, parameters: *const EXPAND_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn ForkVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: FORK_VIRTUAL_DISK_FLAG, parameters: *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped: *mut ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes: *mut u32, pathsbuffer: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetStorageDependencyInformation(objecthandle: ::win32_foundation_sys::HANDLE, flags: GET_STORAGE_DEPENDENCY_FLAG, storagedependencyinfosize: u32, storagedependencyinfo: *mut STORAGE_DEPENDENCY_INFO, sizeused: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetVirtualDiskInformation(virtualdiskhandle: ::win32_foundation_sys::HANDLE, virtualdiskinfosize: *mut u32, virtualdiskinfo: *mut GET_VIRTUAL_DISK_INFO, sizeused: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetVirtualDiskMetadata(virtualdiskhandle: ::win32_foundation_sys::HANDLE, item: *const ::windows_core_sys::GUID, metadatasize: *mut u32, metadata: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetVirtualDiskOperationProgress(virtualdiskhandle: ::win32_foundation_sys::HANDLE, overlapped: *const ::win32_system_sys::IO::OVERLAPPED, progress: *mut VIRTUAL_DISK_PROGRESS) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetVirtualDiskPhysicalPath(virtualdiskhandle: ::win32_foundation_sys::HANDLE, diskpathsizeinbytes: *mut u32, diskpath: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn MergeVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: MERGE_VIRTUAL_DISK_FLAG, parameters: *const MERGE_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn MirrorVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: MIRROR_VIRTUAL_DISK_FLAG, parameters: *const MIRROR_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn ModifyVhdSet(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parameters: *const MODIFY_VHDSET_PARAMETERS, flags: MODIFY_VHDSET_FLAG) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn OpenVirtualDisk(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: ::windows_core_sys::PCWSTR, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, flags: OPEN_VIRTUAL_DISK_FLAG, parameters: *const OPEN_VIRTUAL_DISK_PARAMETERS, handle: *mut ::win32_foundation_sys::HANDLE) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn QueryChangesVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, changetrackingid: ::windows_core_sys::PCWSTR, byteoffset: u64, bytelength: u64, flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges: *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount: *mut u32, processedlength: *mut u64) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn RawSCSIVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parameters: *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags: RAW_SCSI_VIRTUAL_DISK_FLAG, response: *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE) -> ::win32_foundation_sys::WIN32_ERROR;
    #[cfg(feature = "win32-system-sys")]
    pub fn ResizeVirtualDisk(virtualdiskhandle: ::win32_foundation_sys::HANDLE, flags: RESIZE_VIRTUAL_DISK_FLAG, parameters: *const RESIZE_VIRTUAL_DISK_PARAMETERS, overlapped: *const ::win32_system_sys::IO::OVERLAPPED) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn SetVirtualDiskInformation(virtualdiskhandle: ::win32_foundation_sys::HANDLE, virtualdiskinfo: *const SET_VIRTUAL_DISK_INFO) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn SetVirtualDiskMetadata(virtualdiskhandle: ::win32_foundation_sys::HANDLE, item: *const ::windows_core_sys::GUID, metadatasize: u32, metadata: *const ::core::ffi::c_void) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn TakeSnapshotVhdSet(virtualdiskhandle: ::win32_foundation_sys::HANDLE, parameters: *const TAKE_SNAPSHOT_VHDSET_PARAMETERS, flags: TAKE_SNAPSHOT_VHDSET_FLAG) -> ::win32_foundation_sys::WIN32_ERROR;
}
pub type APPLY_SNAPSHOT_VHDSET_FLAG = u32;
pub const APPLY_SNAPSHOT_VHDSET_FLAG_NONE: APPLY_SNAPSHOT_VHDSET_FLAG = 0u32;
pub const APPLY_SNAPSHOT_VHDSET_FLAG_WRITEABLE: APPLY_SNAPSHOT_VHDSET_FLAG = 1u32;
#[repr(C)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: APPLY_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl ::core::marker::Copy for APPLY_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::core::clone::Clone for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl ::core::marker::Copy for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::core::clone::Clone for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows_core_sys::GUID,
    pub LeafSnapshotId: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::core::clone::Clone for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APPLY_SNAPSHOT_VHDSET_VERSION = i32;
pub const APPLY_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: APPLY_SNAPSHOT_VHDSET_VERSION = 0i32;
pub const APPLY_SNAPSHOT_VHDSET_VERSION_1: APPLY_SNAPSHOT_VHDSET_VERSION = 1i32;
pub type ATTACH_VIRTUAL_DISK_FLAG = u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NONE: ATTACH_VIRTUAL_DISK_FLAG = 0u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY: ATTACH_VIRTUAL_DISK_FLAG = 1u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER: ATTACH_VIRTUAL_DISK_FLAG = 2u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME: ATTACH_VIRTUAL_DISK_FLAG = 4u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST: ATTACH_VIRTUAL_DISK_FLAG = 8u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR: ATTACH_VIRTUAL_DISK_FLAG = 16u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY: ATTACH_VIRTUAL_DISK_FLAG = 32u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NON_PNP: ATTACH_VIRTUAL_DISK_FLAG = 64u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_RESTRICTED_RANGE: ATTACH_VIRTUAL_DISK_FLAG = 128u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_SINGLE_PARTITION: ATTACH_VIRTUAL_DISK_FLAG = 256u32;
pub const ATTACH_VIRTUAL_DISK_FLAG_REGISTER_VOLUME: ATTACH_VIRTUAL_DISK_FLAG = 512u32;
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS {
    pub Version: ATTACH_VIRTUAL_DISK_VERSION,
    pub Anonymous: ATTACH_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for ATTACH_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for ATTACH_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: ATTACH_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: ATTACH_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl ::core::marker::Copy for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl ::core::marker::Copy for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub RestrictedOffset: u64,
    pub RestrictedLength: u64,
}
impl ::core::marker::Copy for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::core::clone::Clone for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ATTACH_VIRTUAL_DISK_VERSION = i32;
pub const ATTACH_VIRTUAL_DISK_VERSION_UNSPECIFIED: ATTACH_VIRTUAL_DISK_VERSION = 0i32;
pub const ATTACH_VIRTUAL_DISK_VERSION_1: ATTACH_VIRTUAL_DISK_VERSION = 1i32;
pub const ATTACH_VIRTUAL_DISK_VERSION_2: ATTACH_VIRTUAL_DISK_VERSION = 2i32;
pub type COMPACT_VIRTUAL_DISK_FLAG = u32;
pub const COMPACT_VIRTUAL_DISK_FLAG_NONE: COMPACT_VIRTUAL_DISK_FLAG = 0u32;
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_ZERO_SCAN: COMPACT_VIRTUAL_DISK_FLAG = 1u32;
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_BLOCK_MOVES: COMPACT_VIRTUAL_DISK_FLAG = 2u32;
#[repr(C)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS {
    pub Version: COMPACT_VIRTUAL_DISK_VERSION,
    pub Anonymous: COMPACT_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for COMPACT_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for COMPACT_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: COMPACT_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl ::core::marker::Copy for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type COMPACT_VIRTUAL_DISK_VERSION = i32;
pub const COMPACT_VIRTUAL_DISK_VERSION_UNSPECIFIED: COMPACT_VIRTUAL_DISK_VERSION = 0i32;
pub const COMPACT_VIRTUAL_DISK_VERSION_1: COMPACT_VIRTUAL_DISK_VERSION = 1i32;
pub type CREATE_VIRTUAL_DISK_FLAG = u32;
pub const CREATE_VIRTUAL_DISK_FLAG_NONE: CREATE_VIRTUAL_DISK_FLAG = 0u32;
pub const CREATE_VIRTUAL_DISK_FLAG_FULL_PHYSICAL_ALLOCATION: CREATE_VIRTUAL_DISK_FLAG = 1u32;
pub const CREATE_VIRTUAL_DISK_FLAG_PREVENT_WRITES_TO_SOURCE_DISK: CREATE_VIRTUAL_DISK_FLAG = 2u32;
pub const CREATE_VIRTUAL_DISK_FLAG_DO_NOT_COPY_METADATA_FROM_PARENT: CREATE_VIRTUAL_DISK_FLAG = 4u32;
pub const CREATE_VIRTUAL_DISK_FLAG_CREATE_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = 8u32;
pub const CREATE_VIRTUAL_DISK_FLAG_USE_CHANGE_TRACKING_SOURCE_LIMIT: CREATE_VIRTUAL_DISK_FLAG = 16u32;
pub const CREATE_VIRTUAL_DISK_FLAG_PRESERVE_PARENT_CHANGE_TRACKING_STATE: CREATE_VIRTUAL_DISK_FLAG = 32u32;
pub const CREATE_VIRTUAL_DISK_FLAG_VHD_SET_USE_ORIGINAL_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = 64u32;
pub const CREATE_VIRTUAL_DISK_FLAG_SPARSE_FILE: CREATE_VIRTUAL_DISK_FLAG = 128u32;
pub const CREATE_VIRTUAL_DISK_FLAG_PMEM_COMPATIBLE: CREATE_VIRTUAL_DISK_FLAG = 256u32;
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: CREATE_VIRTUAL_DISK_FLAG = 512u32;
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: CREATE_VIRTUAL_DISK_FLAG = 1024u32;
#[repr(C)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS {
    pub Version: CREATE_VIRTUAL_DISK_VERSION,
    pub Anonymous: CREATE_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: CREATE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: CREATE_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: CREATE_VIRTUAL_DISK_PARAMETERS_0_2,
    pub Version4: CREATE_VIRTUAL_DISK_PARAMETERS_0_3,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub UniqueId: ::windows_core_sys::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub ParentPath: ::windows_core_sys::PCWSTR,
    pub SourcePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub UniqueId: ::windows_core_sys::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: ::windows_core_sys::PCWSTR,
    pub SourcePath: ::windows_core_sys::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub UniqueId: ::windows_core_sys::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: ::windows_core_sys::PCWSTR,
    pub SourcePath: ::windows_core_sys::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows_core_sys::GUID,
    pub SourceLimitPath: ::windows_core_sys::PCWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    pub UniqueId: ::windows_core_sys::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: ::windows_core_sys::PCWSTR,
    pub SourcePath: ::windows_core_sys::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows_core_sys::GUID,
    pub SourceLimitPath: ::windows_core_sys::PCWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
    pub PmemAddressAbstractionType: ::windows_core_sys::GUID,
    pub DataAlignment: u64,
}
impl ::core::marker::Copy for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {}
impl ::core::clone::Clone for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_BLOCK_SIZE: u32 = 0u32;
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_SECTOR_SIZE: u32 = 0u32;
pub type CREATE_VIRTUAL_DISK_VERSION = i32;
pub const CREATE_VIRTUAL_DISK_VERSION_UNSPECIFIED: CREATE_VIRTUAL_DISK_VERSION = 0i32;
pub const CREATE_VIRTUAL_DISK_VERSION_1: CREATE_VIRTUAL_DISK_VERSION = 1i32;
pub const CREATE_VIRTUAL_DISK_VERSION_2: CREATE_VIRTUAL_DISK_VERSION = 2i32;
pub const CREATE_VIRTUAL_DISK_VERSION_3: CREATE_VIRTUAL_DISK_VERSION = 3i32;
pub const CREATE_VIRTUAL_DISK_VERSION_4: CREATE_VIRTUAL_DISK_VERSION = 4i32;
pub type DELETE_SNAPSHOT_VHDSET_FLAG = u32;
pub const DELETE_SNAPSHOT_VHDSET_FLAG_NONE: DELETE_SNAPSHOT_VHDSET_FLAG = 0u32;
pub const DELETE_SNAPSHOT_VHDSET_FLAG_PERSIST_RCT: DELETE_SNAPSHOT_VHDSET_FLAG = 1u32;
#[repr(C)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: DELETE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl ::core::marker::Copy for DELETE_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::core::clone::Clone for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl ::core::marker::Copy for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::core::clone::Clone for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::core::clone::Clone for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DELETE_SNAPSHOT_VHDSET_VERSION = i32;
pub const DELETE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: DELETE_SNAPSHOT_VHDSET_VERSION = 0i32;
pub const DELETE_SNAPSHOT_VHDSET_VERSION_1: DELETE_SNAPSHOT_VHDSET_VERSION = 1i32;
pub type DEPENDENT_DISK_FLAG = u32;
pub const DEPENDENT_DISK_FLAG_NONE: DEPENDENT_DISK_FLAG = 0u32;
pub const DEPENDENT_DISK_FLAG_MULT_BACKING_FILES: DEPENDENT_DISK_FLAG = 1u32;
pub const DEPENDENT_DISK_FLAG_FULLY_ALLOCATED: DEPENDENT_DISK_FLAG = 2u32;
pub const DEPENDENT_DISK_FLAG_READ_ONLY: DEPENDENT_DISK_FLAG = 4u32;
pub const DEPENDENT_DISK_FLAG_REMOTE: DEPENDENT_DISK_FLAG = 8u32;
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME: DEPENDENT_DISK_FLAG = 16u32;
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME_PARENT: DEPENDENT_DISK_FLAG = 32u32;
pub const DEPENDENT_DISK_FLAG_REMOVABLE: DEPENDENT_DISK_FLAG = 64u32;
pub const DEPENDENT_DISK_FLAG_NO_DRIVE_LETTER: DEPENDENT_DISK_FLAG = 128u32;
pub const DEPENDENT_DISK_FLAG_PARENT: DEPENDENT_DISK_FLAG = 256u32;
pub const DEPENDENT_DISK_FLAG_NO_HOST_DISK: DEPENDENT_DISK_FLAG = 512u32;
pub const DEPENDENT_DISK_FLAG_PERMANENT_LIFETIME: DEPENDENT_DISK_FLAG = 1024u32;
pub const DEPENDENT_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: DEPENDENT_DISK_FLAG = 2048u32;
pub const DEPENDENT_DISK_FLAG_ALWAYS_ALLOW_SPARSE: DEPENDENT_DISK_FLAG = 4096u32;
pub const DEPENDENT_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: DEPENDENT_DISK_FLAG = 8192u32;
pub type DETACH_VIRTUAL_DISK_FLAG = u32;
pub const DETACH_VIRTUAL_DISK_FLAG_NONE: DETACH_VIRTUAL_DISK_FLAG = 0u32;
pub type EXPAND_VIRTUAL_DISK_FLAG = u32;
pub const EXPAND_VIRTUAL_DISK_FLAG_NONE: EXPAND_VIRTUAL_DISK_FLAG = 0u32;
pub const EXPAND_VIRTUAL_DISK_FLAG_NOTIFY_CHANGE: EXPAND_VIRTUAL_DISK_FLAG = 1u32;
#[repr(C)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS {
    pub Version: EXPAND_VIRTUAL_DISK_VERSION,
    pub Anonymous: EXPAND_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for EXPAND_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for EXPAND_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: EXPAND_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl ::core::marker::Copy for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EXPAND_VIRTUAL_DISK_VERSION = i32;
pub const EXPAND_VIRTUAL_DISK_VERSION_UNSPECIFIED: EXPAND_VIRTUAL_DISK_VERSION = 0i32;
pub const EXPAND_VIRTUAL_DISK_VERSION_1: EXPAND_VIRTUAL_DISK_VERSION = 1i32;
pub type FORK_VIRTUAL_DISK_FLAG = u32;
pub const FORK_VIRTUAL_DISK_FLAG_NONE: FORK_VIRTUAL_DISK_FLAG = 0u32;
pub const FORK_VIRTUAL_DISK_FLAG_EXISTING_FILE: FORK_VIRTUAL_DISK_FLAG = 1u32;
#[repr(C)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS {
    pub Version: FORK_VIRTUAL_DISK_VERSION,
    pub Anonymous: FORK_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for FORK_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for FORK_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union FORK_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: FORK_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for FORK_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub ForkedVirtualDiskPath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FORK_VIRTUAL_DISK_VERSION = i32;
pub const FORK_VIRTUAL_DISK_VERSION_UNSPECIFIED: FORK_VIRTUAL_DISK_VERSION = 0i32;
pub const FORK_VIRTUAL_DISK_VERSION_1: FORK_VIRTUAL_DISK_VERSION = 1i32;
pub type GET_STORAGE_DEPENDENCY_FLAG = u32;
pub const GET_STORAGE_DEPENDENCY_FLAG_NONE: GET_STORAGE_DEPENDENCY_FLAG = 0u32;
pub const GET_STORAGE_DEPENDENCY_FLAG_HOST_VOLUMES: GET_STORAGE_DEPENDENCY_FLAG = 1u32;
pub const GET_STORAGE_DEPENDENCY_FLAG_DISK_HANDLE: GET_STORAGE_DEPENDENCY_FLAG = 2u32;
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO {
    pub Version: GET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: GET_VIRTUAL_DISK_INFO_0,
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union GET_VIRTUAL_DISK_INFO_0 {
    pub Size: GET_VIRTUAL_DISK_INFO_0_3,
    pub Identifier: ::windows_core_sys::GUID,
    pub ParentLocation: GET_VIRTUAL_DISK_INFO_0_1,
    pub ParentIdentifier: ::windows_core_sys::GUID,
    pub ParentTimestamp: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ProviderSubtype: u32,
    pub Is4kAligned: ::win32_foundation_sys::BOOL,
    pub IsLoaded: ::win32_foundation_sys::BOOL,
    pub PhysicalDisk: GET_VIRTUAL_DISK_INFO_0_2,
    pub VhdPhysicalSectorSize: u32,
    pub SmallestSafeVirtualSize: u64,
    pub FragmentationPercentage: u32,
    pub VirtualDiskId: ::windows_core_sys::GUID,
    pub ChangeTrackingState: GET_VIRTUAL_DISK_INFO_0_0,
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO_0 {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO_0_0 {
    pub Enabled: ::win32_foundation_sys::BOOL,
    pub NewerChanges: ::win32_foundation_sys::BOOL,
    pub MostRecentId: [u16; 1],
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO_0_0 {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO_0_1 {
    pub ParentResolved: ::win32_foundation_sys::BOOL,
    pub ParentLocationBuffer: [u16; 1],
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO_0_1 {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO_0_2 {
    pub LogicalSectorSize: u32,
    pub PhysicalSectorSize: u32,
    pub IsRemote: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO_0_2 {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO_0_3 {
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub BlockSize: u32,
    pub SectorSize: u32,
}
impl ::core::marker::Copy for GET_VIRTUAL_DISK_INFO_0_3 {}
impl ::core::clone::Clone for GET_VIRTUAL_DISK_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GET_VIRTUAL_DISK_INFO_VERSION = i32;
pub const GET_VIRTUAL_DISK_INFO_UNSPECIFIED: GET_VIRTUAL_DISK_INFO_VERSION = 0i32;
pub const GET_VIRTUAL_DISK_INFO_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 1i32;
pub const GET_VIRTUAL_DISK_INFO_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = 2i32;
pub const GET_VIRTUAL_DISK_INFO_PARENT_LOCATION: GET_VIRTUAL_DISK_INFO_VERSION = 3i32;
pub const GET_VIRTUAL_DISK_INFO_PARENT_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = 4i32;
pub const GET_VIRTUAL_DISK_INFO_PARENT_TIMESTAMP: GET_VIRTUAL_DISK_INFO_VERSION = 5i32;
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_STORAGE_TYPE: GET_VIRTUAL_DISK_INFO_VERSION = 6i32;
pub const GET_VIRTUAL_DISK_INFO_PROVIDER_SUBTYPE: GET_VIRTUAL_DISK_INFO_VERSION = 7i32;
pub const GET_VIRTUAL_DISK_INFO_IS_4K_ALIGNED: GET_VIRTUAL_DISK_INFO_VERSION = 8i32;
pub const GET_VIRTUAL_DISK_INFO_PHYSICAL_DISK: GET_VIRTUAL_DISK_INFO_VERSION = 9i32;
pub const GET_VIRTUAL_DISK_INFO_VHD_PHYSICAL_SECTOR_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 10i32;
pub const GET_VIRTUAL_DISK_INFO_SMALLEST_SAFE_VIRTUAL_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 11i32;
pub const GET_VIRTUAL_DISK_INFO_FRAGMENTATION: GET_VIRTUAL_DISK_INFO_VERSION = 12i32;
pub const GET_VIRTUAL_DISK_INFO_IS_LOADED: GET_VIRTUAL_DISK_INFO_VERSION = 13i32;
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: GET_VIRTUAL_DISK_INFO_VERSION = 14i32;
pub const GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: GET_VIRTUAL_DISK_INFO_VERSION = 15i32;
pub const MERGE_VIRTUAL_DISK_DEFAULT_MERGE_DEPTH: u32 = 1u32;
pub type MERGE_VIRTUAL_DISK_FLAG = u32;
pub const MERGE_VIRTUAL_DISK_FLAG_NONE: MERGE_VIRTUAL_DISK_FLAG = 0u32;
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS {
    pub Version: MERGE_VIRTUAL_DISK_VERSION,
    pub Anonymous: MERGE_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for MERGE_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for MERGE_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MERGE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: MERGE_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl ::core::marker::Copy for MERGE_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MergeDepth: u32,
}
impl ::core::marker::Copy for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub MergeSourceDepth: u32,
    pub MergeTargetDepth: u32,
}
impl ::core::marker::Copy for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::core::clone::Clone for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MERGE_VIRTUAL_DISK_VERSION = i32;
pub const MERGE_VIRTUAL_DISK_VERSION_UNSPECIFIED: MERGE_VIRTUAL_DISK_VERSION = 0i32;
pub const MERGE_VIRTUAL_DISK_VERSION_1: MERGE_VIRTUAL_DISK_VERSION = 1i32;
pub const MERGE_VIRTUAL_DISK_VERSION_2: MERGE_VIRTUAL_DISK_VERSION = 2i32;
pub type MIRROR_VIRTUAL_DISK_FLAG = u32;
pub const MIRROR_VIRTUAL_DISK_FLAG_NONE: MIRROR_VIRTUAL_DISK_FLAG = 0u32;
pub const MIRROR_VIRTUAL_DISK_FLAG_EXISTING_FILE: MIRROR_VIRTUAL_DISK_FLAG = 1u32;
pub const MIRROR_VIRTUAL_DISK_FLAG_SKIP_MIRROR_ACTIVATION: MIRROR_VIRTUAL_DISK_FLAG = 2u32;
pub const MIRROR_VIRTUAL_DISK_FLAG_ENABLE_SMB_COMPRESSION: MIRROR_VIRTUAL_DISK_FLAG = 4u32;
pub const MIRROR_VIRTUAL_DISK_FLAG_IS_LIVE_MIGRATION: MIRROR_VIRTUAL_DISK_FLAG = 8u32;
#[repr(C)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS {
    pub Version: MIRROR_VIRTUAL_DISK_VERSION,
    pub Anonymous: MIRROR_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for MIRROR_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for MIRROR_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MIRROR_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MirrorVirtualDiskPath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MIRROR_VIRTUAL_DISK_VERSION = i32;
pub const MIRROR_VIRTUAL_DISK_VERSION_UNSPECIFIED: MIRROR_VIRTUAL_DISK_VERSION = 0i32;
pub const MIRROR_VIRTUAL_DISK_VERSION_1: MIRROR_VIRTUAL_DISK_VERSION = 1i32;
pub type MODIFY_VHDSET_FLAG = u32;
pub const MODIFY_VHDSET_FLAG_NONE: MODIFY_VHDSET_FLAG = 0u32;
pub const MODIFY_VHDSET_FLAG_WRITEABLE_SNAPSHOT: MODIFY_VHDSET_FLAG = 1u32;
#[repr(C)]
pub struct MODIFY_VHDSET_PARAMETERS {
    pub Version: MODIFY_VHDSET_VERSION,
    pub Anonymous: MODIFY_VHDSET_PARAMETERS_0,
}
impl ::core::marker::Copy for MODIFY_VHDSET_PARAMETERS {}
impl ::core::clone::Clone for MODIFY_VHDSET_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union MODIFY_VHDSET_PARAMETERS_0 {
    pub SnapshotPath: MODIFY_VHDSET_PARAMETERS_0_0,
    pub SnapshotId: ::windows_core_sys::GUID,
    pub DefaultFilePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for MODIFY_VHDSET_PARAMETERS_0 {}
impl ::core::clone::Clone for MODIFY_VHDSET_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MODIFY_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows_core_sys::GUID,
    pub SnapshotFilePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for MODIFY_VHDSET_PARAMETERS_0_0 {}
impl ::core::clone::Clone for MODIFY_VHDSET_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MODIFY_VHDSET_VERSION = i32;
pub const MODIFY_VHDSET_UNSPECIFIED: MODIFY_VHDSET_VERSION = 0i32;
pub const MODIFY_VHDSET_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = 1i32;
pub const MODIFY_VHDSET_REMOVE_SNAPSHOT: MODIFY_VHDSET_VERSION = 2i32;
pub const MODIFY_VHDSET_DEFAULT_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = 3i32;
pub type OPEN_VIRTUAL_DISK_FLAG = u32;
pub const OPEN_VIRTUAL_DISK_FLAG_NONE: OPEN_VIRTUAL_DISK_FLAG = 0u32;
pub const OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS: OPEN_VIRTUAL_DISK_FLAG = 1u32;
pub const OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE: OPEN_VIRTUAL_DISK_FLAG = 2u32;
pub const OPEN_VIRTUAL_DISK_FLAG_BOOT_DRIVE: OPEN_VIRTUAL_DISK_FLAG = 4u32;
pub const OPEN_VIRTUAL_DISK_FLAG_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = 8u32;
pub const OPEN_VIRTUAL_DISK_FLAG_CUSTOM_DIFF_CHAIN: OPEN_VIRTUAL_DISK_FLAG = 16u32;
pub const OPEN_VIRTUAL_DISK_FLAG_PARENT_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = 32u32;
pub const OPEN_VIRTUAL_DISK_FLAG_VHDSET_FILE_ONLY: OPEN_VIRTUAL_DISK_FLAG = 64u32;
pub const OPEN_VIRTUAL_DISK_FLAG_IGNORE_RELATIVE_PARENT_LOCATOR: OPEN_VIRTUAL_DISK_FLAG = 128u32;
pub const OPEN_VIRTUAL_DISK_FLAG_NO_WRITE_HARDENING: OPEN_VIRTUAL_DISK_FLAG = 256u32;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: OPEN_VIRTUAL_DISK_FLAG = 512u32;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: OPEN_VIRTUAL_DISK_FLAG = 1024u32;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: OPEN_VIRTUAL_DISK_FLAG = 2048u32;
#[repr(C)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS {
    pub Version: OPEN_VIRTUAL_DISK_VERSION,
    pub Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for OPEN_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for OPEN_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: OPEN_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: OPEN_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: OPEN_VIRTUAL_DISK_PARAMETERS_0_2,
}
impl ::core::marker::Copy for OPEN_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RWDepth: u32,
}
impl ::core::marker::Copy for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub GetInfoOnly: ::win32_foundation_sys::BOOL,
    pub ReadOnly: ::win32_foundation_sys::BOOL,
    pub ResiliencyGuid: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::core::clone::Clone for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub GetInfoOnly: ::win32_foundation_sys::BOOL,
    pub ReadOnly: ::win32_foundation_sys::BOOL,
    pub ResiliencyGuid: ::windows_core_sys::GUID,
    pub SnapshotId: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {}
impl ::core::clone::Clone for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OPEN_VIRTUAL_DISK_RW_DEPTH_DEFAULT: u32 = 1u32;
pub type OPEN_VIRTUAL_DISK_VERSION = i32;
pub const OPEN_VIRTUAL_DISK_VERSION_UNSPECIFIED: OPEN_VIRTUAL_DISK_VERSION = 0i32;
pub const OPEN_VIRTUAL_DISK_VERSION_1: OPEN_VIRTUAL_DISK_VERSION = 1i32;
pub const OPEN_VIRTUAL_DISK_VERSION_2: OPEN_VIRTUAL_DISK_VERSION = 2i32;
pub const OPEN_VIRTUAL_DISK_VERSION_3: OPEN_VIRTUAL_DISK_VERSION = 3i32;
pub type QUERY_CHANGES_VIRTUAL_DISK_FLAG = u32;
pub const QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE: QUERY_CHANGES_VIRTUAL_DISK_FLAG = 0u32;
#[repr(C)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    pub ByteOffset: u64,
    pub ByteLength: u64,
    pub Reserved: u64,
}
impl ::core::marker::Copy for QUERY_CHANGES_VIRTUAL_DISK_RANGE {}
impl ::core::clone::Clone for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RAW_SCSI_VIRTUAL_DISK_FLAG = u32;
pub const RAW_SCSI_VIRTUAL_DISK_FLAG_NONE: RAW_SCSI_VIRTUAL_DISK_FLAG = 0u32;
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RSVDHandle: ::win32_foundation_sys::BOOL,
    pub DataIn: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub SrbFlags: u32,
    pub DataTransferLength: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub SenseInfo: *mut u8,
    pub Cdb: *mut u8,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_RESPONSE {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataTransferLength: u32,
}
impl ::core::marker::Copy for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {}
impl ::core::clone::Clone for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RAW_SCSI_VIRTUAL_DISK_VERSION = i32;
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_UNSPECIFIED: RAW_SCSI_VIRTUAL_DISK_VERSION = 0i32;
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_1: RAW_SCSI_VIRTUAL_DISK_VERSION = 1i32;
pub type RESIZE_VIRTUAL_DISK_FLAG = u32;
pub const RESIZE_VIRTUAL_DISK_FLAG_NONE: RESIZE_VIRTUAL_DISK_FLAG = 0u32;
pub const RESIZE_VIRTUAL_DISK_FLAG_ALLOW_UNSAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = 1u32;
pub const RESIZE_VIRTUAL_DISK_FLAG_RESIZE_TO_SMALLEST_SAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = 2u32;
#[repr(C)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS {
    pub Version: RESIZE_VIRTUAL_DISK_VERSION,
    pub Anonymous: RESIZE_VIRTUAL_DISK_PARAMETERS_0,
}
impl ::core::marker::Copy for RESIZE_VIRTUAL_DISK_PARAMETERS {}
impl ::core::clone::Clone for RESIZE_VIRTUAL_DISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RESIZE_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl ::core::marker::Copy for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::core::clone::Clone for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl ::core::marker::Copy for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RESIZE_VIRTUAL_DISK_VERSION = i32;
pub const RESIZE_VIRTUAL_DISK_VERSION_UNSPECIFIED: RESIZE_VIRTUAL_DISK_VERSION = 0i32;
pub const RESIZE_VIRTUAL_DISK_VERSION_1: RESIZE_VIRTUAL_DISK_VERSION = 1i32;
#[repr(C)]
pub struct SET_VIRTUAL_DISK_INFO {
    pub Version: SET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: SET_VIRTUAL_DISK_INFO_0,
}
impl ::core::marker::Copy for SET_VIRTUAL_DISK_INFO {}
impl ::core::clone::Clone for SET_VIRTUAL_DISK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SET_VIRTUAL_DISK_INFO_0 {
    pub ParentFilePath: ::windows_core_sys::PCWSTR,
    pub UniqueIdentifier: ::windows_core_sys::GUID,
    pub ParentPathWithDepthInfo: SET_VIRTUAL_DISK_INFO_0_1,
    pub VhdPhysicalSectorSize: u32,
    pub VirtualDiskId: ::windows_core_sys::GUID,
    pub ChangeTrackingEnabled: ::win32_foundation_sys::BOOL,
    pub ParentLocator: SET_VIRTUAL_DISK_INFO_0_0,
}
impl ::core::marker::Copy for SET_VIRTUAL_DISK_INFO_0 {}
impl ::core::clone::Clone for SET_VIRTUAL_DISK_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SET_VIRTUAL_DISK_INFO_0_0 {
    pub LinkageId: ::windows_core_sys::GUID,
    pub ParentFilePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for SET_VIRTUAL_DISK_INFO_0_0 {}
impl ::core::clone::Clone for SET_VIRTUAL_DISK_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SET_VIRTUAL_DISK_INFO_0_1 {
    pub ChildDepth: u32,
    pub ParentFilePath: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for SET_VIRTUAL_DISK_INFO_0_1 {}
impl ::core::clone::Clone for SET_VIRTUAL_DISK_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SET_VIRTUAL_DISK_INFO_VERSION = i32;
pub const SET_VIRTUAL_DISK_INFO_UNSPECIFIED: SET_VIRTUAL_DISK_INFO_VERSION = 0i32;
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH: SET_VIRTUAL_DISK_INFO_VERSION = 1i32;
pub const SET_VIRTUAL_DISK_INFO_IDENTIFIER: SET_VIRTUAL_DISK_INFO_VERSION = 2i32;
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH_WITH_DEPTH: SET_VIRTUAL_DISK_INFO_VERSION = 3i32;
pub const SET_VIRTUAL_DISK_INFO_PHYSICAL_SECTOR_SIZE: SET_VIRTUAL_DISK_INFO_VERSION = 4i32;
pub const SET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: SET_VIRTUAL_DISK_INFO_VERSION = 5i32;
pub const SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: SET_VIRTUAL_DISK_INFO_VERSION = 6i32;
pub const SET_VIRTUAL_DISK_INFO_PARENT_LOCATOR: SET_VIRTUAL_DISK_INFO_VERSION = 7i32;
#[repr(C)]
pub struct STORAGE_DEPENDENCY_INFO {
    pub Version: STORAGE_DEPENDENCY_INFO_VERSION,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_DEPENDENCY_INFO_0,
}
impl ::core::marker::Copy for STORAGE_DEPENDENCY_INFO {}
impl ::core::clone::Clone for STORAGE_DEPENDENCY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union STORAGE_DEPENDENCY_INFO_0 {
    pub Version1Entries: [STORAGE_DEPENDENCY_INFO_TYPE_1; 1],
    pub Version2Entries: [STORAGE_DEPENDENCY_INFO_TYPE_2; 1],
}
impl ::core::marker::Copy for STORAGE_DEPENDENCY_INFO_0 {}
impl ::core::clone::Clone for STORAGE_DEPENDENCY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_1 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
}
impl ::core::marker::Copy for STORAGE_DEPENDENCY_INFO_TYPE_1 {}
impl ::core::clone::Clone for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_2 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub DependencyDeviceName: ::windows_core_sys::PWSTR,
    pub HostVolumeName: ::windows_core_sys::PWSTR,
    pub DependentVolumeName: ::windows_core_sys::PWSTR,
    pub DependentVolumeRelativePath: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for STORAGE_DEPENDENCY_INFO_TYPE_2 {}
impl ::core::clone::Clone for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STORAGE_DEPENDENCY_INFO_VERSION = i32;
pub const STORAGE_DEPENDENCY_INFO_VERSION_UNSPECIFIED: STORAGE_DEPENDENCY_INFO_VERSION = 0i32;
pub const STORAGE_DEPENDENCY_INFO_VERSION_1: STORAGE_DEPENDENCY_INFO_VERSION = 1i32;
pub const STORAGE_DEPENDENCY_INFO_VERSION_2: STORAGE_DEPENDENCY_INFO_VERSION = 2i32;
pub type TAKE_SNAPSHOT_VHDSET_FLAG = u32;
pub const TAKE_SNAPSHOT_VHDSET_FLAG_NONE: TAKE_SNAPSHOT_VHDSET_FLAG = 0u32;
pub const TAKE_SNAPSHOT_VHDSET_FLAG_WRITEABLE: TAKE_SNAPSHOT_VHDSET_FLAG = 1u32;
#[repr(C)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: TAKE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl ::core::marker::Copy for TAKE_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::core::clone::Clone for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl ::core::marker::Copy for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::core::clone::Clone for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::core::clone::Clone for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TAKE_SNAPSHOT_VHDSET_VERSION = i32;
pub const TAKE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: TAKE_SNAPSHOT_VHDSET_VERSION = 0i32;
pub const TAKE_SNAPSHOT_VHDSET_VERSION_1: TAKE_SNAPSHOT_VHDSET_VERSION = 1i32;
pub type VIRTUAL_DISK_ACCESS_MASK = u32;
pub const VIRTUAL_DISK_ACCESS_NONE: VIRTUAL_DISK_ACCESS_MASK = 0u32;
pub const VIRTUAL_DISK_ACCESS_ATTACH_RO: VIRTUAL_DISK_ACCESS_MASK = 65536u32;
pub const VIRTUAL_DISK_ACCESS_ATTACH_RW: VIRTUAL_DISK_ACCESS_MASK = 131072u32;
pub const VIRTUAL_DISK_ACCESS_DETACH: VIRTUAL_DISK_ACCESS_MASK = 262144u32;
pub const VIRTUAL_DISK_ACCESS_GET_INFO: VIRTUAL_DISK_ACCESS_MASK = 524288u32;
pub const VIRTUAL_DISK_ACCESS_CREATE: VIRTUAL_DISK_ACCESS_MASK = 1048576u32;
pub const VIRTUAL_DISK_ACCESS_METAOPS: VIRTUAL_DISK_ACCESS_MASK = 2097152u32;
pub const VIRTUAL_DISK_ACCESS_READ: VIRTUAL_DISK_ACCESS_MASK = 851968u32;
pub const VIRTUAL_DISK_ACCESS_ALL: VIRTUAL_DISK_ACCESS_MASK = 4128768u32;
pub const VIRTUAL_DISK_ACCESS_WRITABLE: VIRTUAL_DISK_ACCESS_MASK = 3276800u32;
pub const VIRTUAL_DISK_MAXIMUM_CHANGE_TRACKING_ID_LENGTH: u32 = 256u32;
#[repr(C)]
pub struct VIRTUAL_DISK_PROGRESS {
    pub OperationStatus: u32,
    pub CurrentValue: u64,
    pub CompletionValue: u64,
}
impl ::core::marker::Copy for VIRTUAL_DISK_PROGRESS {}
impl ::core::clone::Clone for VIRTUAL_DISK_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_STORAGE_TYPE {
    pub DeviceId: u32,
    pub VendorId: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for VIRTUAL_STORAGE_TYPE {}
impl ::core::clone::Clone for VIRTUAL_STORAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VIRTUAL_STORAGE_TYPE_DEVICE_ISO: u32 = 1u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_UNKNOWN: u32 = 0u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHD: u32 = 2u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDSET: u32 = 4u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDX: u32 = 3u32;
pub const VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3969403628, data2: 41209, data3: 18409, data4: [144, 31, 113, 65, 90, 102, 52, 91] };
pub const VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
