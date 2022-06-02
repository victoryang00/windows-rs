#[cfg(feature = "UI_Xaml_Automation")]
pub mod Automation;
#[cfg(feature = "UI_Xaml_Controls")]
pub mod Controls;
#[cfg(feature = "UI_Xaml_Core")]
pub mod Core;
#[cfg(feature = "UI_Xaml_Data")]
pub mod Data;
#[cfg(feature = "UI_Xaml_Documents")]
pub mod Documents;
#[cfg(feature = "UI_Xaml_Hosting")]
pub mod Hosting;
#[cfg(feature = "UI_Xaml_Input")]
pub mod Input;
#[cfg(feature = "UI_Xaml_Interop")]
pub mod Interop;
#[cfg(feature = "UI_Xaml_Markup")]
pub mod Markup;
#[cfg(feature = "UI_Xaml_Media")]
pub mod Media;
#[cfg(feature = "UI_Xaml_Navigation")]
pub mod Navigation;
#[cfg(feature = "UI_Xaml_Printing")]
pub mod Printing;
#[cfg(feature = "UI_Xaml_Resources")]
pub mod Resources;
#[cfg(feature = "UI_Xaml_Shapes")]
pub mod Shapes;
pub type AdaptiveTrigger = *mut ::core::ffi::c_void;
pub type Application = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ApplicationHighContrastAdjustment(pub u32);
impl ApplicationHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ApplicationHighContrastAdjustment {}
impl ::core::clone::Clone for ApplicationHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ApplicationInitializationCallback = *mut ::core::ffi::c_void;
pub type ApplicationInitializationCallbackParams = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ApplicationRequiresPointerMode(pub i32);
impl ApplicationRequiresPointerMode {
    pub const Auto: Self = Self(0i32);
    pub const WhenRequested: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationRequiresPointerMode {}
impl ::core::clone::Clone for ApplicationRequiresPointerMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ApplicationTheme(pub i32);
impl ApplicationTheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationTheme {}
impl ::core::clone::Clone for ApplicationTheme {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct AutomationTextAttributesEnum(pub i32);
impl AutomationTextAttributesEnum {
    pub const AnimationStyleAttribute: Self = Self(40000i32);
    pub const BackgroundColorAttribute: Self = Self(40001i32);
    pub const BulletStyleAttribute: Self = Self(40002i32);
    pub const CapStyleAttribute: Self = Self(40003i32);
    pub const CultureAttribute: Self = Self(40004i32);
    pub const FontNameAttribute: Self = Self(40005i32);
    pub const FontSizeAttribute: Self = Self(40006i32);
    pub const FontWeightAttribute: Self = Self(40007i32);
    pub const ForegroundColorAttribute: Self = Self(40008i32);
    pub const HorizontalTextAlignmentAttribute: Self = Self(40009i32);
    pub const IndentationFirstLineAttribute: Self = Self(40010i32);
    pub const IndentationLeadingAttribute: Self = Self(40011i32);
    pub const IndentationTrailingAttribute: Self = Self(40012i32);
    pub const IsHiddenAttribute: Self = Self(40013i32);
    pub const IsItalicAttribute: Self = Self(40014i32);
    pub const IsReadOnlyAttribute: Self = Self(40015i32);
    pub const IsSubscriptAttribute: Self = Self(40016i32);
    pub const IsSuperscriptAttribute: Self = Self(40017i32);
    pub const MarginBottomAttribute: Self = Self(40018i32);
    pub const MarginLeadingAttribute: Self = Self(40019i32);
    pub const MarginTopAttribute: Self = Self(40020i32);
    pub const MarginTrailingAttribute: Self = Self(40021i32);
    pub const OutlineStylesAttribute: Self = Self(40022i32);
    pub const OverlineColorAttribute: Self = Self(40023i32);
    pub const OverlineStyleAttribute: Self = Self(40024i32);
    pub const StrikethroughColorAttribute: Self = Self(40025i32);
    pub const StrikethroughStyleAttribute: Self = Self(40026i32);
    pub const TabsAttribute: Self = Self(40027i32);
    pub const TextFlowDirectionsAttribute: Self = Self(40028i32);
    pub const UnderlineColorAttribute: Self = Self(40029i32);
    pub const UnderlineStyleAttribute: Self = Self(40030i32);
    pub const AnnotationTypesAttribute: Self = Self(40031i32);
    pub const AnnotationObjectsAttribute: Self = Self(40032i32);
    pub const StyleNameAttribute: Self = Self(40033i32);
    pub const StyleIdAttribute: Self = Self(40034i32);
    pub const LinkAttribute: Self = Self(40035i32);
    pub const IsActiveAttribute: Self = Self(40036i32);
    pub const SelectionActiveEndAttribute: Self = Self(40037i32);
    pub const CaretPositionAttribute: Self = Self(40038i32);
    pub const CaretBidiModeAttribute: Self = Self(40039i32);
}
impl ::core::marker::Copy for AutomationTextAttributesEnum {}
impl ::core::clone::Clone for AutomationTextAttributesEnum {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BindingFailedEventArgs = *mut ::core::ffi::c_void;
pub type BindingFailedEventHandler = *mut ::core::ffi::c_void;
pub type BringIntoViewOptions = *mut ::core::ffi::c_void;
pub type BringIntoViewRequestedEventArgs = *mut ::core::ffi::c_void;
pub type BrushTransition = *mut ::core::ffi::c_void;
pub type ColorPaletteResources = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml\"`*"]
pub struct CornerRadius {
    pub TopLeft: f64,
    pub TopRight: f64,
    pub BottomRight: f64,
    pub BottomLeft: f64,
}
impl ::core::marker::Copy for CornerRadius {}
impl ::core::clone::Clone for CornerRadius {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CornerRadiusHelper = *mut ::core::ffi::c_void;
pub type CreateDefaultValueCallback = *mut ::core::ffi::c_void;
pub type DataContextChangedEventArgs = *mut ::core::ffi::c_void;
pub type DataTemplate = *mut ::core::ffi::c_void;
pub type DataTemplateKey = *mut ::core::ffi::c_void;
pub type DebugSettings = *mut ::core::ffi::c_void;
pub type DependencyObject = *mut ::core::ffi::c_void;
pub type DependencyObjectCollection = *mut ::core::ffi::c_void;
pub type DependencyProperty = *mut ::core::ffi::c_void;
pub type DependencyPropertyChangedCallback = *mut ::core::ffi::c_void;
pub type DependencyPropertyChangedEventArgs = *mut ::core::ffi::c_void;
pub type DependencyPropertyChangedEventHandler = *mut ::core::ffi::c_void;
pub type DispatcherTimer = *mut ::core::ffi::c_void;
pub type DragEventArgs = *mut ::core::ffi::c_void;
pub type DragEventHandler = *mut ::core::ffi::c_void;
pub type DragOperationDeferral = *mut ::core::ffi::c_void;
pub type DragStartingEventArgs = *mut ::core::ffi::c_void;
pub type DragUI = *mut ::core::ffi::c_void;
pub type DragUIOverride = *mut ::core::ffi::c_void;
pub type DropCompletedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct Duration {
    pub TimeSpan: super::super::Foundation::TimeSpan,
    pub Type: DurationType,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for Duration {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for Duration {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DurationHelper = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct DurationType(pub i32);
impl DurationType {
    pub const Automatic: Self = Self(0i32);
    pub const TimeSpan: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for DurationType {}
impl ::core::clone::Clone for DurationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EffectiveViewportChangedEventArgs = *mut ::core::ffi::c_void;
pub type ElementFactoryGetArgs = *mut ::core::ffi::c_void;
pub type ElementFactoryRecycleArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementHighContrastAdjustment(pub u32);
impl ElementHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Application: Self = Self(2147483648u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ElementHighContrastAdjustment {}
impl ::core::clone::Clone for ElementHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementSoundKind(pub i32);
impl ElementSoundKind {
    pub const Focus: Self = Self(0i32);
    pub const Invoke: Self = Self(1i32);
    pub const Show: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
    pub const MovePrevious: Self = Self(4i32);
    pub const MoveNext: Self = Self(5i32);
    pub const GoBack: Self = Self(6i32);
}
impl ::core::marker::Copy for ElementSoundKind {}
impl ::core::clone::Clone for ElementSoundKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementSoundMode(pub i32);
impl ElementSoundMode {
    pub const Default: Self = Self(0i32);
    pub const FocusOnly: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundMode {}
impl ::core::clone::Clone for ElementSoundMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ElementSoundPlayer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementSoundPlayerState(pub i32);
impl ElementSoundPlayerState {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundPlayerState {}
impl ::core::clone::Clone for ElementSoundPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementSpatialAudioMode(pub i32);
impl ElementSpatialAudioMode {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSpatialAudioMode {}
impl ::core::clone::Clone for ElementSpatialAudioMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementTheme {}
impl ::core::clone::Clone for ElementTheme {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EnteredBackgroundEventHandler = *mut ::core::ffi::c_void;
pub type EventTrigger = *mut ::core::ffi::c_void;
pub type ExceptionRoutedEventArgs = *mut ::core::ffi::c_void;
pub type ExceptionRoutedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FlowDirection(pub i32);
impl FlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for FlowDirection {}
impl ::core::clone::Clone for FlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FocusState(pub i32);
impl FocusState {
    pub const Unfocused: Self = Self(0i32);
    pub const Pointer: Self = Self(1i32);
    pub const Keyboard: Self = Self(2i32);
    pub const Programmatic: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusState {}
impl ::core::clone::Clone for FocusState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FocusVisualKind(pub i32);
impl FocusVisualKind {
    pub const DottedLine: Self = Self(0i32);
    pub const HighVisibility: Self = Self(1i32);
    pub const Reveal: Self = Self(2i32);
}
impl ::core::marker::Copy for FocusVisualKind {}
impl ::core::clone::Clone for FocusVisualKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontCapitals(pub i32);
impl FontCapitals {
    pub const Normal: Self = Self(0i32);
    pub const AllSmallCaps: Self = Self(1i32);
    pub const SmallCaps: Self = Self(2i32);
    pub const AllPetiteCaps: Self = Self(3i32);
    pub const PetiteCaps: Self = Self(4i32);
    pub const Unicase: Self = Self(5i32);
    pub const Titling: Self = Self(6i32);
}
impl ::core::marker::Copy for FontCapitals {}
impl ::core::clone::Clone for FontCapitals {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontEastAsianLanguage(pub i32);
impl FontEastAsianLanguage {
    pub const Normal: Self = Self(0i32);
    pub const HojoKanji: Self = Self(1i32);
    pub const Jis04: Self = Self(2i32);
    pub const Jis78: Self = Self(3i32);
    pub const Jis83: Self = Self(4i32);
    pub const Jis90: Self = Self(5i32);
    pub const NlcKanji: Self = Self(6i32);
    pub const Simplified: Self = Self(7i32);
    pub const Traditional: Self = Self(8i32);
    pub const TraditionalNames: Self = Self(9i32);
}
impl ::core::marker::Copy for FontEastAsianLanguage {}
impl ::core::clone::Clone for FontEastAsianLanguage {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontEastAsianWidths(pub i32);
impl FontEastAsianWidths {
    pub const Normal: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const Half: Self = Self(2i32);
    pub const Proportional: Self = Self(3i32);
    pub const Quarter: Self = Self(4i32);
    pub const Third: Self = Self(5i32);
}
impl ::core::marker::Copy for FontEastAsianWidths {}
impl ::core::clone::Clone for FontEastAsianWidths {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontFraction(pub i32);
impl FontFraction {
    pub const Normal: Self = Self(0i32);
    pub const Stacked: Self = Self(1i32);
    pub const Slashed: Self = Self(2i32);
}
impl ::core::marker::Copy for FontFraction {}
impl ::core::clone::Clone for FontFraction {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontNumeralAlignment(pub i32);
impl FontNumeralAlignment {
    pub const Normal: Self = Self(0i32);
    pub const Proportional: Self = Self(1i32);
    pub const Tabular: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralAlignment {}
impl ::core::clone::Clone for FontNumeralAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontNumeralStyle(pub i32);
impl FontNumeralStyle {
    pub const Normal: Self = Self(0i32);
    pub const Lining: Self = Self(1i32);
    pub const OldStyle: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralStyle {}
impl ::core::clone::Clone for FontNumeralStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct FontVariants(pub i32);
impl FontVariants {
    pub const Normal: Self = Self(0i32);
    pub const Superscript: Self = Self(1i32);
    pub const Subscript: Self = Self(2i32);
    pub const Ordinal: Self = Self(3i32);
    pub const Inferior: Self = Self(4i32);
    pub const Ruby: Self = Self(5i32);
}
impl ::core::marker::Copy for FontVariants {}
impl ::core::clone::Clone for FontVariants {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FrameworkElement = *mut ::core::ffi::c_void;
pub type FrameworkTemplate = *mut ::core::ffi::c_void;
pub type FrameworkView = *mut ::core::ffi::c_void;
pub type FrameworkViewSource = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml\"`*"]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl ::core::marker::Copy for GridLength {}
impl ::core::clone::Clone for GridLength {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GridLengthHelper = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0i32);
    pub const Pixel: Self = Self(1i32);
    pub const Star: Self = Self(2i32);
}
impl ::core::marker::Copy for GridUnitType {}
impl ::core::clone::Clone for GridUnitType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for HorizontalAlignment {}
impl ::core::clone::Clone for HorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdaptiveTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinWindowWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinWindowWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinWindowHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinWindowHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveTriggerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub MinWindowWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinWindowHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplication {
    pub base__: ::windows_sys::core::IInspectable,
    pub Resources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetResources: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DebugSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestedTheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationTheme) -> ::windows_sys::core::HRESULT,
    pub SetRequestedTheme: unsafe extern "system" fn(this: *mut *mut Self, value: ApplicationTheme) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnhandledException: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnhandledException: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnhandledException: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnhandledException: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub Suspending: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    pub Exit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplication2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FocusVisualKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusVisualKind) -> ::windows_sys::core::HRESULT,
    pub SetFocusVisualKind: unsafe extern "system" fn(this: *mut *mut Self, value: FocusVisualKind) -> ::windows_sys::core::HRESULT,
    pub RequiresPointerMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationRequiresPointerMode) -> ::windows_sys::core::HRESULT,
    pub SetRequiresPointerMode: unsafe extern "system" fn(this: *mut *mut Self, value: ApplicationRequiresPointerMode) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
}
#[repr(C)]
pub struct IApplication3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, value: ApplicationHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationInitializationCallbackParams {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IApplicationOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnLaunched: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnLaunched: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnFileActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnFileActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnSearchActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnSearchActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnShareTargetActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnShareTargetActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnFileOpenPickerActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnFileOpenPickerActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnFileSavePickerActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnFileSavePickerActivated: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnCachedFileUpdaterActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnCachedFileUpdaterActivated: usize,
    pub OnWindowCreated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IApplicationOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub OnBackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    OnBackgroundActivated: usize,
}
#[repr(C)]
pub struct IApplicationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, callback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadComponent: unsafe extern "system" fn(this: *mut *mut Self, component: *mut ::core::ffi::c_void, resourcelocator: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadComponent: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub LoadComponentWithResourceLocation: unsafe extern "system" fn(this: *mut *mut Self, component: *mut ::core::ffi::c_void, resourcelocator: *mut ::core::ffi::c_void, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    LoadComponentWithResourceLocation: usize,
}
#[repr(C)]
pub struct IBindingFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBringIntoViewOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnimationDesired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAnimationDesired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TargetRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetTargetRect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTargetRect: usize,
}
#[repr(C)]
pub struct IBringIntoViewOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBringIntoViewRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AnimationDesired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAnimationDesired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TargetRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetTargetRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTargetRect: usize,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBrushTransition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
}
#[repr(C)]
pub struct IBrushTransitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IColorPaletteResources {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AltHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetAltHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAltHigh: usize,
    #[cfg(feature = "Foundation")]
    pub AltLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetAltLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAltLow: usize,
    #[cfg(feature = "Foundation")]
    pub AltMedium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltMedium: usize,
    #[cfg(feature = "Foundation")]
    pub SetAltMedium: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAltMedium: usize,
    #[cfg(feature = "Foundation")]
    pub AltMediumHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltMediumHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetAltMediumHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAltMediumHigh: usize,
    #[cfg(feature = "Foundation")]
    pub AltMediumLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetAltMediumLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAltMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub BaseHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseHigh: usize,
    #[cfg(feature = "Foundation")]
    pub BaseLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseLow: usize,
    #[cfg(feature = "Foundation")]
    pub BaseMedium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseMedium: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseMedium: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseMedium: usize,
    #[cfg(feature = "Foundation")]
    pub BaseMediumHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseMediumHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseMediumHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseMediumHigh: usize,
    #[cfg(feature = "Foundation")]
    pub BaseMediumLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetBaseMediumLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBaseMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeAltLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeAltLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeAltLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeAltLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeBlackHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeBlackHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeBlackHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeBlackHigh: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeBlackLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeBlackLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeBlackLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeBlackLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeBlackMediumLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeBlackMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeBlackMediumLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeBlackMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeBlackMedium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeBlackMedium: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeBlackMedium: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeBlackMedium: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeDisabledHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeDisabledHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeDisabledHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeDisabledHigh: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeDisabledLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeDisabledLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeDisabledLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeDisabledLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeHigh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeHigh: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeMedium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeMedium: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeMedium: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeMedium: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeMediumLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeMediumLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeMediumLow: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeWhite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeWhite: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeWhite: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeWhite: usize,
    #[cfg(feature = "Foundation")]
    pub ChromeGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChromeGray: usize,
    #[cfg(feature = "Foundation")]
    pub SetChromeGray: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetChromeGray: usize,
    #[cfg(feature = "Foundation")]
    pub ListLow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ListLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetListLow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetListLow: usize,
    #[cfg(feature = "Foundation")]
    pub ListMedium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ListMedium: usize,
    #[cfg(feature = "Foundation")]
    pub SetListMedium: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetListMedium: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorText: usize,
    #[cfg(feature = "Foundation")]
    pub SetErrorText: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetErrorText: usize,
    #[cfg(feature = "Foundation")]
    pub Accent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Accent: usize,
    #[cfg(feature = "Foundation")]
    pub SetAccent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAccent: usize,
}
#[repr(C)]
pub struct IColorPaletteResourcesFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICornerRadiusHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICornerRadiusHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromRadii: unsafe extern "system" fn(this: *mut *mut Self, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64, result__: *mut CornerRadius) -> ::windows_sys::core::HRESULT,
    pub FromUniformRadius: unsafe extern "system" fn(this: *mut *mut Self, uniformradius: f64, result__: *mut CornerRadius) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataContextChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplate {
    pub base__: ::windows_sys::core::IInspectable,
    pub LoadContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateExtension {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResetTemplate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ProcessBinding: unsafe extern "system" fn(this: *mut *mut Self, phase: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub ProcessBindings: unsafe extern "system" fn(this: *mut *mut Self, arg: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    ProcessBindings: usize,
}
#[repr(C)]
pub struct IDataTemplateFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateKey {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateKeyFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithType: unsafe extern "system" fn(this: *mut *mut Self, datatype: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataTemplateStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtensionInstanceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetExtensionInstance: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetExtensionInstance: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDebugSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableFrameRateCounter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableFrameRateCounter: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsBindingTracingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBindingTracingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsOverdrawHeatMapEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOverdrawHeatMapEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BindingFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindingFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBindingFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBindingFailed: usize,
}
#[repr(C)]
pub struct IDebugSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableRedrawRegions: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDebugSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextPerformanceVisualizationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextPerformanceVisualizationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDebugSettings4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FailFastOnErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFailFastOnErrors: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDependencyObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClearValue: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReadLocalValue: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAnimationBaseValue: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[repr(C)]
pub struct IDependencyObject2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterPropertyChangedCallback: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub UnregisterPropertyChangedCallback: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, token: i64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDependencyObjectCollectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IDependencyObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDependencyProperty {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub GetMetadata: unsafe extern "system" fn(this: *mut *mut Self, fortype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    GetMetadata: usize,
}
#[repr(C)]
pub struct IDependencyPropertyChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDependencyPropertyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnsetValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, typemetadata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Register: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub RegisterAttached: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, defaultmetadata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    RegisterAttached: usize,
}
#[repr(C)]
pub struct IDispatcherTimer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDispatcherTimerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Data: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    SetData: usize,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IDragEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub DataView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    DataView: usize,
    pub DragUIOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
    pub Modifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_DragDrop"))]
    Modifiers: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub AcceptedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    AcceptedOperation: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub SetAcceptedOperation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    SetAcceptedOperation: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    AllowedOperations: usize,
}
#[repr(C)]
pub struct IDragOperationDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    Data: usize,
    pub DragUI: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeto: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[repr(C)]
pub struct IDragStartingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    AllowedOperations: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    SetAllowedOperations: usize,
}
#[repr(C)]
pub struct IDragUI {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media_Imaging")]
    pub SetContentFromBitmapImage: unsafe extern "system" fn(this: *mut *mut Self, bitmapimage: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Imaging"))]
    SetContentFromBitmapImage: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Imaging"))]
    pub SetContentFromBitmapImageWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, bitmapimage: *mut ::core::ffi::c_void, anchorpoint: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media_Imaging")))]
    SetContentFromBitmapImageWithAnchorPoint: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub SetContentFromDataPackage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDragUIOverride {
    pub base__: ::windows_sys::core::IInspectable,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsContentVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Imaging")]
    pub SetContentFromBitmapImage: unsafe extern "system" fn(this: *mut *mut Self, bitmapimage: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Imaging"))]
    SetContentFromBitmapImage: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Imaging"))]
    pub SetContentFromBitmapImageWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, bitmapimage: *mut ::core::ffi::c_void, anchorpoint: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Media_Imaging")))]
    SetContentFromBitmapImageWithAnchorPoint: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
}
#[repr(C)]
pub struct IDropCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub DropResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    DropResult: usize,
}
#[repr(C)]
pub struct IDurationHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDurationHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Automatic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Automatic: usize,
    #[cfg(feature = "Foundation")]
    pub Forever: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Forever: usize,
    #[cfg(feature = "Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut *mut Self, duration1: Duration, duration2: Duration, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Compare: usize,
    #[cfg(feature = "Foundation")]
    pub FromTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, timespan: super::super::Foundation::TimeSpan, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromTimeSpan: usize,
    #[cfg(feature = "Foundation")]
    pub GetHasTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, target: Duration, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHasTimeSpan: usize,
    #[cfg(feature = "Foundation")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Add: usize,
    #[cfg(feature = "Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: Duration, value: Duration, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Equals: usize,
    #[cfg(feature = "Foundation")]
    pub Subtract: unsafe extern "system" fn(this: *mut *mut Self, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Subtract: usize,
}
#[repr(C)]
pub struct IEffectiveViewportChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub EffectiveViewport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EffectiveViewport: usize,
    #[cfg(feature = "Foundation")]
    pub MaxViewport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxViewport: usize,
    pub BringIntoViewDistanceX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub BringIntoViewDistanceY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetElement: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecycleElement: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementFactoryGetArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementFactoryGetArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementFactoryRecycleArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Element: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementFactoryRecycleArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementSoundPlayer {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IElementSoundPlayerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ElementSoundPlayerState) -> ::windows_sys::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut *mut Self, value: ElementSoundPlayerState) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self, sound: ElementSoundKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementSoundPlayerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpatialAudioMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ElementSpatialAudioMode) -> ::windows_sys::core::HRESULT,
    pub SetSpatialAudioMode: unsafe extern "system" fn(this: *mut *mut Self, value: ElementSpatialAudioMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEventTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoutedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoutedEvent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
}
#[repr(C)]
pub struct IExceptionRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IExceptionRoutedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFrameworkElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Triggers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Triggers: usize,
    pub Resources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetResources: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ActualWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ActualHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MinHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMinHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub HorizontalAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: HorizontalAlignment) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: VerticalAlignment) -> ::windows_sys::core::HRESULT,
    pub Margin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(this: *mut *mut Self, value: Thickness) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BaseUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseUri: usize,
    pub DataContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDataContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Style: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FlowDirection) -> ::windows_sys::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self, value: FlowDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Loaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Loaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub Unloaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Unloaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnloaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnloaded: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub LayoutUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutUpdated: usize,
    pub FindName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Data")]
    pub SetBinding: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, binding: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    SetBinding: usize,
}
#[repr(C)]
pub struct IFrameworkElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestedTheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ElementTheme) -> ::windows_sys::core::HRESULT,
    pub SetRequestedTheme: unsafe extern "system" fn(this: *mut *mut Self, value: ElementTheme) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataContextChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataContextChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataContextChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataContextChanged: usize,
    #[cfg(feature = "UI_Xaml_Data")]
    pub GetBindingExpression: unsafe extern "system" fn(this: *mut *mut Self, dp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Data"))]
    GetBindingExpression: usize,
}
#[repr(C)]
pub struct IFrameworkElement3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Loading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Loading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLoading: usize,
}
#[repr(C)]
pub struct IFrameworkElement4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FocusVisualMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
    pub SetFocusVisualMargin: unsafe extern "system" fn(this: *mut *mut Self, value: Thickness) -> ::windows_sys::core::HRESULT,
    pub FocusVisualSecondaryThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
    pub SetFocusVisualSecondaryThickness: unsafe extern "system" fn(this: *mut *mut Self, value: Thickness) -> ::windows_sys::core::HRESULT,
    pub FocusVisualPrimaryThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
    pub SetFocusVisualPrimaryThickness: unsafe extern "system" fn(this: *mut *mut Self, value: Thickness) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusVisualSecondaryBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusVisualSecondaryBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusVisualSecondaryBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusVisualSecondaryBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusVisualPrimaryBrush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusVisualPrimaryBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusVisualPrimaryBrush: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusVisualPrimaryBrush: usize,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElement6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActualTheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ElementTheme) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActualThemeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualThemeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActualThemeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActualThemeChanged: usize,
}
#[repr(C)]
pub struct IFrameworkElement7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsLoaded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EffectiveViewportChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EffectiveViewportChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEffectiveViewportChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEffectiveViewportChanged: usize,
}
#[repr(C)]
pub struct IFrameworkElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MeasureOverride: unsafe extern "system" fn(this: *mut *mut Self, availablesize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MeasureOverride: usize,
    #[cfg(feature = "Foundation")]
    pub ArrangeOverride: unsafe extern "system" fn(this: *mut *mut Self, finalsize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArrangeOverride: usize,
    pub OnApplyTemplate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementOverrides2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GoToElementStateCore: unsafe extern "system" fn(this: *mut *mut Self, statename: ::windows_sys::core::HSTRING, usetransitions: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementProtected7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InvalidateViewport: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TagProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActualWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActualHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxWidthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MinHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HorizontalAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VerticalAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DataContextProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlowDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestedThemeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusVisualMarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusVisualSecondaryThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusVisualPrimaryThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusVisualSecondaryBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusVisualPrimaryBrushProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllowFocusWhenDisabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeferTree: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkElementStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActualThemeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkTemplate {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFrameworkTemplateFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameworkView {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFrameworkViewSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridLengthHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IGridLengthHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GridLength) -> ::windows_sys::core::HRESULT,
    pub FromPixels: unsafe extern "system" fn(this: *mut *mut Self, pixels: f64, result__: *mut GridLength) -> ::windows_sys::core::HRESULT,
    pub FromValueAndType: unsafe extern "system" fn(this: *mut *mut Self, value: f64, r#type: GridUnitType, result__: *mut GridLength) -> ::windows_sys::core::HRESULT,
    pub GetIsAbsolute: unsafe extern "system" fn(this: *mut *mut Self, target: GridLength, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetIsAuto: unsafe extern "system" fn(this: *mut *mut Self, target: GridLength, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetIsStar: unsafe extern "system" fn(this: *mut *mut Self, target: GridLength, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: GridLength, value: GridLength, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaFailedRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorTrace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPointHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPointHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromCoordinates: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromCoordinates: usize,
}
#[repr(C)]
pub struct IPropertyMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub DefaultValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDefaultValueCallback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPropertyMetadataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithDefaultValue: unsafe extern "system" fn(this: *mut *mut Self, defaultvalue: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceWithDefaultValueAndCallback: unsafe extern "system" fn(this: *mut *mut Self, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPropertyMetadataStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithDefaultValue: unsafe extern "system" fn(this: *mut *mut Self, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithDefaultValueAndCallback: unsafe extern "system" fn(this: *mut *mut Self, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithFactory: unsafe extern "system" fn(this: *mut *mut Self, createdefaultvaluecallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithFactoryAndCallback: unsafe extern "system" fn(this: *mut *mut Self, createdefaultvaluecallback: *mut ::core::ffi::c_void, propertychangedcallback: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPropertyPath {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPropertyPathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRectHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRectHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Empty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Empty: usize,
    #[cfg(feature = "Foundation")]
    pub FromCoordinatesAndDimensions: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, width: f32, height: f32, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromCoordinatesAndDimensions: usize,
    #[cfg(feature = "Foundation")]
    pub FromPoints: unsafe extern "system" fn(this: *mut *mut Self, point1: super::super::Foundation::Point, point2: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromPoints: usize,
    #[cfg(feature = "Foundation")]
    pub FromLocationAndSize: unsafe extern "system" fn(this: *mut *mut Self, location: super::super::Foundation::Point, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromLocationAndSize: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsEmpty: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsEmpty: usize,
    #[cfg(feature = "Foundation")]
    pub GetBottom: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBottom: usize,
    #[cfg(feature = "Foundation")]
    pub GetLeft: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLeft: usize,
    #[cfg(feature = "Foundation")]
    pub GetRight: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRight: usize,
    #[cfg(feature = "Foundation")]
    pub GetTop: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTop: usize,
    #[cfg(feature = "Foundation")]
    pub Contains: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Contains: usize,
    #[cfg(feature = "Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, value: super::super::Foundation::Rect, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Equals: usize,
    #[cfg(feature = "Foundation")]
    pub Intersect: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Intersect: usize,
    #[cfg(feature = "Foundation")]
    pub UnionWithPoint: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnionWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub UnionWithRect: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnionWithRect: usize,
}
#[repr(C)]
pub struct IResourceDictionary {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MergedDictionaries: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MergedDictionaries: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ThemeDictionaries: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ThemeDictionaries: usize,
}
#[repr(C)]
pub struct IResourceDictionaryFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRoutedEvent {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRoutedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OriginalSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRoutedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScalarTransition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
}
#[repr(C)]
pub struct IScalarTransitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetterBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSealed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetterBaseCollection {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSealed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetterBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISetterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, targetproperty: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISizeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PreviousSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousSize: usize,
    #[cfg(feature = "Foundation")]
    pub NewSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewSize: usize,
}
#[repr(C)]
pub struct ISizeHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISizeHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Empty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Empty: usize,
    #[cfg(feature = "Foundation")]
    pub FromDimensions: unsafe extern "system" fn(this: *mut *mut Self, width: f32, height: f32, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromDimensions: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsEmpty: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Size, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsEmpty: usize,
    #[cfg(feature = "Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: super::super::Foundation::Size, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Equals: usize,
}
#[repr(C)]
pub struct IStateTrigger {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStateTriggerBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IStateTriggerBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStateTriggerBaseProtected {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetActive: unsafe extern "system" fn(this: *mut *mut Self, isactive: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStateTriggerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsActiveProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStyle {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSealed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Setters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub TargetType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    TargetType: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub SetTargetType: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    SetTargetType: usize,
    pub BasedOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBasedOn: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Seal: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IStyleFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, targettype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct ITargetPropertyPath {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITargetPropertyPathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, targetproperty: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IThicknessHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IThicknessHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromLengths: unsafe extern "system" fn(this: *mut *mut Self, left: f64, top: f64, right: f64, bottom: f64, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
    pub FromUniformLength: unsafe extern "system" fn(this: *mut *mut Self, uniformlength: f64, result__: *mut Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITriggerAction {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITriggerActionFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITriggerBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITriggerBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUIElement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DesiredSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredSize: usize,
    pub AllowDrop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowDrop: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Clip: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetClip: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetClip: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub RenderTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    RenderTransform: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetRenderTransform: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetRenderTransform: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Projection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Projection: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetProjection: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetProjection: usize,
    #[cfg(feature = "Foundation")]
    pub RenderTransformOrigin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTransformOrigin: usize,
    #[cfg(feature = "Foundation")]
    pub SetRenderTransformOrigin: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRenderTransformOrigin: usize,
    pub IsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHitTestVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Visibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Visibility) -> ::windows_sys::core::HRESULT,
    pub SetVisibility: unsafe extern "system" fn(this: *mut *mut Self, value: Visibility) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RenderSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderSize: usize,
    pub UseLayoutRounding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseLayoutRounding: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub Transitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    Transitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetTransitions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetTransitions: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CacheMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CacheMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCacheMode: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCacheMode: usize,
    pub IsTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDoubleTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDoubleTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRightTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRightTapEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHoldingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHoldingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::ManipulationModes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetManipulationMode: unsafe extern "system" fn(this: *mut *mut Self, value: Input::ManipulationModes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetManipulationMode: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
    pub PointerCaptures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input")))]
    PointerCaptures: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub KeyUp: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    KeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyUp: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub KeyDown: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    KeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub DragEnter: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragEnter: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragEnter: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragEnter: usize,
    #[cfg(feature = "Foundation")]
    pub DragLeave: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragLeave: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragLeave: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragLeave: usize,
    #[cfg(feature = "Foundation")]
    pub DragOver: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragOver: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragOver: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragOver: usize,
    #[cfg(feature = "Foundation")]
    pub Drop: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Drop: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrop: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrop: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerCanceled: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerWheelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub Tapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    Tapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTapped: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub DoubleTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    DoubleTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDoubleTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDoubleTapped: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub Holding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    Holding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHolding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHolding: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub RightTapped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    RightTapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRightTapped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRightTapped: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ManipulationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ManipulationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarting: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ManipulationInertiaStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ManipulationInertiaStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationInertiaStarting: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ManipulationDelta: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ManipulationDelta: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationDelta: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationDelta: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Measure: unsafe extern "system" fn(this: *mut *mut Self, availablesize: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Measure: usize,
    #[cfg(feature = "Foundation")]
    pub Arrange: unsafe extern "system" fn(this: *mut *mut Self, finalrect: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Arrange: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub CapturePointer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    CapturePointer: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ReleasePointerCapture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ReleasePointerCapture: usize,
    pub ReleasePointerCaptures: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddHandler: unsafe extern "system" fn(this: *mut *mut Self, routedevent: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, handledeventstoo: bool) -> ::windows_sys::core::HRESULT,
    pub RemoveHandler: unsafe extern "system" fn(this: *mut *mut Self, routedevent: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub TransformToVisual: unsafe extern "system" fn(this: *mut *mut Self, visual: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    TransformToVisual: usize,
    pub InvalidateMeasure: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub InvalidateArrange: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UpdateLayout: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElement10 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub ActualOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ActualOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ActualSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ActualSize: usize,
    pub XamlRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Shadow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Shadow: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetShadow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetShadow: usize,
}
#[repr(C)]
pub struct IUIElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CompositeMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Media::ElementCompositeMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CompositeMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut *mut Self, value: Media::ElementCompositeMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCompositeMode: usize,
    pub CancelDirectManipulations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElement3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub Transform3D: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    Transform3D: usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub SetTransform3D: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))]
    SetTransform3D: usize,
    pub CanDrag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanDrag: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DragStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DragStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDragStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDragStarting: usize,
    #[cfg(feature = "Foundation")]
    pub DropCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDropCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDropCompleted: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "UI_Input"))]
    pub StartDragAsync: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "UI_Input")))]
    StartDragAsync: usize,
}
#[repr(C)]
pub struct IUIElement4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub ContextFlyout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    ContextFlyout: usize,
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub SetContextFlyout: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls_Primitives"))]
    SetContextFlyout: usize,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ContextRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ContextCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContextCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContextCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContextCanceled: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayDismissed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyInvoked: usize,
}
#[repr(C)]
pub struct IUIElement5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Lights: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Lights: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::KeyTipPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, value: Input::KeyTipPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusKeyboardNavigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::XYFocusKeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusKeyboardNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusKeyboardNavigation: unsafe extern "system" fn(this: *mut *mut Self, value: Input::XYFocusKeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusKeyboardNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub HighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ElementHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, value: ElementHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TabFocusNavigation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::KeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TabFocusNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetTabFocusNavigation: unsafe extern "system" fn(this: *mut *mut Self, value: Input::KeyboardNavigationMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetTabFocusNavigation: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub GettingFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    GettingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGettingFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGettingFocus: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub LosingFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    LosingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLosingFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLosingFocus: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub NoFocusCandidateFound: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    NoFocusCandidateFound: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNoFocusCandidateFound: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNoFocusCandidateFound: usize,
    pub StartBringIntoView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartBringIntoViewWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElement7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
    pub KeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input")))]
    KeyboardAccelerators: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub CharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    CharacterReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub ProcessKeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    ProcessKeyboardAccelerators: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessKeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessKeyboardAccelerators: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PreviewKeyDown: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PreviewKeyDown: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewKeyDown: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewKeyDown: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub PreviewKeyUp: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    PreviewKeyUp: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewKeyUp: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewKeyUp: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryInvokeKeyboardAccelerator: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryInvokeKeyboardAccelerator: usize,
}
#[repr(C)]
pub struct IUIElement8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyTipTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetKeyTipTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetKeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyboardAcceleratorPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Input::KeyboardAcceleratorPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyboardAcceleratorPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyboardAcceleratorPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyboardAcceleratorPlacementMode: usize,
    #[cfg(feature = "Foundation")]
    pub BringIntoViewRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BringIntoViewRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBringIntoViewRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBringIntoViewRequested: usize,
}
#[repr(C)]
pub struct IUIElement9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanBeScrollAnchor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanBeScrollAnchor: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OpacityTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOpacityTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTranslation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTranslation: usize,
    pub TranslationTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTranslationTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRotationTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    pub ScaleTransition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScaleTransition: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CenterPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenterPoint: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenterPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RotationAxis: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRotationAxis: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRotationAxis: usize,
    #[cfg(feature = "UI_Composition")]
    pub StartAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    StartAnimation: usize,
    #[cfg(feature = "UI_Composition")]
    pub StopAnimation: unsafe extern "system" fn(this: *mut *mut Self, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    StopAnimation: usize,
}
#[repr(C)]
pub struct IUIElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUIElementOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub OnCreateAutomationPeer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    OnCreateAutomationPeer: usize,
    pub OnDisconnectVisualChildren: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindSubElementsForTouchTargeting: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::Foundation::Point, boundingrect: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindSubElementsForTouchTargeting: usize,
}
#[repr(C)]
pub struct IUIElementOverrides7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetChildrenInTabFocusOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetChildrenInTabFocusOrder: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnProcessKeyboardAccelerators: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnProcessKeyboardAccelerators: usize,
}
#[repr(C)]
pub struct IUIElementOverrides8 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnKeyboardAcceleratorInvoked: unsafe extern "system" fn(this: *mut *mut Self, args: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnKeyboardAcceleratorInvoked: usize,
    pub OnBringIntoViewRequested: unsafe extern "system" fn(this: *mut *mut Self, e: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementOverrides9 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub PopulatePropertyInfoOverride: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::windows_sys::core::HSTRING, animationpropertyinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    PopulatePropertyInfoOverride: usize,
}
#[repr(C)]
pub struct IUIElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyDownEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyUpEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerEnteredEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerPressedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerMovedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerReleasedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerExitedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerCaptureLostEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerCanceledEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerWheelChangedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TappedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DoubleTappedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HoldingEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RightTappedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationStartingEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationInertiaStartingEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationStartedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationDeltaEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationCompletedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragEnterEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragLeaveEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DragOverEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DropEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllowDropProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpacityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ClipProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RenderTransformProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProjectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RenderTransformOriginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHitTestVisibleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VisibilityProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UseLayoutRoundingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransitionsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CacheModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTapEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDoubleTapEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRightTapEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsHoldingEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ManipulationModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointerCapturesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics10 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShadowProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompositeModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Transform3DProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanDragProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryStartDirectManipulation: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryStartDirectManipulation: usize,
}
#[repr(C)]
pub struct IUIElementStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContextFlyoutProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LightsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusKeyboardNavigationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HighContrastAdjustmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TabFocusNavigationProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GettingFocusEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LosingFocusEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NoFocusCandidateFoundEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PreviewKeyDownEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterReceivedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreviewKeyUpEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BringIntoViewRequestedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContextRequestedEvent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyboardAcceleratorPlacementTargetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyboardAcceleratorPlacementModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterAsScrollPort: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementStatics9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanBeScrollAnchorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUIElementWeakCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IUIElementWeakCollectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUnhandledExceptionEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exception: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVector3Transition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Vector3TransitionComponents) -> ::windows_sys::core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(this: *mut *mut Self, value: Vector3TransitionComponents) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVector3TransitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualState {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub Storyboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    Storyboard: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub SetStoryboard: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    SetStoryboard: usize,
}
#[repr(C)]
pub struct IVisualState2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Setters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StateTriggers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StateTriggers: usize,
}
#[repr(C)]
pub struct IVisualStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOldState: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNewState: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub Control: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    Control: usize,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub SetControl: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    SetControl: usize,
}
#[repr(C)]
pub struct IVisualStateGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Transitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Transitions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub States: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    States: usize,
    pub CurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentStateChanging: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentStateChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentStateChanging: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentStateChanging: usize,
}
#[repr(C)]
pub struct IVisualStateManager {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVisualStateManagerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVisualStateManagerOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub GoToStateCore: unsafe extern "system" fn(this: *mut *mut Self, control: *mut ::core::ffi::c_void, templateroot: *mut ::core::ffi::c_void, statename: ::windows_sys::core::HSTRING, group: *mut ::core::ffi::c_void, state: *mut ::core::ffi::c_void, usetransitions: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    GoToStateCore: usize,
}
#[repr(C)]
pub struct IVisualStateManagerProtected {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub RaiseCurrentStateChanging: unsafe extern "system" fn(this: *mut *mut Self, stategroup: *mut ::core::ffi::c_void, oldstate: *mut ::core::ffi::c_void, newstate: *mut ::core::ffi::c_void, control: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    RaiseCurrentStateChanging: usize,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub RaiseCurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, stategroup: *mut ::core::ffi::c_void, oldstate: *mut ::core::ffi::c_void, newstate: *mut ::core::ffi::c_void, control: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    RaiseCurrentStateChanged: usize,
}
#[repr(C)]
pub struct IVisualStateManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetVisualStateGroups: unsafe extern "system" fn(this: *mut *mut Self, obj: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetVisualStateGroups: usize,
    pub CustomVisualStateManagerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCustomVisualStateManager: unsafe extern "system" fn(this: *mut *mut Self, obj: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCustomVisualStateManager: unsafe extern "system" fn(this: *mut *mut Self, obj: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub GoToState: unsafe extern "system" fn(this: *mut *mut Self, control: *mut ::core::ffi::c_void, statename: ::windows_sys::core::HSTRING, usetransitions: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    GoToState: usize,
}
#[repr(C)]
pub struct IVisualTransition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GeneratedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GeneratedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetGeneratedDuration: unsafe extern "system" fn(this: *mut *mut Self, value: Duration) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetGeneratedDuration: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub GeneratedEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    GeneratedEasingFunction: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub SetGeneratedEasingFunction: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    SetGeneratedEasingFunction: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub Storyboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    Storyboard: usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub SetStoryboard: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))]
    SetStoryboard: usize,
}
#[repr(C)]
pub struct IVisualTransitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindow {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CoreWindow: usize,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub SizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    VisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibilityChanged: usize,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindow2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTitleBar: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindow3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
}
#[repr(C)]
pub struct IWindow4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Window: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlRoot {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    pub RasterizationScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub IsHostVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[repr(C)]
pub struct IXamlRootChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
pub type LeavingBackgroundEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct LineStackingStrategy(pub i32);
impl LineStackingStrategy {
    pub const MaxHeight: Self = Self(0i32);
    pub const BlockLineHeight: Self = Self(1i32);
    pub const BaselineToBaseline: Self = Self(2i32);
}
impl ::core::marker::Copy for LineStackingStrategy {}
impl ::core::clone::Clone for LineStackingStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaFailedRoutedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct OpticalMarginAlignment(pub i32);
impl OpticalMarginAlignment {
    pub const None: Self = Self(0i32);
    pub const TrimSideBearings: Self = Self(1i32);
}
impl ::core::marker::Copy for OpticalMarginAlignment {}
impl ::core::clone::Clone for OpticalMarginAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PointHelper = *mut ::core::ffi::c_void;
pub type PropertyChangedCallback = *mut ::core::ffi::c_void;
pub type PropertyMetadata = *mut ::core::ffi::c_void;
pub type PropertyPath = *mut ::core::ffi::c_void;
pub type RectHelper = *mut ::core::ffi::c_void;
pub type ResourceDictionary = *mut ::core::ffi::c_void;
pub type RoutedEvent = *mut ::core::ffi::c_void;
pub type RoutedEventArgs = *mut ::core::ffi::c_void;
pub type RoutedEventHandler = *mut ::core::ffi::c_void;
pub type ScalarTransition = *mut ::core::ffi::c_void;
pub type Setter = *mut ::core::ffi::c_void;
pub type SetterBase = *mut ::core::ffi::c_void;
pub type SetterBaseCollection = *mut ::core::ffi::c_void;
pub type SizeChangedEventArgs = *mut ::core::ffi::c_void;
pub type SizeChangedEventHandler = *mut ::core::ffi::c_void;
pub type SizeHelper = *mut ::core::ffi::c_void;
pub type StateTrigger = *mut ::core::ffi::c_void;
pub type StateTriggerBase = *mut ::core::ffi::c_void;
pub type Style = *mut ::core::ffi::c_void;
pub type SuspendingEventHandler = *mut ::core::ffi::c_void;
pub type TargetPropertyPath = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct TextAlignment(pub i32);
impl TextAlignment {
    pub const Center: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Start: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const End: Self = Self(2i32);
    pub const Justify: Self = Self(3i32);
    pub const DetectFromContent: Self = Self(4i32);
}
impl ::core::marker::Copy for TextAlignment {}
impl ::core::clone::Clone for TextAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct TextLineBounds(pub i32);
impl TextLineBounds {
    pub const Full: Self = Self(0i32);
    pub const TrimToCapHeight: Self = Self(1i32);
    pub const TrimToBaseline: Self = Self(2i32);
    pub const Tight: Self = Self(3i32);
}
impl ::core::marker::Copy for TextLineBounds {}
impl ::core::clone::Clone for TextLineBounds {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct TextReadingOrder(pub i32);
impl TextReadingOrder {
    pub const Default: Self = Self(0i32);
    pub const UseFlowDirection: Self = Self(0i32);
    pub const DetectFromContent: Self = Self(1i32);
}
impl ::core::marker::Copy for TextReadingOrder {}
impl ::core::clone::Clone for TextReadingOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct TextTrimming(pub i32);
impl TextTrimming {
    pub const None: Self = Self(0i32);
    pub const CharacterEllipsis: Self = Self(1i32);
    pub const WordEllipsis: Self = Self(2i32);
    pub const Clip: Self = Self(3i32);
}
impl ::core::marker::Copy for TextTrimming {}
impl ::core::clone::Clone for TextTrimming {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1i32);
    pub const Wrap: Self = Self(2i32);
    pub const WrapWholeWords: Self = Self(3i32);
}
impl ::core::marker::Copy for TextWrapping {}
impl ::core::clone::Clone for TextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml\"`*"]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl ::core::marker::Copy for Thickness {}
impl ::core::clone::Clone for Thickness {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ThicknessHelper = *mut ::core::ffi::c_void;
pub type TriggerAction = *mut ::core::ffi::c_void;
pub type TriggerActionCollection = *mut ::core::ffi::c_void;
pub type TriggerBase = *mut ::core::ffi::c_void;
pub type TriggerCollection = *mut ::core::ffi::c_void;
pub type UIElement = *mut ::core::ffi::c_void;
pub type UIElementWeakCollection = *mut ::core::ffi::c_void;
pub type UnhandledExceptionEventArgs = *mut ::core::ffi::c_void;
pub type UnhandledExceptionEventHandler = *mut ::core::ffi::c_void;
pub type Vector3Transition = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct Vector3TransitionComponents(pub u32);
impl Vector3TransitionComponents {
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for Vector3TransitionComponents {}
impl ::core::clone::Clone for Vector3TransitionComponents {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for VerticalAlignment {}
impl ::core::clone::Clone for VerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Xaml\"`*"]
#[repr(transparent)]
pub struct Visibility(pub i32);
impl Visibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
}
impl ::core::marker::Copy for Visibility {}
impl ::core::clone::Clone for Visibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VisualState = *mut ::core::ffi::c_void;
pub type VisualStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type VisualStateChangedEventHandler = *mut ::core::ffi::c_void;
pub type VisualStateGroup = *mut ::core::ffi::c_void;
pub type VisualStateManager = *mut ::core::ffi::c_void;
pub type VisualTransition = *mut ::core::ffi::c_void;
pub type Window = *mut ::core::ffi::c_void;
pub type WindowActivatedEventHandler = *mut ::core::ffi::c_void;
pub type WindowClosedEventHandler = *mut ::core::ffi::c_void;
pub type WindowCreatedEventArgs = *mut ::core::ffi::c_void;
pub type WindowSizeChangedEventHandler = *mut ::core::ffi::c_void;
pub type WindowVisibilityChangedEventHandler = *mut ::core::ffi::c_void;
pub type XamlRoot = *mut ::core::ffi::c_void;
pub type XamlRootChangedEventArgs = *mut ::core::ffi::c_void;
