#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
pub type DisplayMonitor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: Self = Self(0i32);
    pub const Wired: Self = Self(1i32);
    pub const Wireless: Self = Self(2i32);
    pub const Virtual: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayMonitorConnectionKind {}
impl ::core::clone::Clone for DisplayMonitorConnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: Self = Self(0i32);
    pub const DisplayId: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayMonitorDescriptorKind {}
impl ::core::clone::Clone for DisplayMonitorDescriptorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: Self = Self(0i32);
    pub const HD15: Self = Self(1i32);
    pub const AnalogTV: Self = Self(2i32);
    pub const Dvi: Self = Self(3i32);
    pub const Hdmi: Self = Self(4i32);
    pub const Lvds: Self = Self(5i32);
    pub const Sdi: Self = Self(6i32);
    pub const DisplayPort: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayMonitorPhysicalConnectorKind {}
impl ::core::clone::Clone for DisplayMonitorPhysicalConnectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: Self = Self(0i32);
    pub const HeadMounted: Self = Self(1i32);
    pub const SpecialPurpose: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayMonitorUsageKind {}
impl ::core::clone::Clone for DisplayMonitorUsageKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDisplayMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayMonitorConnectionKind) -> ::windows_sys::core::HRESULT,
    pub PhysicalConnector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows_sys::core::HRESULT,
    pub DisplayAdapterDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub DisplayAdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    DisplayAdapterId: usize,
    pub DisplayAdapterTargetId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayMonitorUsageKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub NativeResolutionInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    NativeResolutionInRawPixels: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalSizeInInches: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalSizeInInches: usize,
    pub RawDpiX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RedPrimary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub GreenPrimary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GreenPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub BluePrimary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BluePrimary: usize,
    #[cfg(feature = "Foundation")]
    pub WhitePoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhitePoint: usize,
    pub MaxLuminanceInNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetDescriptor: unsafe extern "system" fn(this: *mut *mut Self, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayMonitor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDolbyVisionSupportedInHdrMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayMonitorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromInterfaceIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceinterfaceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromInterfaceIdAsync: usize,
}
