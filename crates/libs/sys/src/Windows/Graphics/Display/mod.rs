#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
pub type AdvancedColorInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl ::core::marker::Copy for AdvancedColorKind {}
impl ::core::clone::Clone for AdvancedColorKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BrightnessOverride = *mut ::core::ffi::c_void;
pub type BrightnessOverrideSettings = *mut ::core::ffi::c_void;
pub type ColorOverrideSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideOptions {}
impl ::core::clone::Clone for DisplayBrightnessOverrideOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideScenario {}
impl ::core::clone::Clone for DisplayBrightnessOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayBrightnessScenario {}
impl ::core::clone::Clone for DisplayBrightnessScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayColorOverrideScenario {}
impl ::core::clone::Clone for DisplayColorOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayEnhancementOverride = *mut ::core::ffi::c_void;
pub type DisplayEnhancementOverrideCapabilities = *mut ::core::ffi::c_void;
pub type DisplayEnhancementOverrideCapabilitiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type DisplayInformation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
impl ::core::marker::Copy for DisplayOrientations {}
impl ::core::clone::Clone for DisplayOrientations {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayPropertiesEventHandler = *mut ::core::ffi::c_void;
pub type DisplayServices = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl ::core::marker::Copy for HdrMetadataFormat {}
impl ::core::clone::Clone for HdrMetadataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdvancedColorInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdvancedColorKind) -> ::windows_sys::core::HRESULT,
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
    pub SdrWhiteLevelInNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported: unsafe extern "system" fn(this: *mut *mut Self, format: HdrMetadataFormat, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(this: *mut *mut Self, kind: AdvancedColorKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedColorInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2274876667, data2: 45609, data3: 16513, data4: [174, 154, 44, 200, 94, 52, 173, 106] };
}
#[repr(C)]
pub struct IBrightnessOverride {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut *mut Self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows_sys::core::HRESULT,
    pub SetBrightnessScenario: unsafe extern "system" fn(this: *mut *mut Self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows_sys::core::HRESULT,
    pub GetLevelForScenario: unsafe extern "system" fn(this: *mut *mut Self, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub StartOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BrightnessLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BrightnessLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBrightnessLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBrightnessLevelChanged: usize,
}
impl ::windows_sys::core::Interface for IBrightnessOverride {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2529780250, data2: 49475, data3: 17298, data4: [190, 221, 74, 126, 149, 116, 200, 253] };
}
#[repr(C)]
pub struct IBrightnessOverrideSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DesiredNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBrightnessOverrideSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3507661610, data2: 30212, data3: 19898, data4: [188, 248, 75, 111, 73, 80, 44, 176] };
}
#[repr(C)]
pub struct IBrightnessOverrideSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLevel: unsafe extern "system" fn(this: *mut *mut Self, level: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNits: unsafe extern "system" fn(this: *mut *mut Self, nits: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDisplayBrightnessOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBrightnessOverrideSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3565673616, data2: 28532, data3: 17419, data4: [179, 131, 95, 233, 108, 240, 11, 15] };
}
#[repr(C)]
pub struct IBrightnessOverrideStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultForSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveForSystemAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveForSystemAsync: usize,
}
impl ::windows_sys::core::Interface for IBrightnessOverrideStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 61323757, data2: 57841, data3: 19048, data4: [161, 31, 148, 106, 216, 206, 83, 147] };
}
#[repr(C)]
pub struct IColorOverrideSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayColorOverrideScenario) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorOverrideSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4226785588, data2: 19073, data3: 19533, data4: [165, 182, 125, 27, 92, 75, 208, 11] };
}
#[repr(C)]
pub struct IColorOverrideSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, overridescenario: DisplayColorOverrideScenario, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorOverrideSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2959663199, data2: 50207, data3: 19145, data4: [175, 171, 130, 122, 182, 36, 143, 154] };
}
#[repr(C)]
pub struct IDisplayEnhancementOverride {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorOverrideSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColorOverrideSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BrightnessOverrideSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBrightnessOverrideSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetCurrentDisplayEnhancementOverrideCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CanOverrideChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanOverrideChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanOverrideChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanOverrideChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsOverrideActiveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayEnhancementOverrideCapabilitiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayEnhancementOverrideCapabilitiesChanged: usize,
}
impl ::windows_sys::core::Interface for IDisplayEnhancementOverride {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1117099215, data2: 55674, data3: 19202, data4: [164, 40, 92, 66, 146, 247, 245, 34] };
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBrightnessControlSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBrightnessNitsControlSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedNitRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedNitRanges: usize,
}
impl ::windows_sys::core::Interface for IDisplayEnhancementOverrideCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1164992734, data2: 61018, data3: 18359, data4: [153, 24, 30, 81, 232, 18, 204, 200] };
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3680626276, data2: 5626, data3: 18906, data4: [139, 119, 7, 219, 210, 175, 88, 93] };
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayEnhancementOverrideStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3478879937, data2: 38801, data3: 17491, data4: [176, 19, 41, 182, 247, 120, 229, 25] };
}
#[repr(C)]
pub struct IDisplayInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    pub NativeOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OrientationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOrientationChanged: usize,
    pub ResolutionScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ResolutionScale) -> ::windows_sys::core::HRESULT,
    pub LogicalDpi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DpiChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DpiChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDpiChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDpiChanged: usize,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StereoEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetColorProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorProfileChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorProfileChanged: usize,
}
impl ::windows_sys::core::Interface for IDisplayInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3201372846, data2: 44483, data3: 19913, data4: [174, 101, 133, 31, 77, 125, 71, 153] };
}
#[repr(C)]
pub struct IDisplayInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawPixelsPerViewPixel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1305280545, data2: 64209, data3: 19342, data4: [142, 223, 119, 88, 135, 184, 191, 25] };
}
#[repr(C)]
pub struct IDisplayInformation3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DiagonalSizeInInches: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiagonalSizeInInches: usize,
}
impl ::windows_sys::core::Interface for IDisplayInformation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3675586845, data2: 3849, data3: 17510, data4: [143, 243, 17, 222, 154, 60, 146, 154] };
}
#[repr(C)]
pub struct IDisplayInformation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScreenWidthInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ScreenHeightInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayInformation4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3379744303, data2: 4674, data3: 18110, data4: [181, 54, 225, 170, 254, 158, 122, 207] };
}
#[repr(C)]
pub struct IDisplayInformation5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvancedColorInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvancedColorInfoChanged: usize,
}
impl ::windows_sys::core::Interface for IDisplayInformation5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 978600668, data2: 11486, data3: 19085, data4: [128, 209, 33, 220, 90, 220, 193, 170] };
}
#[repr(C)]
pub struct IDisplayInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut *mut Self, value: DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayContentsInvalidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayContentsInvalidated: usize,
}
impl ::windows_sys::core::Interface for IDisplayInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3332385388, data2: 54354, data3: 17628, data4: [186, 7, 150, 243, 198, 173, 249, 209] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDisplayPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub NativeOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NativeOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut *mut Self, value: DisplayOrientations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAutoRotationPreferences: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    OrientationChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveOrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ResolutionScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ResolutionScale) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResolutionScale: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpi: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LogicalDpiChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LogicalDpiChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLogicalDpiChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabled: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    GetColorProfileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ColorProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveColorProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DisplayContentsInvalidated: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveDisplayContentsInvalidated: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IDisplayPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1765272973, data2: 12522, data3: 19949, data4: [130, 113, 69, 83, 255, 2, 246, 138] };
}
#[repr(C)]
pub struct IDisplayServices {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IDisplayServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 458552107, data2: 35085, data3: 22343, data4: [189, 38, 253, 189, 235, 12, 138, 113] };
}
#[repr(C)]
pub struct IDisplayServicesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindAll: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDisplayServicesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3693123263, data2: 29450, data3: 21856, data4: [180, 97, 145, 193, 61, 105, 46, 12] };
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Display\"`*"]
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl ::core::marker::Copy for NitRange {}
impl ::core::clone::Clone for NitRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: Self = Self(0i32);
    pub const Scale100Percent: Self = Self(100i32);
    pub const Scale120Percent: Self = Self(120i32);
    pub const Scale125Percent: Self = Self(125i32);
    pub const Scale140Percent: Self = Self(140i32);
    pub const Scale150Percent: Self = Self(150i32);
    pub const Scale160Percent: Self = Self(160i32);
    pub const Scale175Percent: Self = Self(175i32);
    pub const Scale180Percent: Self = Self(180i32);
    pub const Scale200Percent: Self = Self(200i32);
    pub const Scale225Percent: Self = Self(225i32);
    pub const Scale250Percent: Self = Self(250i32);
    pub const Scale300Percent: Self = Self(300i32);
    pub const Scale350Percent: Self = Self(350i32);
    pub const Scale400Percent: Self = Self(400i32);
    pub const Scale450Percent: Self = Self(450i32);
    pub const Scale500Percent: Self = Self(500i32);
}
impl ::core::marker::Copy for ResolutionScale {}
impl ::core::clone::Clone for ResolutionScale {
    fn clone(&self) -> Self {
        *self
    }
}
