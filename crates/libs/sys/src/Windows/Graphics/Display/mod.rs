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
#[repr(C)]
pub struct IBrightnessOverrideSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DesiredNits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBrightnessOverrideSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLevel: unsafe extern "system" fn(this: *mut *mut Self, level: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNits: unsafe extern "system" fn(this: *mut *mut Self, nits: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDisplayBrightnessOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IColorOverrideSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayColorOverrideScenario) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorOverrideSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut *mut Self, overridescenario: DisplayColorOverrideScenario, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDisplayInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawPixelsPerViewPixel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayInformation3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DiagonalSizeInInches: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiagonalSizeInInches: usize,
}
#[repr(C)]
pub struct IDisplayInformation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScreenWidthInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ScreenHeightInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IDisplayServices {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDisplayServicesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FindAll: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows_sys::core::HRESULT,
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
