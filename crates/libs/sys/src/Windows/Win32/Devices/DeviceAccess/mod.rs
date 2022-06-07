#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
    pub fn CreateDeviceAccessInstance(deviceinterfacepath: ::windows_sys::core::PCWSTR, desiredaccess: u32, createasync: *mut *mut *mut ICreateDeviceAccessAsync) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DeviceIoControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 315876210, data2: 34635, data3: 17789, data4: [159, 223, 115, 151, 119, 120, 104, 108] };
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_1394: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_ARTI: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_COM1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_COM2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_COM3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_COM4: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_DIAQ: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_MAX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_SIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const DEV_PORT_USB: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_10: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_11: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_12: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_13: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_14: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_15: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_16: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_17: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_18: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_19: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_20: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_21: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_22: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_23: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_24: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_3: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_4: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_5: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_6: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_7: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_8: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_9: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_AUDIO_ALL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_BASE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_BOTTOM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_CENTER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_LEFT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_MIDDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_RIGHT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_TOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAccess\"`*"]
pub const ED_VIDEO: i32 = 33554432i32;
#[repr(C)]
pub struct ICreateDeviceAccessAsync {
    pub base__: ::windows_sys::core::IUnknown,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut *mut Self, timeout: u32) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateDeviceAccessAsync {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 880042639, data2: 26685, data3: 17106, data4: [171, 203, 219, 1, 140, 101, 3, 188] };
}
#[repr(C)]
pub struct IDeviceIoControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub DeviceIoControlSync: unsafe extern "system" fn(this: *mut *mut Self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DeviceIoControlAsync: unsafe extern "system" fn(this: *mut *mut Self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: *mut ::core::ffi::c_void, cancelcontext: *mut usize) -> ::windows_sys::core::HRESULT,
    pub CancelOperation: unsafe extern "system" fn(this: *mut *mut Self, cancelcontext: usize) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceIoControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2666520929, data2: 9131, data3: 20248, data4: [155, 73, 153, 27, 88, 106, 233, 112] };
}
#[repr(C)]
pub struct IDeviceRequestCompletionCallback {
    pub base__: ::windows_sys::core::IUnknown,
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self, requestresult: ::windows_sys::core::HRESULT, bytesreturned: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeviceRequestCompletionCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2577116452, data2: 39629, data3: 17851, data4: [134, 105, 42, 47, 192, 40, 139, 4] };
}
