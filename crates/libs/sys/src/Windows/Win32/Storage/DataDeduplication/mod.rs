#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl ::core::marker::Copy for DDP_FILE_EXTENT {}
impl ::core::clone::Clone for DDP_FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DEDUP_BACKUP_SUPPORT_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl ::core::marker::Copy for DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::clone::Clone for DEDUP_CHUNK_INFO_HASH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for DEDUP_CONTAINER_EXTENT {}
impl ::core::clone::Clone for DEDUP_CONTAINER_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DEDUP_SET_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = 5i32;
pub const DedupBackupSupport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1943450285, data2: 10628, data3: 18197, data4: [178, 227, 146, 76, 20, 151, 68, 221] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl ::core::marker::Copy for DedupChunk {}
impl ::core::clone::Clone for DedupChunk {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupChunkFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_None: DedupChunkFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_Compressed: DedupChunkFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupChunkingAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupCompressionAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = 1i32;
pub const DedupDataPort: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400219655, data2: 6185, data3: 18610, data4: [166, 75, 230, 31, 142, 13, 154, 203] };
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortManagerOption = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortRequestStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = 5i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortVolumeStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = 5i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl ::core::marker::Copy for DedupHash {}
impl ::core::clone::Clone for DedupHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupHashingAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DedupStream {
    pub Path: super::super::Foundation::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DedupStream {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DedupStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl ::core::marker::Copy for DedupStreamEntry {}
impl ::core::clone::Clone for DedupStreamEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDedupBackupSupport {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreFiles: unsafe extern "system" fn(this: *mut *mut Self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: *mut ::core::ffi::c_void, flags: u32, fileresults: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreFiles: usize,
}
#[repr(C)]
pub struct IDedupChunkLibrary {
    pub base__: ::windows_sys::core::IUnknown,
    pub InitializeForPushBuffers: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetParameter: unsafe extern "system" fn(this: *mut *mut Self, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetParameter: usize,
    pub StartChunking: unsafe extern "system" fn(this: *mut *mut Self, iiditeratorinterfaceid: ::windows_sys::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDedupDataPort {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LookupChunks: unsafe extern "system" fn(this: *mut *mut Self, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub InsertChunks: unsafe extern "system" fn(this: *mut *mut Self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertChunksWithStream: unsafe extern "system" fn(this: *mut *mut Self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut ::core::ffi::c_void, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertChunksWithStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CommitStreams: unsafe extern "system" fn(this: *mut *mut Self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CommitStreams: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CommitStreamsWithStream: unsafe extern "system" fn(this: *mut *mut Self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut ::core::ffi::c_void, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CommitStreamsWithStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStreams: unsafe extern "system" fn(this: *mut *mut Self, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStreams: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStreamsResults: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStreamsResults: usize,
    pub GetChunks: unsafe extern "system" fn(this: *mut *mut Self, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetChunksResults: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetRequestStatus: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows_sys::core::HRESULT,
    pub GetRequestResults: unsafe extern "system" fn(this: *mut *mut Self, requestid: ::windows_sys::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows_sys::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDedupDataPortManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeStatus: unsafe extern "system" fn(this: *mut *mut Self, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeDataPort: unsafe extern "system" fn(this: *mut *mut Self, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeDataPort: usize,
}
#[repr(C)]
pub struct IDedupIterateChunksHash32 {
    pub base__: ::windows_sys::core::IUnknown,
    pub PushBuffer: unsafe extern "system" fn(this: *mut *mut Self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Drain: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDedupReadFileCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadBackupFile: unsafe extern "system" fn(this: *mut *mut Self, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadBackupFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OrderContainersRestore: unsafe extern "system" fn(this: *mut *mut Self, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OrderContainersRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreviewContainerRead: unsafe extern "system" fn(this: *mut *mut Self, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreviewContainerRead: usize,
}
