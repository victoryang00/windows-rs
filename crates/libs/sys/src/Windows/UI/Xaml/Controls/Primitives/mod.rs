#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBarButtonTemplateSettings = *mut ::core::ffi::c_void;
pub type AppBarTemplateSettings = *mut ::core::ffi::c_void;
pub type AppBarToggleButtonTemplateSettings = *mut ::core::ffi::c_void;
pub type ButtonBase = *mut ::core::ffi::c_void;
pub type CalendarPanel = *mut ::core::ffi::c_void;
pub type CalendarViewTemplateSettings = *mut ::core::ffi::c_void;
pub type CarouselPanel = *mut ::core::ffi::c_void;
pub type ColorPickerSlider = *mut ::core::ffi::c_void;
pub type ColorSpectrum = *mut ::core::ffi::c_void;
pub type ComboBoxTemplateSettings = *mut ::core::ffi::c_void;
pub type CommandBarFlyoutCommandBar = *mut ::core::ffi::c_void;
pub type CommandBarFlyoutCommandBarTemplateSettings = *mut ::core::ffi::c_void;
pub type CommandBarTemplateSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct ComponentResourceLocation(pub i32);
impl ComponentResourceLocation {
    pub const Application: Self = Self(0i32);
    pub const Nested: Self = Self(1i32);
}
impl ::core::marker::Copy for ComponentResourceLocation {}
impl ::core::clone::Clone for ComponentResourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DragCompletedEventArgs = *mut ::core::ffi::c_void;
pub type DragCompletedEventHandler = *mut ::core::ffi::c_void;
pub type DragDeltaEventArgs = *mut ::core::ffi::c_void;
pub type DragDeltaEventHandler = *mut ::core::ffi::c_void;
pub type DragStartedEventArgs = *mut ::core::ffi::c_void;
pub type DragStartedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct EdgeTransitionLocation(pub i32);
impl EdgeTransitionLocation {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for EdgeTransitionLocation {}
impl ::core::clone::Clone for EdgeTransitionLocation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FlyoutBase = *mut ::core::ffi::c_void;
pub type FlyoutBaseClosingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
    pub const Auto: Self = Self(13i32);
}
impl ::core::marker::Copy for FlyoutPlacementMode {}
impl ::core::clone::Clone for FlyoutPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct FlyoutShowMode(pub i32);
impl FlyoutShowMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Transient: Self = Self(2i32);
    pub const TransientWithDismissOnPointerMoveAway: Self = Self(3i32);
}
impl ::core::marker::Copy for FlyoutShowMode {}
impl ::core::clone::Clone for FlyoutShowMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FlyoutShowOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct GeneratorDirection(pub i32);
impl GeneratorDirection {
    pub const Forward: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
}
impl ::core::marker::Copy for GeneratorDirection {}
impl ::core::clone::Clone for GeneratorDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
pub struct GeneratorPosition {
    pub Index: i32,
    pub Offset: i32,
}
impl ::core::marker::Copy for GeneratorPosition {}
impl ::core::clone::Clone for GeneratorPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GeneratorPositionHelper = *mut ::core::ffi::c_void;
pub type GridViewItemPresenter = *mut ::core::ffi::c_void;
pub type GridViewItemTemplateSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct GroupHeaderPlacement(pub i32);
impl GroupHeaderPlacement {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for GroupHeaderPlacement {}
impl ::core::clone::Clone for GroupHeaderPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAppBarButtonTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ClipRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClipRect: usize,
    pub CompactVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CompactRootMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub MinimalVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinimalRootMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub HiddenVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HiddenRootMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarTemplateSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NegativeCompactVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NegativeMinimalVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NegativeHiddenVerticalDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBarToggleButtonTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButtonBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClickMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ClickMode) -> ::windows_sys::core::HRESULT,
    pub SetClickMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::ClickMode) -> ::windows_sys::core::HRESULT,
    pub IsPointerOver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    Command: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetCommand: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetCommand: usize,
    pub CommandParameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCommandParameter: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Click: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Click: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClick: usize,
}
#[repr(C)]
pub struct IButtonBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IButtonBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClickModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPointerOverProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPressedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICalendarPanel {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICalendarViewTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinViewWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HeaderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay6: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WeekDay7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasMoreContentAfter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasMoreContentBefore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasMoreViews: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClipRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClipRect: usize,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICarouselPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanVerticallyScroll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MakeVisible: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MakeVisible: usize,
}
#[repr(C)]
pub struct ICarouselPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPickerSlider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorChannel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ColorPickerHsvChannel) -> ::windows_sys::core::HRESULT,
    pub SetColorChannel: unsafe extern "system" fn(this: *mut *mut Self, value: super::ColorPickerHsvChannel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPickerSliderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPickerSliderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorChannelProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorSpectrum {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub HsvColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HsvColor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetHsvColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetHsvColor: usize,
    pub MinHue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinHue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MaxHue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxHue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MinSaturation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinSaturation: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MaxSaturation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxSaturation: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMinValue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxValue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ColorSpectrumShape) -> ::windows_sys::core::HRESULT,
    pub SetShape: unsafe extern "system" fn(this: *mut *mut Self, value: super::ColorSpectrumShape) -> ::windows_sys::core::HRESULT,
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ColorSpectrumComponents) -> ::windows_sys::core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(this: *mut *mut Self, value: super::ColorSpectrumComponents) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ColorChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorChanged: usize,
}
#[repr(C)]
pub struct IColorSpectrumFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorSpectrumStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HsvColorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinHueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxHueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinSaturationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxSaturationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ComponentsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropDownOpenedHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DropDownClosedHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DropDownOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SelectedItemDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AnimationDirection) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComboBoxTemplateSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DropDownContentMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarFlyoutCommandBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlyoutTemplateSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarFlyoutCommandBarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OpenAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CloseAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurrentWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandedWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub WidthExpansionDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub WidthExpansionAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub WidthExpansionAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub WidthExpansionMoreButtonAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub WidthExpansionMoreButtonAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandUpOverflowVerticalPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandDownOverflowVerticalPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandUpAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandUpAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandUpAnimationHoldPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandDownAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandDownAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExpandDownAnimationHoldPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentClipRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentClipRect: usize,
    #[cfg(feature = "Foundation")]
    pub OverflowContentClipRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverflowContentClipRect: usize,
}
#[repr(C)]
pub struct ICommandBarTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OverflowContentClipRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverflowContentClipRect: usize,
    pub OverflowContentMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowContentMaxHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowContentHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowContentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NegativeOverflowContentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarTemplateSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverflowContentMaxWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarTemplateSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EffectiveOverflowButtonVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Visibility) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICommandBarTemplateSettings4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverflowContentCompactYTranslation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowContentMinimalYTranslation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OverflowContentHiddenYTranslation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Canceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragCompletedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled: unsafe extern "system" fn(this: *mut *mut Self, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragDeltaEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragDeltaEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithHorizontalChangeAndVerticalChange: unsafe extern "system" fn(this: *mut *mut Self, horizontalchange: f64, verticalchange: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragStartedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithHorizontalOffsetAndVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, horizontaloffset: f64, verticaloffset: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Placement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlyoutPlacementMode) -> ::windows_sys::core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: FlyoutPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub Opening: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opening: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpening: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpening: usize,
    pub ShowAt: unsafe extern "system" fn(this: *mut *mut Self, placementtarget: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBase2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
#[repr(C)]
pub struct IFlyoutBase3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverlayInputPassThroughElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOverlayInputPassThroughElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBase4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryInvokeKeyboardAccelerator: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryInvokeKeyboardAccelerator: usize,
}
#[repr(C)]
pub struct IFlyoutBase5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlyoutShowMode) -> ::windows_sys::core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(this: *mut *mut Self, value: FlyoutShowMode) -> ::windows_sys::core::HRESULT,
    pub InputDevicePrefersPrimaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AreOpenCloseAnimationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreOpenCloseAnimationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowAt: unsafe extern "system" fn(this: *mut *mut Self, placementtarget: *mut ::core::ffi::c_void, showoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBase6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreatePresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseOverrides4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnProcessKeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnProcessKeyboardAccelerators: usize,
}
#[repr(C)]
pub struct IFlyoutBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AttachedFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttachedFlyout: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttachedFlyout: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowAttachedFlyout: unsafe extern "system" fn(this: *mut *mut Self, flyoutowner: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllowFocusWhenDisabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverlayInputPassThroughElementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputDevicePrefersPrimaryCommandsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AreOpenCloseAnimationsEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutBaseStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutShowOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation")]
    pub ExclusionRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExclusionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetExclusionRect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExclusionRect: usize,
    pub ShowMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlyoutShowMode) -> ::windows_sys::core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(this: *mut *mut Self, value: FlyoutShowMode) -> ::windows_sys::core::HRESULT,
    pub Placement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlyoutPlacementMode) -> ::windows_sys::core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: FlyoutPlacementMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFlyoutShowOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeneratorPositionHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGeneratorPositionHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromIndexAndOffset: unsafe extern "system" fn(this: *mut *mut Self, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewItemPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckHintBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckHintBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckSelectingBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckSelectingBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBorderBrush: usize,
    pub SelectedBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterPadding: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewItemPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewItemPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterHorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterHorizontalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterVerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterVerticalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGridViewItemTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DragItemsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItemsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeneratorPosition) -> ::windows_sys::core::HRESULT,
    pub OldPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeneratorPosition) -> ::windows_sys::core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ItemUICount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJumpListItemBackgroundConverter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Enabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetEnabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Disabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Disabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabled: usize,
}
#[repr(C)]
pub struct IJumpListItemBackgroundConverterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJumpListItemForegroundConverter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Enabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetEnabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Disabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Disabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabled: usize,
}
#[repr(C)]
pub struct IJumpListItemForegroundConverterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILayoutInformation {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILayoutInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetLayoutExceptionElement: unsafe extern "system" fn(this: *mut *mut Self, dispatcher: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetLayoutSlot: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLayoutSlot: usize,
}
#[repr(C)]
pub struct ILayoutInformationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAvailableSize: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAvailableSize: usize,
}
#[repr(C)]
pub struct IListViewItemPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckHintBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckHintBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckSelectingBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckSelectingBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBorderBrush: usize,
    pub SelectedBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::VerticalAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterPadding: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterPadding: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PressedBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPressedBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusSecondaryBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusSecondaryBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusSecondaryBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusSecondaryBorderBrush: usize,
    pub CheckMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ListViewItemPresenterCheckMode) -> ::windows_sys::core::HRESULT,
    pub SetCheckMode: unsafe extern "system" fn(this: *mut *mut Self, value: ListViewItemPresenterCheckMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverForeground: usize,
}
#[repr(C)]
pub struct IListViewItemPresenter3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub RevealBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    RevealBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetRevealBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetRevealBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub RevealBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    RevealBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetRevealBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetRevealBorderBrush: usize,
    pub RevealBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetRevealBorderThickness: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub RevealBackgroundShowsAboveContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRevealBackgroundShowsAboveContent: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenter4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxDisabledBorderBrush: usize,
    pub CheckBoxCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetCheckBoxCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SetSelectionIndicatorCornerRadius: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::CornerRadius) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelectionIndicatorVisualEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionIndicatorMode: unsafe extern "system" fn(this: *mut *mut Self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorPointerOverBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorPressedBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorDisabledBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBorderBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBorderBrush: usize,
}
#[repr(C)]
pub struct IListViewItemPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterHorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterHorizontalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterVerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterVerticalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterPaddingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedPressedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PressedBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusSecondaryBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenterStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RevealBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevealBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevealBorderThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RevealBackgroundShowsAboveContentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemPresenterStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDisabledBackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckPressedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckDisabledBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxPointerOverBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxPressedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxDisabledBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxSelectedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxSelectedPointerOverBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxSelectedPressedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxSelectedDisabledBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxPressedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxDisabledBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CheckBoxCornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorCornerRadiusProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorVisualEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorPointerOverBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorPressedBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectionIndicatorDisabledBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedPressedBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedDisabledBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedInnerBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListViewItemTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub DragItemsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoopingSelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldLoop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldLoop: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetItems: usize,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ItemHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
}
#[repr(C)]
pub struct ILoopingSelectorItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILoopingSelectorPanel {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILoopingSelectorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldLoopProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutItemTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMenuFlyoutPresenterTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlyoutContentMinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemPresenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INavigationViewItemPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IconProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOrientedVirtualizingPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanVerticallyScroll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub LineRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PageRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MakeVisible: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MakeVisible: usize,
}
#[repr(C)]
pub struct IOrientedVirtualizingPanelFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPickerFlyoutBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPickerFlyoutBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPickerFlyoutBaseOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnConfirmed: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShouldShowConfirmationButtons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPickerFlyoutBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TitleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotHeaderItem {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotHeaderItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPivotHeaderPanel {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPivotPanel {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPopup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Child: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ChildTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ChildTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetChildTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetChildTransitions: usize,
    pub IsLightDismissEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLightDismissEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[repr(C)]
pub struct IPopup2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::LightDismissOverlayMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopup3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopup4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DesiredPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PopupPlacementMode) -> ::windows_sys::core::HRESULT,
    pub SetDesiredPlacement: unsafe extern "system" fn(this: *mut *mut Self, value: PopupPlacementMode) -> ::windows_sys::core::HRESULT,
    pub ActualPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PopupPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActualPlacementChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualPlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualPlacementChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualPlacementChanged: usize,
}
#[repr(C)]
pub struct IPopupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChildProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChildTransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsLightDismissEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopupStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopupStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPopupStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlacementTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DesiredPlacementProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressBarTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub EllipseDiameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub EllipseAnimationWellPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub EllipseAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ContainerAnimationStartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ContainerAnimationEndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IndicatorLengthDelta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProgressRingTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub EllipseDiameter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    pub MaxSideLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub Minimum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinimum: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Maximum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaximum: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSmallChange: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLargeChange: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
}
#[repr(C)]
pub struct IRangeBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeBaseOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnMinimumChanged: unsafe extern "system" fn(this: *mut *mut Self, oldminimum: f64, newminimum: f64) -> ::windows_sys::core::HRESULT,
    pub OnMaximumChanged: unsafe extern "system" fn(this: *mut *mut Self, oldmaximum: f64, newmaximum: f64) -> ::windows_sys::core::HRESULT,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut *mut Self, oldvalue: f64, newvalue: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeBaseStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinimumProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRangeBaseValueChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRepeatButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRepeatButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DelayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IntervalProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Orientation) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::Orientation) -> ::windows_sys::core::HRESULT,
    pub ViewportSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetViewportSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub IndicatorMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollingIndicatorMode) -> ::windows_sys::core::HRESULT,
    pub SetIndicatorMode: unsafe extern "system" fn(this: *mut *mut Self, value: ScrollingIndicatorMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Scroll: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScroll: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScroll: usize,
}
#[repr(C)]
pub struct IScrollBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ViewportSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndicatorModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ScrollEventType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ScrollEventType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScrollSnapPointsInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreHorizontalSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AreVerticalSnapPointsRegular: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HorizontalSnapPointsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HorizontalSnapPointsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHorizontalSnapPointsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHorizontalSnapPointsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VerticalSnapPointsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerticalSnapPointsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVerticalSnapPointsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVerticalSnapPointsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIrregularSnapPoints: unsafe extern "system" fn(this: *mut *mut Self, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIrregularSnapPoints: usize,
    pub GetRegularSnapPoints: unsafe extern "system" fn(this: *mut *mut Self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSelectedValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValuePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSelectedValuePath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSynchronizedWithCurrentItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSynchronizedWithCurrentItem: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsSynchronizedWithCurrentItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsSynchronizedWithCurrentItem: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
}
#[repr(C)]
pub struct ISelectorFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISelectorItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectorItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectorItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelectedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValueProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedValuePathProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsSynchronizedWithCurrentItemProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIsSelectionActive: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISettingsFlyoutTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderBackground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderForeground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub IconSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    IconSource: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ContentTransitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ContentTransitions: usize,
}
#[repr(C)]
pub struct ISplitViewTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub OpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NegativeOpenPaneLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OpenPaneLengthMinusCompactLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NegativeOpenPaneLengthMinusCompactLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OpenPaneGridLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::GridLength) -> ::windows_sys::core::HRESULT,
    pub CompactPaneGridLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::GridLength) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThumb {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDragging: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DragStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragStarted: usize,
    #[cfg(feature = "Foundation")]
    pub DragDelta: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragDelta: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragDelta: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragDelta: usize,
    #[cfg(feature = "Foundation")]
    pub DragCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragCompleted: usize,
    pub CancelDrag: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThumbStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDraggingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITickBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
}
#[repr(C)]
pub struct ITickBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleButton {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsChecked: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsChecked: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsChecked: usize,
    pub IsThreeState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsThreeState: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Checked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Checked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChecked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChecked: usize,
    #[cfg(feature = "Foundation")]
    pub Unchecked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Unchecked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnchecked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnchecked: usize,
    #[cfg(feature = "Foundation")]
    pub Indeterminate: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Indeterminate: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIndeterminate: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIndeterminate: usize,
}
#[repr(C)]
pub struct IToggleButtonFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleButtonOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnToggle: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleButtonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCheckedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsThreeStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToggleSwitchTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub KnobCurrentToOnOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub KnobCurrentToOffOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub KnobOnToOffOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub KnobOffToOnOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurtainCurrentToOnOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurtainCurrentToOffOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurtainOnToOffOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CurtainOffToOnOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IToolTipTemplateSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
pub type ItemsChangedEventArgs = *mut ::core::ffi::c_void;
pub type ItemsChangedEventHandler = *mut ::core::ffi::c_void;
pub type JumpListItemBackgroundConverter = *mut ::core::ffi::c_void;
pub type JumpListItemForegroundConverter = *mut ::core::ffi::c_void;
pub type LayoutInformation = *mut ::core::ffi::c_void;
pub type ListViewItemPresenter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct ListViewItemPresenterCheckMode(pub i32);
impl ListViewItemPresenterCheckMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterCheckMode {}
impl ::core::clone::Clone for ListViewItemPresenterCheckMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct ListViewItemPresenterSelectionIndicatorMode(pub i32);
impl ListViewItemPresenterSelectionIndicatorMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterSelectionIndicatorMode {}
impl ::core::clone::Clone for ListViewItemPresenterSelectionIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ListViewItemTemplateSettings = *mut ::core::ffi::c_void;
pub type LoopingSelector = *mut ::core::ffi::c_void;
pub type LoopingSelectorItem = *mut ::core::ffi::c_void;
pub type LoopingSelectorPanel = *mut ::core::ffi::c_void;
pub type MenuFlyoutItemTemplateSettings = *mut ::core::ffi::c_void;
pub type MenuFlyoutPresenterTemplateSettings = *mut ::core::ffi::c_void;
pub type NavigationViewItemPresenter = *mut ::core::ffi::c_void;
pub type OrientedVirtualizingPanel = *mut ::core::ffi::c_void;
pub type PickerFlyoutBase = *mut ::core::ffi::c_void;
pub type PivotHeaderItem = *mut ::core::ffi::c_void;
pub type PivotHeaderPanel = *mut ::core::ffi::c_void;
pub type PivotPanel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(9i32);
    pub const Mouse: Self = Self(7i32);
    pub const Right: Self = Self(4i32);
    pub const Top: Self = Self(10i32);
}
impl ::core::marker::Copy for PlacementMode {}
impl ::core::clone::Clone for PlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Popup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct PopupPlacementMode(pub i32);
impl PopupPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
}
impl ::core::marker::Copy for PopupPlacementMode {}
impl ::core::clone::Clone for PopupPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProgressBarTemplateSettings = *mut ::core::ffi::c_void;
pub type ProgressRingTemplateSettings = *mut ::core::ffi::c_void;
pub type RangeBase = *mut ::core::ffi::c_void;
pub type RangeBaseValueChangedEventArgs = *mut ::core::ffi::c_void;
pub type RangeBaseValueChangedEventHandler = *mut ::core::ffi::c_void;
pub type RepeatButton = *mut ::core::ffi::c_void;
pub type ScrollBar = *mut ::core::ffi::c_void;
pub type ScrollEventArgs = *mut ::core::ffi::c_void;
pub type ScrollEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct ScrollEventType(pub i32);
impl ScrollEventType {
    pub const SmallDecrement: Self = Self(0i32);
    pub const SmallIncrement: Self = Self(1i32);
    pub const LargeDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const ThumbPosition: Self = Self(4i32);
    pub const ThumbTrack: Self = Self(5i32);
    pub const First: Self = Self(6i32);
    pub const Last: Self = Self(7i32);
    pub const EndScroll: Self = Self(8i32);
}
impl ::core::marker::Copy for ScrollEventType {}
impl ::core::clone::Clone for ScrollEventType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: Self = Self(0i32);
    pub const TouchIndicator: Self = Self(1i32);
    pub const MouseIndicator: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollingIndicatorMode {}
impl ::core::clone::Clone for ScrollingIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Selector = *mut ::core::ffi::c_void;
pub type SelectorItem = *mut ::core::ffi::c_void;
pub type SettingsFlyoutTemplateSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct SliderSnapsTo(pub i32);
impl SliderSnapsTo {
    pub const StepValues: Self = Self(0i32);
    pub const Ticks: Self = Self(1i32);
}
impl ::core::marker::Copy for SliderSnapsTo {}
impl ::core::clone::Clone for SliderSnapsTo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct SnapPointsAlignment(pub i32);
impl SnapPointsAlignment {
    pub const Near: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Far: Self = Self(2i32);
}
impl ::core::marker::Copy for SnapPointsAlignment {}
impl ::core::clone::Clone for SnapPointsAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SplitViewTemplateSettings = *mut ::core::ffi::c_void;
pub type Thumb = *mut ::core::ffi::c_void;
pub type TickBar = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
#[repr(transparent)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const Outside: Self = Self(3i32);
    pub const Inline: Self = Self(4i32);
}
impl ::core::marker::Copy for TickPlacement {}
impl ::core::clone::Clone for TickPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ToggleButton = *mut ::core::ffi::c_void;
pub type ToggleSwitchTemplateSettings = *mut ::core::ffi::c_void;
pub type ToolTipTemplateSettings = *mut ::core::ffi::c_void;
