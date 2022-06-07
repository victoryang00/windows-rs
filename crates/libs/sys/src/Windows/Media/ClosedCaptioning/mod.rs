#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: Self = Self(0i32);
    pub const White: Self = Self(1i32);
    pub const Black: Self = Self(2i32);
    pub const Red: Self = Self(3i32);
    pub const Green: Self = Self(4i32);
    pub const Blue: Self = Self(5i32);
    pub const Yellow: Self = Self(6i32);
    pub const Magenta: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
}
impl ::core::marker::Copy for ClosedCaptionColor {}
impl ::core::clone::Clone for ClosedCaptionColor {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Raised: Self = Self(2i32);
    pub const Depressed: Self = Self(3i32);
    pub const Uniform: Self = Self(4i32);
    pub const DropShadow: Self = Self(5i32);
}
impl ::core::marker::Copy for ClosedCaptionEdgeEffect {}
impl ::core::clone::Clone for ClosedCaptionEdgeEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0i32);
    pub const OneHundredPercent: Self = Self(1i32);
    pub const SeventyFivePercent: Self = Self(2i32);
    pub const TwentyFivePercent: Self = Self(3i32);
    pub const ZeroPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionOpacity {}
impl ::core::clone::Clone for ClosedCaptionOpacity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0i32);
    pub const FiftyPercent: Self = Self(1i32);
    pub const OneHundredPercent: Self = Self(2i32);
    pub const OneHundredFiftyPercent: Self = Self(3i32);
    pub const TwoHundredPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionSize {}
impl ::core::clone::Clone for ClosedCaptionSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_ClosedCaptioning\"`*"]
#[repr(transparent)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: Self = Self(0i32);
    pub const MonospacedWithSerifs: Self = Self(1i32);
    pub const ProportionalWithSerifs: Self = Self(2i32);
    pub const MonospacedWithoutSerifs: Self = Self(3i32);
    pub const ProportionalWithoutSerifs: Self = Self(4i32);
    pub const Casual: Self = Self(5i32);
    pub const Cursive: Self = Self(6i32);
    pub const SmallCapitals: Self = Self(7i32);
}
impl ::core::marker::Copy for ClosedCaptionStyle {}
impl ::core::clone::Clone for ClosedCaptionStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IClosedCaptionPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionColor) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionOpacity) -> ::windows_sys::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionSize) -> ::windows_sys::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionStyle) -> ::windows_sys::core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionEdgeEffect) -> ::windows_sys::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionColor) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionOpacity) -> ::windows_sys::core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionColor) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClosedCaptionOpacity) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClosedCaptionPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 279584644, data2: 52272, data3: 16705, data4: [181, 3, 82, 114, 40, 158, 12, 32] };
}
