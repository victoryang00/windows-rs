#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
    pub fn DMOEnum(guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut *mut *mut IEnumDMO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
    pub fn DMOGetName(clsiddmo: *const ::windows_sys::core::GUID, szname: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
    pub fn DMOGetTypes(clsiddmo: *const ::windows_sys::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
    pub fn DMORegister(szname: ::windows_sys::core::PCWSTR, clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
    pub fn DMOUnregister(clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
}
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3214294400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AGC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3901528992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133857978, data2: 15881, data3: 18720, data4: [170, 95, 33, 152, 17, 20, 143, 9] };
pub const DMOCATEGORY_AUDIO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1475533707, data2: 59067, data3: 17683, data4: [157, 67, 220, 210, 166, 89, 49, 37] };
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4083166015, data2: 1426, data3: 18655, data4: [164, 205, 103, 71, 33, 231, 235, 235] };
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 869902177, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3766456383, data2: 25341, data3: 20064, data4: [140, 221, 222, 167, 35, 102, 101, 181] };
pub const DMOCATEGORY_VIDEO_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1248441410, data2: 10430, data3: 18833, data4: [150, 156, 181, 0, 173, 245, 216, 168] };
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3650154004, data2: 30572, data3: 18211, data4: [190, 70, 61, 162, 245, 111, 16, 185] };
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 869902176, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type DMO_ENUM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_INVALIDSTREAMINDEX: ::windows_sys::core::HRESULT = -2147220991i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_INVALIDTYPE: ::windows_sys::core::HRESULT = -2147220990i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_NOTACCEPTING: ::windows_sys::core::HRESULT = -2147220988i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = -2147220986i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows_sys::core::HRESULT = -2147220987i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_TYPE_NOT_SET: ::windows_sys::core::HRESULT = -2147220989i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DMO_MEDIA_TYPE {
    pub majortype: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_sys::core::GUID,
    pub pUnk: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: *mut *mut *mut *mut IMediaBuffer,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl ::core::marker::Copy for DMO_OUTPUT_DATA_BUFFER {}
impl ::core::clone::Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows_sys::core::GUID,
    pub subtype: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DMO_PARTIAL_MEDIATYPE {}
impl ::core::clone::Clone for DMO_PARTIAL_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type DMO_REGISTER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = 1i32;
#[repr(C)]
pub struct IDMOQualityControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetNow: unsafe extern "system" fn(this: *mut *mut Self, rtnow: i64) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDMOQualityControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1705765526, data2: 53046, data3: 17727, data4: [175, 138, 112, 94, 152, 241, 98, 96] };
}
#[repr(C)]
pub struct IDMOVideoOutputOptimizations {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueryOperationModePreferences: unsafe extern "system" fn(this: *mut *mut Self, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(this: *mut *mut Self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(this: *mut *mut Self, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(this: *mut *mut Self, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDMOVideoOutputOptimizations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3197062990, data2: 23318, data3: 19753, data4: [179, 80, 127, 107, 93, 146, 152, 172] };
}
#[repr(C)]
pub struct IEnumDMO {
    pub base__: ::windows_sys::core::IUnknown,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, citemstofetch: u32, pclsid: *mut ::windows_sys::core::GUID, names: *mut ::windows_sys::core::PWSTR, pcitemsfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, citemstoskip: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnumDMO {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 742185354, data2: 11258, data3: 19027, data4: [156, 39, 82, 73, 186, 100, 186, 15] };
}
#[repr(C)]
pub struct IMediaBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, cblength: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, pcbmaxlength: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut *mut Self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1508899001, data2: 37772, data3: 18982, data4: [130, 242, 149, 203, 132, 205, 200, 55] };
}
#[repr(C)]
pub struct IMediaObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut *mut Self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(this: *mut *mut Self, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputType: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputType: unsafe extern "system" fn(this: *mut *mut Self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInputType: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputType: unsafe extern "system" fn(this: *mut *mut Self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputCurrentType: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputCurrentType: unsafe extern "system" fn(this: *mut *mut Self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputCurrentType: usize,
    pub GetInputSizeInfo: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputSizeInfo: unsafe extern "system" fn(this: *mut *mut Self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputMaxLatency: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetInputMaxLatency: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32) -> ::windows_sys::core::HRESULT,
    pub AllocateStreamingResources: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FreeStreamingResources: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetInputStatus: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ProcessInput: unsafe extern "system" fn(this: *mut *mut Self, dwinputstreamindex: u32, pbuffer: *mut ::core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows_sys::core::HRESULT,
    pub ProcessOutput: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut *mut Self, block: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635220312, data2: 21652, data3: 16642, data4: [151, 197, 236, 121, 142, 89, 188, 244] };
}
#[repr(C)]
pub struct IMediaObjectInPlace {
    pub base__: ::windows_sys::core::IUnknown,
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut *mut Self, platencytime: *mut i64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaObjectInPlace {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1696307920, data2: 4039, data3: 19113, data4: [149, 56, 216, 153, 49, 1, 7, 65] };
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_INPLACE_PROCESS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_INPUT_DATA_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_INPUT_STATUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_INPUT_STREAM_INFO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_OUTPUT_DATA_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_OUTPUT_STREAM_INFO_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_PROCESS_OUTPUT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_QUALITY_STATUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_SET_TYPE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub type _DMO_VIDEO_OUTPUT_STREAM_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = 1i32;
