#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[repr(C)]
pub struct ILamp {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub IsColorSettable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
}
impl ::windows_sys::core::Interface for ILamp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 75324314, data2: 59973, data3: 19243, data4: [177, 162, 20, 223, 240, 11, 222, 123] };
}
#[repr(C)]
pub struct ILampArray {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub HardwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub LampArrayKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampArrayKind) -> ::windows_sys::core::HRESULT,
    pub LampCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinUpdateInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinUpdateInterval: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BoundingBox: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportsVirtualKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetLampInfo: unsafe extern "system" fn(this: *mut *mut Self, lampindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetIndicesForKey: unsafe extern "system" fn(this: *mut *mut Self, key: super::super::System::VirtualKey, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIndicesForKey: usize,
    pub GetIndicesForPurposes: unsafe extern "system" fn(this: *mut *mut Self, purposes: LampPurposes, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut *mut Self, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut *mut Self, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKey: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKey: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKeys: unsafe extern "system" fn(this: *mut *mut Self, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, keys_array_size: u32, keys: *const super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKeys: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForPurposes: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForPurposes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: i32, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestMessageAsync: usize,
}
impl ::windows_sys::core::Interface for ILampArray {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2060359559, data2: 51360, data3: 20117, data4: [161, 224, 213, 134, 118, 83, 134, 73] };
}
#[repr(C)]
pub struct ILampArrayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for ILampArrayStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2075707789, data2: 24513, data3: 17709, data4: [187, 31, 74, 212, 16, 211, 152, 255] };
}
#[repr(C)]
pub struct ILampAvailabilityChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILampAvailabilityChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1332624877, data2: 1954, data3: 18845, data4: [146, 96, 103, 227, 4, 83, 43, 164] };
}
#[repr(C)]
pub struct ILampInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Purposes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LampPurposes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RedLevelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GreenLevelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BlueLevelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GainLevelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub FixedColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    FixedColor: usize,
    #[cfg(feature = "UI")]
    pub GetNearestSupportedColor: unsafe extern "system" fn(this: *mut *mut Self, desiredcolor: super::super::UI::Color, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetNearestSupportedColor: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateLatency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateLatency: usize,
}
impl ::windows_sys::core::Interface for ILampInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 817582620, data2: 2767, data3: 18906, data4: [140, 16, 21, 11, 156, 246, 39, 19] };
}
#[repr(C)]
pub struct ILampStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
impl ::windows_sys::core::Interface for ILampStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2820817260, data2: 34949, data3: 16414, data4: [184, 33, 142, 139, 56, 168, 232, 236] };
}
pub type Lamp = *mut ::core::ffi::c_void;
pub type LampArray = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const GameController: Self = Self(3i32);
    pub const Peripheral: Self = Self(4i32);
    pub const Scene: Self = Self(5i32);
    pub const Notification: Self = Self(6i32);
    pub const Chassis: Self = Self(7i32);
    pub const Wearable: Self = Self(8i32);
    pub const Furniture: Self = Self(9i32);
    pub const Art: Self = Self(10i32);
}
impl ::core::marker::Copy for LampArrayKind {}
impl ::core::clone::Clone for LampArrayKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LampAvailabilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type LampInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Accent: Self = Self(2u32);
    pub const Branding: Self = Self(4u32);
    pub const Status: Self = Self(8u32);
    pub const Illumination: Self = Self(16u32);
    pub const Presentation: Self = Self(32u32);
}
impl ::core::marker::Copy for LampPurposes {}
impl ::core::clone::Clone for LampPurposes {
    fn clone(&self) -> Self {
        *self
    }
}
