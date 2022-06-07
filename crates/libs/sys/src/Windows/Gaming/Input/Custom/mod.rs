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
impl ::windows_sys::core::Interface for ICustomGameControllerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1772138078, data2: 30094, data3: 19646, data4: [172, 230, 98, 21, 95, 233, 18, 111] };
}
#[repr(C)]
pub struct IGameControllerFactoryManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, interfaceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_sys::core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameControllerFactoryManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 919299811, data2: 53409, data3: 18822, data4: [162, 76, 64, 177, 55, 222, 186, 158] };
}
#[repr(C)]
pub struct IGameControllerFactoryManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameControllerFactoryManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3939391044, data2: 6623, data3: 16661, data4: [179, 42, 39, 147, 226, 174, 163, 187] };
}
#[repr(C)]
pub struct IGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputResumed: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64) -> ::windows_sys::core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameControllerInputSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 536279330, data2: 50752, data3: 19576, data4: [168, 32, 154, 113, 92, 85, 139, 203] };
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
impl ::windows_sys::core::Interface for IGameControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872864642, data2: 10646, data3: 17753, data4: [177, 108, 62, 87, 212, 110, 88, 214] };
}
#[repr(C)]
pub struct IGipFirmwareUpdateResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GipFirmwareUpdateStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGipFirmwareUpdateResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1803111730, data2: 34131, data3: 17042, data4: [142, 3, 225, 102, 81, 162, 248, 188] };
}
#[repr(C)]
pub struct IGipGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_sys::core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGipGameControllerInputSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2718993087, data2: 2545, data3: 17340, data4: [161, 64, 128, 248, 153, 236, 54, 251] };
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
impl ::windows_sys::core::Interface for IGipGameControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687783961, data2: 6901, data3: 17832, data4: [191, 2, 160, 238, 80, 200, 35, 252] };
}
#[repr(C)]
pub struct IHidGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHidGameControllerInputSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4149527330, data2: 6189, data3: 16612, data4: [161, 38, 252, 238, 79, 250, 30, 49] };
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
impl ::windows_sys::core::Interface for IHidGameControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2513320692, data2: 44016, data3: 19304, data4: [160, 129, 59, 125, 231, 63, 240, 231] };
}
#[repr(C)]
pub struct IXusbGameControllerInputSink {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut *mut Self, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXusbGameControllerInputSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2997624213, data2: 28363, data3: 17075, data4: [138, 171, 2, 84, 1, 202, 71, 18] };
}
#[repr(C)]
pub struct IXusbGameControllerProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetVibration: unsafe extern "system" fn(this: *mut *mut Self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXusbGameControllerProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1848209899, data2: 3835, data3: 18612, data4: [128, 139, 131, 118, 67, 178, 242, 22] };
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
