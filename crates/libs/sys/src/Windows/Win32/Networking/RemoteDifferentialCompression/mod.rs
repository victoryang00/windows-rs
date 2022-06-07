#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl ::core::marker::Copy for FindSimilarFileIndexResults {}
impl ::core::clone::Clone for FindSimilarFileIndexResults {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FindSimilarResults: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903443, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type GeneratorParametersType = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_Unused: GeneratorParametersType = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = 1i32;
#[repr(C)]
pub struct IFindSimilarResults {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetNextFileId: unsafe extern "system" fn(this: *mut *mut Self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFindSimilarResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903425, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcComparator {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
}
impl ::windows_sys::core::Interface for IRdcComparator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903415, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcFileReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, filesize: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut *mut Self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Read: usize,
    pub GetFilePosition: unsafe extern "system" fn(this: *mut *mut Self, offsetfromstart: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcFileReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903412, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcFileWriter {
    pub base__: IRdcFileReader,
    pub Write: unsafe extern "system" fn(this: *mut *mut Self, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Truncate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DeleteOnClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcFileWriter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903413, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcGenerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetGeneratorParameters: unsafe extern "system" fn(this: *mut *mut Self, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
}
impl ::windows_sys::core::Interface for IRdcGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903411, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcGeneratorFilterMaxParameters {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetHorizonSize: unsafe extern "system" fn(this: *mut *mut Self, horizonsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHorizonSize: unsafe extern "system" fn(this: *mut *mut Self, horizonsize: u32) -> ::windows_sys::core::HRESULT,
    pub GetHashWindowSize: unsafe extern "system" fn(this: *mut *mut Self, hashwindowsize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHashWindowSize: unsafe extern "system" fn(this: *mut *mut Self, hashwindowsize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcGeneratorFilterMaxParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903410, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcGeneratorParameters {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetGeneratorParametersType: unsafe extern "system" fn(this: *mut *mut Self, parameterstype: *mut GeneratorParametersType) -> ::windows_sys::core::HRESULT,
    pub GetParametersVersion: unsafe extern "system" fn(this: *mut *mut Self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSerializeSize: unsafe extern "system" fn(this: *mut *mut Self, size: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcGeneratorParameters {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903409, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcLibrary {
    pub base__: ::windows_sys::core::IUnknown,
    pub ComputeDefaultRecursionDepth: unsafe extern "system" fn(this: *mut *mut Self, filesize: u64, depth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CreateGeneratorParameters: unsafe extern "system" fn(this: *mut *mut Self, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenGeneratorParameters: unsafe extern "system" fn(this: *mut *mut Self, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGenerator: unsafe extern "system" fn(this: *mut *mut Self, depth: u32, igeneratorparametersarray: *const *mut ::core::ffi::c_void, igenerator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateComparator: unsafe extern "system" fn(this: *mut *mut Self, iseedsignaturesfile: *mut ::core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSignatureReader: unsafe extern "system" fn(this: *mut *mut Self, ifilereader: *mut ::core::ffi::c_void, isignaturereader: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRDCVersion: unsafe extern "system" fn(this: *mut *mut Self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcLibrary {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903416, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcSignatureReader {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReadHeader: unsafe extern "system" fn(this: *mut *mut Self, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadSignatures: unsafe extern "system" fn(this: *mut *mut Self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadSignatures: usize,
}
impl ::windows_sys::core::Interface for IRdcSignatureReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903414, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct IRdcSimilarityGenerator {
    pub base__: ::windows_sys::core::IUnknown,
    pub EnableSimilarity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Results: unsafe extern "system" fn(this: *mut *mut Self, similaritydata: *mut SimilarityData) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRdcSimilarityGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903424, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarity {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut *mut Self, mapping: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut *mut Self, isvalid: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows_sys::core::HRESULT,
    pub FindSimilarFileId: unsafe extern "system" fn(this: *mut *mut Self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyAndSwap: unsafe extern "system" fn(this: *mut *mut Self, newsimilaritytables: *mut ::core::ffi::c_void, reportprogress: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut *mut Self, recordcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimilarity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903427, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityFileIdTable {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut *mut Self, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut *mut Self, isvalid: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows_sys::core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(this: *mut *mut Self, similarityfileindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut *mut Self, recordcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimilarityFileIdTable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903423, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityReportProgress {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReportProgress: unsafe extern "system" fn(this: *mut *mut Self, percentcompleted: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimilarityReportProgress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903418, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityTableDumpState {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextData: unsafe extern "system" fn(this: *mut *mut Self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextData: usize,
}
impl ::windows_sys::core::Interface for ISimilarityTableDumpState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903419, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityTraitsMappedView {
    pub base__: ::windows_sys::core::IUnknown,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Get: unsafe extern "system" fn(this: *mut *mut Self, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Get: usize,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8),
}
impl ::windows_sys::core::Interface for ISimilarityTraitsMappedView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903420, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityTraitsMapping {
    pub base__: ::windows_sys::core::IUnknown,
    pub CloseMapping: unsafe extern "system" fn(this: *mut *mut Self),
    pub SetFileSize: unsafe extern "system" fn(this: *mut *mut Self, filesize: u64) -> ::windows_sys::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut *mut Self, filesize: *mut u64) -> ::windows_sys::core::HRESULT,
    pub OpenMapping: unsafe extern "system" fn(this: *mut *mut Self, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ResizeMapping: unsafe extern "system" fn(this: *mut *mut Self, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetPageSize: unsafe extern "system" fn(this: *mut *mut Self, pagesize: *mut u32),
    pub CreateView: unsafe extern "system" fn(this: *mut *mut Self, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimilarityTraitsMapping {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903421, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[repr(C)]
pub struct ISimilarityTraitsTable {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut *mut Self, mapping: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut *mut Self, isvalid: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, data: *const SimilarityData, fileindex: u32) -> ::windows_sys::core::HRESULT,
    pub FindSimilarFileIndex: unsafe extern "system" fn(this: *mut *mut Self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BeginDump: unsafe extern "system" fn(this: *mut *mut Self, similaritytabledumpstate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLastIndex: unsafe extern "system" fn(this: *mut *mut Self, fileindex: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISimilarityTraitsTable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903422, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RDC_ErrorCode = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_NoError: RDC_ErrorCode = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = 3i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderWrongType: RDC_ErrorCode = 4i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = 5i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataTooManyRecords: RDC_ErrorCode = 6i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = 7i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_ApplicationError: RDC_ErrorCode = 8i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Aborted: RDC_ErrorCode = 9i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Win32Error: RDC_ErrorCode = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl ::core::marker::Copy for RdcBufferPointer {}
impl ::core::clone::Clone for RdcBufferPointer {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RdcComparator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903435, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcCreatedTables = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_Existing: RdcCreatedTables = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_New: RdcCreatedTables = 2i32;
pub const RdcFileReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903433, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const RdcGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903432, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const RdcGeneratorFilterMaxParameters: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903431, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const RdcGeneratorParameters: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903430, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const RdcLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903429, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcMappingAccessMode = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl ::core::marker::Copy for RdcNeed {}
impl ::core::clone::Clone for RdcNeed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl ::core::marker::Copy for RdcNeedPointer {}
impl ::core::clone::Clone for RdcNeedPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcNeedType = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SOURCE: RdcNeedType = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_TARGET: RdcNeedType = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED: RdcNeedType = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED_MAX: RdcNeedType = 255i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl ::core::marker::Copy for RdcSignature {}
impl ::core::clone::Clone for RdcSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl ::core::marker::Copy for RdcSignaturePointer {}
impl ::core::clone::Clone for RdcSignaturePointer {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RdcSignatureReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903434, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const RdcSimilarityGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903442, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const Similarity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903441, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl ::core::marker::Copy for SimilarityData {}
impl ::core::clone::Clone for SimilarityData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl ::core::marker::Copy for SimilarityDumpData {}
impl ::core::clone::Clone for SimilarityDumpData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl ::core::marker::Copy for SimilarityFileId {}
impl ::core::clone::Clone for SimilarityFileId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMaxSize: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub const SimilarityFileIdTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903440, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl ::core::marker::Copy for SimilarityMappedViewInfo {}
impl ::core::clone::Clone for SimilarityMappedViewInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SimilarityReportProgress: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903437, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const SimilarityTableDumpState: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903438, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const SimilarityTraitsMappedView: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903445, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const SimilarityTraitsMapping: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903444, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
pub const SimilarityTraitsTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518903439, data2: 40380, data3: 4570, data4: [158, 63, 0, 17, 17, 74, 227, 17] };
