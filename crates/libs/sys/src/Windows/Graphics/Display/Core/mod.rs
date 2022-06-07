#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayColorSpace {}
impl ::core::clone::Clone for HdmiDisplayColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
pub struct HdmiDisplayHdr2086Metadata {
    pub RedPrimaryX: u16,
    pub RedPrimaryY: u16,
    pub GreenPrimaryX: u16,
    pub GreenPrimaryY: u16,
    pub BluePrimaryX: u16,
    pub BluePrimaryY: u16,
    pub WhitePointX: u16,
    pub WhitePointY: u16,
    pub MaxMasteringLuminance: u16,
    pub MinMasteringLuminance: u16,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for HdmiDisplayHdr2086Metadata {}
impl ::core::clone::Clone for HdmiDisplayHdr2086Metadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayHdrOption {}
impl ::core::clone::Clone for HdmiDisplayHdrOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HdmiDisplayInformation = *mut ::core::ffi::c_void;
pub type HdmiDisplayMode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayPixelEncoding {}
impl ::core::clone::Clone for HdmiDisplayPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IHdmiDisplayInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedDisplayModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedDisplayModes: usize,
    pub GetCurrentDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultDisplayModeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut ::core::ffi::c_void, hdroption: HdmiDisplayHdrOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut ::core::ffi::c_void, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayModesChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayModesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayModesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayModesChanged: usize,
}
impl ::windows_sys::core::Interface for IHdmiDisplayInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 319503370, data2: 62821, data3: 18286, data4: [171, 213, 234, 5, 174, 231, 76, 105] };
}
#[repr(C)]
pub struct IHdmiDisplayInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHdmiDisplayInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1827058272, data2: 62506, data3: 18965, data4: [145, 76, 123, 142, 42, 90, 101, 223] };
}
#[repr(C)]
pub struct IHdmiDisplayMode {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolutionWidthInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ResolutionHeightInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HdmiDisplayColorSpace) -> ::windows_sys::core::HRESULT,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HdmiDisplayPixelEncoding) -> ::windows_sys::core::HRESULT,
    pub IsSdrLuminanceSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSmpte2084Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Is2086MetadataSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHdmiDisplayMode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 201774509, data2: 7056, data3: 20305, data4: [153, 129, 239, 90, 28, 13, 223, 102] };
}
#[repr(C)]
pub struct IHdmiDisplayMode2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDolbyVisionLowLatencySupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHdmiDisplayMode2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 130895519, data2: 19260, data3: 17080, data4: [132, 231, 137, 83, 104, 113, 138, 242] };
}
