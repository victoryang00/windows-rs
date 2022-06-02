#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GipFirmwareUpdateResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for GipFirmwareUpdateStatus {}
impl ::core::clone::Clone for GipFirmwareUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GipGameControllerProvider = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl ::core::marker::Copy for GipMessageClass {}
impl ::core::clone::Clone for GipMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HidGameControllerProvider = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICustomGameControllerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateGameController: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameControllerFactoryManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, interfaceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_sys::core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameControllerFactoryManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputResumed: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64) -> ::windows_sys::core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub FirmwareVersionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameControllerVersionInfo) -> ::windows_sys::core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HardwareVersionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameControllerVersionInfo) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGipFirmwareUpdateResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GipFirmwareUpdateStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGipGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_sys::core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGipGameControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub SendMessage: unsafe extern "system" fn(this: *mut *mut Self, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub SendReceiveMessage: unsafe extern "system" fn(this: *mut *mut Self, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UpdateFirmwareAsync: unsafe extern "system" fn(this: *mut *mut Self, firmwareimage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UpdateFirmwareAsync: usize,
}
#[repr(C)]
pub struct IHidGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHidGameControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub UsageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetFeatureReport: unsafe extern "system" fn(this: *mut *mut Self, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SendFeatureReport: unsafe extern "system" fn(this: *mut *mut Self, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_sys::core::HRESULT,
    pub SendOutputReport: unsafe extern "system" fn(this: *mut *mut Self, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXusbGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXusbGameControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetVibration: unsafe extern "system" fn(this: *mut *mut Self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl ::core::marker::Copy for XusbDeviceSubtype {}
impl ::core::clone::Clone for XusbDeviceSubtype {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl ::core::marker::Copy for XusbDeviceType {}
impl ::core::clone::Clone for XusbDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XusbGameControllerProvider = *mut ::core::ffi::c_void;
