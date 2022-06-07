#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
    pub fn DXCoreCreateAdapterFactory(riid: *const ::windows_sys::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2353497707, data2: 30083, data3: 17677, data4: [240, 240, 107, 173, 168, 149, 175, 75] };
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 613296128, data2: 42899, data3: 18212, data4: [171, 170, 35, 166, 222, 27, 224, 144] };
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 211734093, data2: 12142, data3: 20225, data4: [140, 150, 232, 158, 51, 27, 71, 177] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreAdapterMemoryBudget {
    pub budget: u64,
    pub currentUsage: u64,
    pub availableForReservation: u64,
    pub currentReservation: u64,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudget {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    pub nodeIndex: u32,
    pub segmentGroup: DXCoreSegmentGroup,
}
impl ::core::marker::Copy for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::clone::Clone for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterPreference = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Hardware: DXCoreAdapterPreference = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const MinimumPower: DXCoreAdapterPreference = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HighPerformance: DXCoreAdapterPreference = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterProperty = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const InstanceLuid: DXCoreAdapterProperty = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverVersion: DXCoreAdapterProperty = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DriverDescription: DXCoreAdapterProperty = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareID: DXCoreAdapterProperty = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const KmdModelVersion: DXCoreAdapterProperty = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const ComputePreemptionGranularity: DXCoreAdapterProperty = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const GraphicsPreemptionGranularity: DXCoreAdapterProperty = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedAdapterMemory: DXCoreAdapterProperty = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const DedicatedSystemMemory: DXCoreAdapterProperty = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const SharedSystemMemory: DXCoreAdapterProperty = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AcgCompatible: DXCoreAdapterProperty = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsHardware: DXCoreAdapterProperty = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsIntegrated: DXCoreAdapterProperty = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDetachable: DXCoreAdapterProperty = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const HardwareIDParts: DXCoreAdapterProperty = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreAdapterState = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const IsDriverUpdateInProgress: DXCoreAdapterState = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterMemoryBudget: DXCoreAdapterState = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreHardwareID {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSysID: u32,
    pub revision: u32,
}
impl ::core::marker::Copy for DXCoreHardwareID {}
impl ::core::clone::Clone for DXCoreHardwareID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub struct DXCoreHardwareIDParts {
    pub vendorID: u32,
    pub deviceID: u32,
    pub subSystemID: u32,
    pub subVendorID: u32,
    pub revisionID: u32,
}
impl ::core::marker::Copy for DXCoreHardwareIDParts {}
impl ::core::clone::Clone for DXCoreHardwareIDParts {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreNotificationType = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterListStale: DXCoreNotificationType = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterNoLongerValid: DXCoreNotificationType = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterBudgetChange: DXCoreNotificationType = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const AdapterHardwareContentProtectionTeardown: DXCoreNotificationType = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type DXCoreSegmentGroup = u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const Local: DXCoreSegmentGroup = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const NonLocal: DXCoreSegmentGroup = 1u32;
#[repr(C)]
pub struct IDXCoreAdapter {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsValid: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub IsAttributeSupported: unsafe extern "system" fn(this: *mut *mut Self, attributeguid: *const ::windows_sys::core::GUID) -> bool,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut *mut Self, property: DXCoreAdapterProperty) -> bool,
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPropertySize: unsafe extern "system" fn(this: *mut *mut Self, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows_sys::core::HRESULT,
    pub IsQueryStateSupported: unsafe extern "system" fn(this: *mut *mut Self, property: DXCoreAdapterState) -> bool,
    pub QueryState: unsafe extern "system" fn(this: *mut *mut Self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSetStateSupported: unsafe extern "system" fn(this: *mut *mut Self, property: DXCoreAdapterState) -> bool,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFactory: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXCoreAdapter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4040903807, data2: 65114, data3: 17058, data4: [189, 98, 242, 166, 207, 111, 200, 62] };
}
#[repr(C)]
pub struct IDXCoreAdapterFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateAdapterList: unsafe extern "system" fn(this: *mut *mut Self, numattributes: u32, filterattributes: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAdapterByLuid: unsafe extern "system" fn(this: *mut *mut Self, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows_sys::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAdapterByLuid: usize,
    pub IsNotificationTypeSupported: unsafe extern "system" fn(this: *mut *mut Self, notificationtype: DXCoreNotificationType) -> bool,
    pub RegisterEventNotification: unsafe extern "system" fn(this: *mut *mut Self, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnregisterEventNotification: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDXCoreAdapterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2028886341, data2: 50030, data3: 19219, data4: [166, 105, 0, 93, 209, 28, 15, 6] };
}
#[repr(C)]
pub struct IDXCoreAdapterList {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAdapter: unsafe extern "system" fn(this: *mut *mut Self, index: u32, riid: *const ::windows_sys::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub IsStale: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub GetFactory: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Sort: unsafe extern "system" fn(this: *mut *mut Self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows_sys::core::HRESULT,
    pub IsAdapterPreferenceSupported: unsafe extern "system" fn(this: *mut *mut Self, preference: DXCoreAdapterPreference) -> bool,
}
impl ::windows_sys::core::Interface for IDXCoreAdapterList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1382840182, data2: 16617, data3: 17819, data4: [183, 17, 243, 42, 215, 109, 252, 40] };
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub type PFN_DXCORE_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: DXCoreNotificationType, object: *mut *mut ::windows_sys::core::IUnknown, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`*"]
pub const _FACDXCORE: u32 = 2176u32;
