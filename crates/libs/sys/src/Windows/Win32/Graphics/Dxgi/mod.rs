#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub fn CreateDXGIFactory(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub fn CreateDXGIFactory1(riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub fn CreateDXGIFactory2(flags: u32, riid: *const ::windows_sys::core::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub fn DXGIDeclareAdapterRemovalSupport() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    pub fn DXGIGetDebugInterface1(flags: u32, riid: *const ::windows_sys::core::GUID, pdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_ADAPTER_DESC3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_ADAPTER_FLAG = u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_ADAPTER_FLAG3 = u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_COMPUTE_PREEMPTION_GRANULARITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1u32;
pub const DXGI_DEBUG_ALL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3834307203, data2: 55936, data3: 18699, data4: [135, 230, 67, 233, 169, 207, 218, 8] };
pub const DXGI_DEBUG_APP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 114126337, data2: 16921, data3: 20157, data4: [135, 9, 39, 237, 35, 54, 12, 98] };
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 902682620, data2: 5042, data3: 16925, data4: [165, 215, 126, 68, 81, 40, 125, 100] };
pub const DXGI_DEBUG_DXGI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 634247844, data2: 45510, data3: 18401, data4: [172, 62, 152, 135, 91, 90, 46, 42] };
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_DEBUG_RLO_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
impl ::core::marker::Copy for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::clone::Clone for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl ::core::marker::Copy for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::clone::Clone for DXGI_DISPLAY_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_SCALING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ENUM_MODES_STEREO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_DENIED: ::windows_sys::core::HRESULT = -2005270485i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ACCESS_LOST: ::windows_sys::core::HRESULT = -2005270490i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2005270474i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_CORRUPT: ::windows_sys::core::HRESULT = -2005270477i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_FULL: ::windows_sys::core::HRESULT = -2005270476i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CACHE_HASH_COLLISION: ::windows_sys::core::HRESULT = -2005270475i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: ::windows_sys::core::HRESULT = -2005270486i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_HUNG: ::windows_sys::core::HRESULT = -2005270522i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_REMOVED: ::windows_sys::core::HRESULT = -2005270523i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DEVICE_RESET: ::windows_sys::core::HRESULT = -2005270521i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -2005270496i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: ::windows_sys::core::HRESULT = -2005270479i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: ::windows_sys::core::HRESULT = -2005270517i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: ::windows_sys::core::HRESULT = -2005270516i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: ::windows_sys::core::HRESULT = -2005270480i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_INVALID_CALL: ::windows_sys::core::HRESULT = -2005270527i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: ::windows_sys::core::HRESULT = -2005270491i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_MORE_DATA: ::windows_sys::core::HRESULT = -2005270525i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2005270484i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NONEXCLUSIVE: ::windows_sys::core::HRESULT = -2005270495i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NON_COMPOSITED_UI: ::windows_sys::core::HRESULT = -2005270478i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENT: ::windows_sys::core::HRESULT = -2005270482i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: ::windows_sys::core::HRESULT = -2005270494i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_NOT_FOUND: ::windows_sys::core::HRESULT = -2005270526i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: ::windows_sys::core::HRESULT = -2005270493i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: ::windows_sys::core::HRESULT = -2005270492i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: ::windows_sys::core::HRESULT = -2005270487i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: ::windows_sys::core::HRESULT = -2005270483i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_SESSION_DISCONNECTED: ::windows_sys::core::HRESULT = -2005270488i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_UNSUPPORTED: ::windows_sys::core::HRESULT = -2005270524i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAIT_TIMEOUT: ::windows_sys::core::HRESULT = -2005270489i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_ERROR_WAS_STILL_DRAWING: ::windows_sys::core::HRESULT = -2005270518i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_FEATURE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_FRAME_PRESENTATION_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
impl ::core::marker::Copy for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::clone::Clone for DXGI_FRAME_STATISTICS_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_GPU_PREFERENCE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_GRAPHICS_PREEMPTION_GRANULARITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10 {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl ::core::marker::Copy for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::clone::Clone for DXGI_HDR_METADATA_HDR10PLUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_HDR_METADATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: ::windows_sys::core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::clone::Clone for DXGI_INFO_QUEUE_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_INFO_QUEUE_MESSAGE_CATEGORY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_INFO_QUEUE_MESSAGE_SEVERITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl ::core::marker::Copy for DXGI_MAPPED_RECT {}
impl ::core::clone::Clone for DXGI_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_DISCARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAP_WRITE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
impl ::core::marker::Copy for DXGI_MATRIX_3X2_F {}
impl ::core::clone::Clone for DXGI_MATRIX_3X2_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_MEMORY_SEGMENT_GROUP = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_MODE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MWA_VALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_Message_Id = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id = 17i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id = 18i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id = 19i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id = 20i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id = 21i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id = 22i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id = 23i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = 24i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id = 25i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id = 26i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id = 27i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id = 28i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id = 29i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id = 30i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id = 31i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id = 33i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id = 34i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id = 35i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id = 36i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id = 37i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = 38i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = 39i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = 40i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id = 41i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = 42i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id = 43i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id = 44i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id = 45i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: DXGI_Message_Id = 46i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id = 47i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id = 48i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id = 49i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id = 50i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id = 51i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id = 52i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = 53i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id = 54i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id = 55i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id = 56i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: DXGI_Message_Id = 57i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id = 58i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id = 59i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id = 60i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id = 61i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id = 62i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id = 63i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id = 65i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id = 66i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id = 67i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id = 68i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = 69i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: DXGI_Message_Id = 70i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: DXGI_Message_Id = 71i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: DXGI_Message_Id = 72i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: DXGI_Message_Id = 73i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id = 74i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: DXGI_Message_Id = 75i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id = 76i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: DXGI_Message_Id = 77i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id = 78i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id = 79i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: DXGI_Message_Id = 80i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id = 81i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: DXGI_Message_Id = 82i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = 83i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = 84i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = 85i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = 86i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = 87i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id = 88i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id = 89i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = 90i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = 91i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = 92i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id = 93i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id = 94i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id = 95i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id = 96i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id = 97i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id = 98i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id = 99i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id = 100i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id = 101i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: DXGI_Message_Id = 102i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id = 103i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id = 104i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: DXGI_Message_Id = 105i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id = 106i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id = 107i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id = 108i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id = 109i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id = 110i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: DXGI_Message_Id = 111i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = 112i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id = 113i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id = 114i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: DXGI_Message_Id = 115i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id = 116i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: DXGI_Message_Id = 117i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: DXGI_Message_Id = 118i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: DXGI_Message_Id = 119i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: DXGI_Message_Id = 120i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id = 121i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id = 122i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id = 123i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id = 124i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id = 125i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id = 126i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id = 127i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id = 129i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: DXGI_Message_Id = 130i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: DXGI_Message_Id = 131i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id = 132i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: DXGI_Message_Id = 133i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id = 134i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: DXGI_Message_Id = 135i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: DXGI_Message_Id = 136i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: DXGI_Message_Id = 137i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: DXGI_Message_Id = 138i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id = 139i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id = 140i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id = 141i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id = 142i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = 143i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id = 144i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id = 145i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: DXGI_Message_Id = 146i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id = 147i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id = 148i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = 149i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id = 150i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id = 151i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id = 152i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id = 153i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id = 154i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id = 155i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id = 156i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id = 157i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id = 158i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = 159i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id = 160i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id = 161i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id = 162i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id = 163i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: DXGI_Message_Id = 164i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id = 165i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id = 166i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id = 167i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = 168i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id = 169i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id = 170i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = 171i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id = 172i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: DXGI_Message_Id = 173i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = 174i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: DXGI_Message_Id = 175i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id = 176i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: DXGI_Message_Id = 177i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id = 178i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id = 179i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id = 180i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: DXGI_Message_Id = 181i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id = 182i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id = 183i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id = 184i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id = 185i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id = 186i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id = 187i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id = 188i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id = 189i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id = 190i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id = 191i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id = 192i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id = 193i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id = 194i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id = 195i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id = 196i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id = 197i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id = 198i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id = 199i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id = 200i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id = 201i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id = 202i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id = 203i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id = 204i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id = 205i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id = 206i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id = 207i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = 208i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = 209i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: DXGI_Message_Id = 210i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: DXGI_Message_Id = 211i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id = 212i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id = 213i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id = 214i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = 215i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id = 216i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = 217i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id = 218i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id = 219i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: DXGI_Message_Id = 220i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id = 221i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id = 222i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id = 223i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = 224i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = 225i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id = 226i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: DXGI_Message_Id = 227i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = 228i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = 229i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = 230i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = 231i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = 232i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = 233i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id = 234i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = 235i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id = 236i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id = 237i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = 238i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id = 239i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id = 240i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id = 241i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id = 242i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id = 243i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id = 244i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id = 245i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = 246i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = 247i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = 248i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = 249i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = 250i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id = 251i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id = 252i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id = 253i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id = 254i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id = 255i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id = 257i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: DXGI_Message_Id = 258i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id = 259i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = 260i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id = 261i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id = 262i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id = 263i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id = 264i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id = 265i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id = 266i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id = 267i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id = 268i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id = 269i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id = 270i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id = 271i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id = 272i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id = 273i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id = 274i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = 275i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id = 276i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id = 277i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id = 278i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id = 279i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id = 280i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id = 281i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id = 282i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id = 283i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id = 284i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id = 285i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id = 286i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id = 287i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id = 288i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id = 289i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id = 290i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id = 291i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id = 292i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id = 293i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id = 294i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id = 295i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id = 296i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: DXGI_Message_Id = 297i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id = 298i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id = 1000i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id = 1001i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id = 1002i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id = 1003i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id = 1004i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id = 1005i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id = 1006i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id = 1007i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id = 1008i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id = 1009i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id = 1010i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id = 1011i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id = 1012i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id = 1013i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id = 1014i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id = 1015i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id = 1016i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id = 1017i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id = 1018i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id = 1019i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id = 1020i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id = 1021i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: DXGI_Message_Id = 1022i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id = 1023i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: DXGI_Message_Id = 1025i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: DXGI_Message_Id = 1026i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id = 1027i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id = 1028i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id = 1029i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = 1030i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = 1031i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OFFER_RESOURCE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OFFER_RESOURCE_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_OUTDUPL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OUTDUPL_FLAG = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::Foundation::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_FRAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_MOVE_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OUTDUPL_POINTER_SHAPE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: super::super::Foundation::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DXGI_OUTPUT_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_OVERLAY_SUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_TEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_PRESENT_USE_DURATION: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
impl ::core::marker::Copy for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::clone::Clone for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_RECLAIM_RESOURCE_RESULTS = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_RESIDENCY = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_RESOURCE_PRIORITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DXGI_RESOURCE_PRIORITY = 671088640u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_LOW: DXGI_RESOURCE_PRIORITY = 1342177280u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DXGI_RESOURCE_PRIORITY = 2013265920u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_HIGH: DXGI_RESOURCE_PRIORITY = 2684354560u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DXGI_RESOURCE_PRIORITY = 3355443200u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DXGI_RGBA {}
impl ::core::clone::Clone for DXGI_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_SCALING = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_SHARED_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DXGI_SURFACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: super::super::Foundation::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: super::super::Foundation::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_SWAP_CHAIN_FLAG = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub type DXGI_SWAP_EFFECT = i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_READ_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_SHARED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024u32;
#[repr(C)]
pub struct IDXGIAdapter {
    pub base__: IDXGIObject,
    pub EnumOutputs: unsafe extern "system" fn(this: *mut *mut Self, output: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(this: *mut *mut Self, interfacename: *const ::windows_sys::core::GUID, pumdversion: *mut i64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIAdapter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 605153249, data2: 4780, data3: 19663, data4: [189, 20, 151, 152, 232, 83, 77, 192] };
}
#[repr(C)]
pub struct IDXGIAdapter1 {
    pub base__: IDXGIAdapter,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc1: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc1: usize,
}
impl ::windows_sys::core::Interface for IDXGIAdapter1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 688099169, data2: 14393, data3: 17958, data4: [145, 253, 8, 104, 121, 1, 26, 5] };
}
#[repr(C)]
pub struct IDXGIAdapter2 {
    pub base__: IDXGIAdapter1,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc2: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc2: usize,
}
impl ::windows_sys::core::Interface for IDXGIAdapter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 178368010, data2: 64014, data3: 19332, data4: [134, 68, 224, 95, 248, 229, 172, 181] };
}
#[repr(C)]
pub struct IDXGIAdapter3 {
    pub base__: IDXGIAdapter2,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterHardwareContentProtectionTeardownStatusEvent: usize,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(this: *mut *mut Self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_sys::core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(this: *mut *mut Self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterVideoMemoryBudgetChangeNotificationEvent: usize,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32),
}
impl ::windows_sys::core::Interface for IDXGIAdapter3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1683580836, data2: 5010, data3: 17168, data4: [167, 152, 128, 83, 206, 62, 147, 253] };
}
#[repr(C)]
pub struct IDXGIAdapter4 {
    pub base__: IDXGIAdapter3,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesc3: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesc3: usize,
}
impl ::windows_sys::core::Interface for IDXGIAdapter4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1015912913, data2: 20415, data3: 16769, data4: [168, 44, 175, 102, 191, 123, 210, 78] };
}
#[repr(C)]
pub struct IDXGIDebug {
    pub base__: ::windows_sys::core::IUnknown,
    pub ReportLiveObjects: unsafe extern "system" fn(this: *mut *mut Self, apiid: ::windows_sys::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIDebug {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 295597138, data2: 56990, data3: 16638, data4: [136, 6, 136, 249, 12, 18, 180, 65] };
}
#[repr(C)]
pub struct IDXGIDebug1 {
    pub base__: IDXGIDebug,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(this: *mut *mut Self),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(feature = "Win32_Foundation")]
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLeakTrackingEnabledForThread: usize,
}
impl ::windows_sys::core::Interface for IDXGIDebug1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3315621644, data2: 5874, data3: 19167, data4: [159, 77, 168, 196, 213, 138, 197, 80] };
}
#[repr(C)]
pub struct IDXGIDecodeSwapChain {
    pub base__: ::windows_sys::core::IUnknown,
    pub PresentBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut *mut Self, prect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTargetRect: unsafe extern "system" fn(this: *mut *mut Self, prect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTargetRect: usize,
    pub SetDestSize: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSourceRect: unsafe extern "system" fn(this: *mut *mut Self, prect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSourceRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetRect: unsafe extern "system" fn(this: *mut *mut Self, prect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetRect: usize,
    pub GetDestSize: unsafe extern "system" fn(this: *mut *mut Self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(this: *mut *mut Self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows_sys::core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(this: *mut *mut Self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
impl ::windows_sys::core::Interface for IDXGIDecodeSwapChain {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 640878187, data2: 17684, data3: 19578, data4: [143, 216, 18, 234, 152, 5, 157, 24] };
}
#[repr(C)]
pub struct IDXGIDevice {
    pub base__: IDXGIObject,
    pub GetAdapter: unsafe extern "system" fn(this: *mut *mut Self, padapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSurface: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(this: *mut *mut Self, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_sys::core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(this: *mut *mut Self, priority: i32) -> ::windows_sys::core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(this: *mut *mut Self, ppriority: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424783354, data2: 4983, data3: 17638, data4: [140, 50, 136, 253, 95, 68, 200, 76] };
}
#[repr(C)]
pub struct IDXGIDevice1 {
    pub base__: IDXGIDevice,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut *mut Self, maxlatency: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut *mut Self, pmaxlatency: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIDevice1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2010879759, data2: 25206, data3: 18618, data4: [186, 40, 7, 1, 67, 180, 57, 44] };
}
#[repr(C)]
pub struct IDXGIDevice2 {
    pub base__: IDXGIDevice1,
    pub OfferResources: unsafe extern "system" fn(this: *mut *mut Self, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReclaimResources: unsafe extern "system" fn(this: *mut *mut Self, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReclaimResources: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnqueueSetEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnqueueSetEvent: usize,
}
impl ::windows_sys::core::Interface for IDXGIDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 83920407, data2: 64509, data3: 16465, data4: [167, 144, 20, 72, 132, 180, 246, 169] };
}
#[repr(C)]
pub struct IDXGIDevice3 {
    pub base__: IDXGIDevice2,
    pub Trim: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for IDXGIDevice3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1611106668, data2: 12868, data3: 19197, data4: [191, 24, 166, 211, 190, 218, 80, 35] };
}
#[repr(C)]
pub struct IDXGIDevice4 {
    pub base__: IDXGIDevice3,
    pub OfferResources1: unsafe extern "system" fn(this: *mut *mut Self, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows_sys::core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(this: *mut *mut Self, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIDevice4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2511665503, data2: 55514, data3: 19620, data4: [158, 230, 59, 118, 213, 150, 138, 16] };
}
#[repr(C)]
pub struct IDXGIDeviceSubObject {
    pub base__: IDXGIObject,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIDeviceSubObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027474297, data2: 63966, data3: 19800, data4: [187, 108, 24, 214, 41, 146, 241, 166] };
}
#[repr(C)]
pub struct IDXGIDisplayControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStereoEnabled: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStereoEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStereoEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStereoEnabled: usize,
}
impl ::windows_sys::core::Interface for IDXGIDisplayControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3936206618, data2: 51342, data3: 17542, data4: [133, 74, 152, 170, 1, 56, 243, 12] };
}
#[repr(C)]
pub struct IDXGIFactory {
    pub base__: IDXGIObject,
    pub EnumAdapters: unsafe extern "system" fn(this: *mut *mut Self, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MakeWindowAssociation: unsafe extern "system" fn(this: *mut *mut Self, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MakeWindowAssociation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindowAssociation: unsafe extern "system" fn(this: *mut *mut Self, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindowAssociation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChain: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSoftwareAdapter: unsafe extern "system" fn(this: *mut *mut Self, module: super::super::Foundation::HINSTANCE, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSoftwareAdapter: usize,
}
impl ::windows_sys::core::Interface for IDXGIFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2071029484, data2: 8647, data3: 17582, data4: [178, 26, 201, 174, 50, 26, 227, 105] };
}
#[repr(C)]
pub struct IDXGIFactory1 {
    pub base__: IDXGIFactory,
    pub EnumAdapters1: unsafe extern "system" fn(this: *mut *mut Self, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCurrent: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCurrent: usize,
}
impl ::windows_sys::core::Interface for IDXGIFactory1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1997188728, data2: 62063, data3: 19898, data4: [168, 41, 37, 60, 131, 209, 179, 135] };
}
#[repr(C)]
pub struct IDXGIFactory2 {
    pub base__: IDXGIFactory1,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWindowedStereoEnabled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForHwnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCoreWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(this: *mut *mut Self, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedResourceAdapterLuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(this: *mut *mut Self, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterStereoStatusEvent: usize,
    pub UnregisterStereoStatus: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32),
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(this: *mut *mut Self, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterOcclusionStatusEvent: usize,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForComposition: usize,
}
impl ::windows_sys::core::Interface for IDXGIFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1355299356, data2: 57458, data3: 19528, data4: [135, 176, 54, 48, 250, 54, 166, 208] };
}
#[repr(C)]
pub struct IDXGIFactory3 {
    pub base__: IDXGIFactory2,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
}
impl ::windows_sys::core::Interface for IDXGIFactory3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 625489955, data2: 52550, data3: 19581, data4: [134, 202, 71, 170, 149, 184, 55, 189] };
}
#[repr(C)]
pub struct IDXGIFactory4 {
    pub base__: IDXGIFactory3,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumAdapterByLuid: unsafe extern "system" fn(this: *mut *mut Self, adapterluid: super::super::Foundation::LUID, riid: *const ::windows_sys::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumAdapterByLuid: usize,
    pub EnumWarpAdapter: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIFactory4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 466020866, data2: 61238, data3: 17999, data4: [191, 12, 33, 202, 57, 229, 22, 138] };
}
#[repr(C)]
pub struct IDXGIFactory5 {
    pub base__: IDXGIFactory4,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut *mut Self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIFactory5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1983046133, data2: 61029, data3: 19914, data4: [135, 253, 132, 205, 117, 248, 131, 141] };
}
#[repr(C)]
pub struct IDXGIFactory6 {
    pub base__: IDXGIFactory5,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(this: *mut *mut Self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows_sys::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIFactory6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3249957199, data2: 65289, data3: 17577, data4: [176, 60, 119, 144, 10, 10, 29, 23] };
}
#[repr(C)]
pub struct IDXGIFactory7 {
    pub base__: IDXGIFactory6,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterAdaptersChangedEvent: usize,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIFactory7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2761322221, data2: 30427, data3: 17626, data4: [132, 193, 238, 154, 122, 251, 32, 168] };
}
#[repr(C)]
pub struct IDXGIFactoryMedia {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut ::core::ffi::c_void, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDecodeSwapChainForCompositionSurfaceHandle: usize,
}
impl ::windows_sys::core::Interface for IDXGIFactoryMedia {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1105711602, data2: 42385, data3: 20347, data4: [162, 229, 250, 156, 132, 62, 28, 18] };
}
#[repr(C)]
pub struct IDXGIInfoQueue {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMessageCountLimit: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, messagecountlimit: u64) -> ::windows_sys::core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID),
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_sys::core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_sys::core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_sys::core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_sys::core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_sys::core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_sys::core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_sys::core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> u32,
    pub AddMessage: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(this: *mut *mut Self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows_sys::core::PCSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnCategory: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnSeverity: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBreakOnID: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnCategory: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnCategory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnSeverity: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnSeverity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBreakOnID: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, id: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBreakOnID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID, bmute: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMuteDebugOutput: unsafe extern "system" fn(this: *mut *mut Self, producer: ::windows_sys::core::GUID) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMuteDebugOutput: usize,
}
impl ::windows_sys::core::Interface for IDXGIInfoQueue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3597943239, data2: 26410, data3: 18287, data4: [158, 130, 205, 85, 180, 73, 73, 206] };
}
#[repr(C)]
pub struct IDXGIKeyedMutex {
    pub base__: IDXGIDeviceSubObject,
    pub AcquireSync: unsafe extern "system" fn(this: *mut *mut Self, key: u64, dwmilliseconds: u32) -> ::windows_sys::core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(this: *mut *mut Self, key: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIKeyedMutex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2643333769, data2: 55219, data3: 18015, data4: [129, 38, 37, 14, 52, 154, 248, 93] };
}
#[repr(C)]
pub struct IDXGIObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut *mut Self, name: *const ::windows_sys::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut *mut Self, name: *const ::windows_sys::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut *mut Self, name: *const ::windows_sys::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2931961784, data2: 30451, data3: 17977, data4: [155, 224, 40, 235, 67, 166, 122, 46] };
}
#[repr(C)]
pub struct IDXGIOutput {
    pub base__: IDXGIObject,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList: unsafe extern "system" fn(this: *mut *mut Self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode: unsafe extern "system" fn(this: *mut *mut Self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TakeOwnership: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TakeOwnership: usize,
    pub ReleaseOwnership: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetGammaControl: unsafe extern "system" fn(this: *mut *mut Self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControl: unsafe extern "system" fn(this: *mut *mut Self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(this: *mut *mut Self, pscanoutsurface: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(this: *mut *mut Self, pdestination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut *mut Self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIOutput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2919427803, data2: 50997, data3: 18064, data4: [141, 82, 90, 141, 194, 2, 19, 170] };
}
#[repr(C)]
pub struct IDXGIOutput1 {
    pub base__: IDXGIOutput,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayModeList1: unsafe extern "system" fn(this: *mut *mut Self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayModeList1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(this: *mut *mut Self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(this: *mut *mut Self, pdestination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIOutput1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 13491880, data2: 37787, data3: 19331, data4: [163, 64, 166, 133, 34, 102, 102, 204] };
}
#[repr(C)]
pub struct IDXGIOutput2 {
    pub base__: IDXGIOutput1,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsOverlays: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsOverlays: usize,
}
impl ::windows_sys::core::Interface for IDXGIOutput2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1499347409, data2: 10020, data3: 18019, data4: [153, 177, 218, 150, 157, 226, 131, 100] };
}
#[repr(C)]
pub struct IDXGIOutput3 {
    pub base__: IDXGIOutput2,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlaySupport: unsafe extern "system" fn(this: *mut *mut Self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlaySupport: usize,
}
impl ::windows_sys::core::Interface for IDXGIOutput3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2322313985, data2: 32382, data3: 16884, data4: [168, 224, 91, 50, 247, 249, 155, 24] };
}
#[repr(C)]
pub struct IDXGIOutput4 {
    pub base__: IDXGIOutput3,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(this: *mut *mut Self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlayColorSpaceSupport: usize,
}
impl ::windows_sys::core::Interface for IDXGIOutput4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3699231285, data2: 8598, data3: 16717, data4: [159, 83, 97, 120, 132, 3, 42, 96] };
}
#[repr(C)]
pub struct IDXGIOutput5 {
    pub base__: IDXGIOutput4,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DuplicateOutput1: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DuplicateOutput1: usize,
}
impl ::windows_sys::core::Interface for IDXGIOutput5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2157999140, data2: 43858, data3: 17131, data4: [131, 60, 12, 66, 253, 40, 45, 152] };
}
#[repr(C)]
pub struct IDXGIOutput6 {
    pub base__: IDXGIOutput5,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(this: *mut *mut Self, pflags: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIOutput6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 109266664, data2: 43756, data3: 19332, data4: [173, 215, 19, 127, 81, 63, 119, 161] };
}
#[repr(C)]
pub struct IDXGIOutputDuplication {
    pub base__: IDXGIObject,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireNextFrame: unsafe extern "system" fn(this: *mut *mut Self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireNextFrame: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameDirtyRects: unsafe extern "system" fn(this: *mut *mut Self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameDirtyRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameMoveRects: unsafe extern "system" fn(this: *mut *mut Self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameMoveRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFramePointerShape: unsafe extern "system" fn(this: *mut *mut Self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFramePointerShape: usize,
    pub MapDesktopSurface: unsafe extern "system" fn(this: *mut *mut Self, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows_sys::core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIOutputDuplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 421329603, data2: 41793, data3: 18189, data4: [178, 110, 168, 100, 244, 40, 49, 156] };
}
#[repr(C)]
pub struct IDXGIResource {
    pub base__: IDXGIDeviceSubObject,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSharedHandle: unsafe extern "system" fn(this: *mut *mut Self, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSharedHandle: usize,
    pub GetUsage: unsafe extern "system" fn(this: *mut *mut Self, pusage: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut *mut Self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows_sys::core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut *mut Self, pevictionpriority: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGIResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 56572596, data2: 18478, data3: 20048, data4: [180, 31, 138, 127, 139, 216, 150, 11] };
}
#[repr(C)]
pub struct IDXGIResource1 {
    pub base__: IDXGIResource,
    pub CreateSubresourceSurface: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut *mut Self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows_sys::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    CreateSharedHandle: usize,
}
impl ::windows_sys::core::Interface for IDXGIResource1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 815141753, data2: 17929, data3: 19009, data4: [153, 142, 84, 254, 86, 126, 224, 193] };
}
#[repr(C)]
pub struct IDXGISurface {
    pub base__: IDXGIDeviceSubObject,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(this: *mut *mut Self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_sys::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3405559148, data2: 27331, data3: 18569, data4: [191, 71, 158, 35, 187, 210, 96, 236] };
}
#[repr(C)]
pub struct IDXGISurface1 {
    pub base__: IDXGISurface,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDC: unsafe extern "system" fn(this: *mut *mut Self, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut *mut Self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseDC: usize,
}
impl ::windows_sys::core::Interface for IDXGISurface1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1256599698, data2: 25383, data3: 19483, data4: [128, 174, 191, 225, 46, 163, 43, 134] };
}
#[repr(C)]
pub struct IDXGISurface2 {
    pub base__: IDXGISurface1,
    pub GetResource: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISurface2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2879690461, data2: 46615, data3: 19640, data4: [168, 102, 188, 68, 215, 235, 31, 162] };
}
#[repr(C)]
pub struct IDXGISwapChain {
    pub base__: IDXGIDeviceSubObject,
    pub Present: unsafe extern "system" fn(this: *mut *mut Self, syncinterval: u32, flags: u32) -> ::windows_sys::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: u32, riid: *const ::windows_sys::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFullscreenState: unsafe extern "system" fn(this: *mut *mut Self, fullscreen: super::super::Foundation::BOOL, ptarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFullscreenState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullscreenState: unsafe extern "system" fn(this: *mut *mut Self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullscreenState: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(this: *mut *mut Self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeTarget: unsafe extern "system" fn(this: *mut *mut Self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(this: *mut *mut Self, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut *mut Self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_sys::core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(this: *mut *mut Self, plastpresentcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISwapChain {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 822949536, data2: 53991, data3: 19466, data4: [170, 4, 106, 157, 35, 184, 136, 106] };
}
#[repr(C)]
pub struct IDXGISwapChain1 {
    pub base__: IDXGISwapChain,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDesc1: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDesc1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetFullscreenDesc: unsafe extern "system" fn(this: *mut *mut Self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetFullscreenDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwnd: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwnd: usize,
    pub GetCoreWindow: unsafe extern "system" fn(this: *mut *mut Self, refiid: *const ::windows_sys::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Present1: unsafe extern "system" fn(this: *mut *mut Self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Present1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTemporaryMonoSupported: usize,
    pub GetRestrictToOutput: unsafe extern "system" fn(this: *mut *mut Self, pprestricttooutput: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *const DXGI_RGBA) -> ::windows_sys::core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut DXGI_RGBA) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotation: unsafe extern "system" fn(this: *mut *mut Self, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotation: usize,
}
impl ::windows_sys::core::Interface for IDXGISwapChain1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2030716407, data2: 3394, data3: 18550, data4: [152, 58, 10, 85, 207, 230, 244, 170] };
}
#[repr(C)]
pub struct IDXGISwapChain2 {
    pub base__: IDXGISwapChain1,
    pub SetSourceSize: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32) -> ::windows_sys::core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(this: *mut *mut Self, pwidth: *mut u32, pheight: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(this: *mut *mut Self, maxlatency: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(this: *mut *mut Self, pmaxlatency: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFrameLatencyWaitableObject: usize,
    pub SetMatrixTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_sys::core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(this: *mut *mut Self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISwapChain2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2831035076, data2: 6559, data3: 18758, data4: [179, 49, 121, 89, 159, 185, 141, 231] };
}
#[repr(C)]
pub struct IDXGISwapChain3 {
    pub base__: IDXGISwapChain2,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(this: *mut *mut Self, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace1: unsafe extern "system" fn(this: *mut *mut Self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers1: unsafe extern "system" fn(this: *mut *mut Self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers1: usize,
}
impl ::windows_sys::core::Interface for IDXGISwapChain3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2497289179, data2: 61944, data3: 19120, data4: [178, 54, 125, 160, 23, 14, 218, 177] };
}
#[repr(C)]
pub struct IDXGISwapChain4 {
    pub base__: IDXGISwapChain3,
    pub SetHDRMetaData: unsafe extern "system" fn(this: *mut *mut Self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISwapChain4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029201242, data2: 48458, data3: 18590, data4: [177, 244, 61, 188, 182, 69, 47, 251] };
}
#[repr(C)]
pub struct IDXGISwapChainMedia {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(this: *mut *mut Self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows_sys::core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: u32) -> ::windows_sys::core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(this: *mut *mut Self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXGISwapChainMedia {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3717576971, data2: 61535, data3: 20330, data4: [189, 101, 37, 191, 178, 100, 189, 132] };
}
#[repr(C)]
pub struct IDXGraphicsAnalysis {
    pub base__: ::windows_sys::core::IUnknown,
    pub BeginCapture: unsafe extern "system" fn(this: *mut *mut Self),
    pub EndCapture: unsafe extern "system" fn(this: *mut *mut Self),
}
impl ::windows_sys::core::Interface for IDXGraphicsAnalysis {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2670007572, data2: 40269, data3: 18690, data4: [157, 96, 24, 152, 138, 183, 212, 181] };
}
